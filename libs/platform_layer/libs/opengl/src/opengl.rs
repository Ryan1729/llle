// This was originally based on example code for https://github.com/alexheretic/glyph-brush
// the code is licensed under the Apache 2.0 license, as described in the license file in the
// opengl folder, to the extent that the code remains as it was
// (at commit 90e7c7c331e9f991e11de6404b2ca073c0a09e61)
use glutin::dpi::LogicalPosition;
use glutin::{Api, ContextTrait, GlProfile, GlRequest};
use glyph_brush::{rusttype::Font, *};
use macros::d;

use platform_types::{BufferView, CharDim, Input, ScreenSpaceXY, Sizes, UpdateAndRender};

#[perf_viz::record]
pub fn run(update_and_render: UpdateAndRender) -> gl_layer::Res<()> {
    run_inner(update_and_render)
}

// This extra fn is a workaround for the record attribute causing a "procedural macros cannot
// expand to macro definitions" error otherwise.According to issue #54727, this is because there
// is some worry that all the macro hygiene edge cases may not be handled.
fn run_inner(update_and_render: UpdateAndRender) -> gl_layer::Res<()> {
    if cfg!(target_os = "linux") {
        use std::env;
        // winit wayland is currently still wip
        if env::var("WINIT_UNIX_BACKEND").is_err() {
            env::set_var("WINIT_UNIX_BACKEND", "x11");
        }
        // disables vsync sometimes on x11
        if env::var("vblank_mode").is_err() {
            env::set_var("vblank_mode", "0");
        }
    }

    let mut events = glutin::EventsLoop::new();
    let title = "rote";

    let window = glutin::WindowedContext::new_windowed(
        glutin::WindowBuilder::new()
            .with_dimensions((1024, 576).into())
            .with_title(title),
        glutin::ContextBuilder::new()
            .with_gl_profile(GlProfile::Core)
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 2)))
            .with_srgb(true),
        &events,
    )?;
    unsafe { window.make_current()? };

    let font_bytes: &[u8] = include_bytes!("./fonts/FiraCode-Retina-plus-CR-and-LF.ttf");
    let font: Font<'static> = Font::from_bytes(font_bytes)?;
    let text_size: f32 = 600.0;
    let status_size: f32 = 22.0;
    let scroll_multiplier: f32 = 16.0;

    let text_scale =
        rusttype::Scale::uniform((text_size * window.get_hidpi_factor() as f32).round());
    let status_scale =
        rusttype::Scale::uniform((status_size * window.get_hidpi_factor() as f32).round());

    let mut glyph_brush = GlyphBrushBuilder::using_font(font.clone())
        // Leaving this at the default of 0.1 makes the cache get cleared too often.
        // Putting this at 1.0 means that the characters are visibly poorly kerned.
        // This value seems like a happy medium at the moment.
        .gpu_cache_position_tolerance(0.25)
        .build();

    let mut gl_state = gl_layer::init(&glyph_brush, |symbol| window.get_proc_address(symbol) as _)?;

    let mut loop_helper = spin_sleep::LoopHelper::builder().build_with_target_rate(250.0);
    let mut running = true;
    let mut dimensions = window
        .get_inner_size()
        .ok_or("get_inner_size = None")?
        .to_physical(window.get_hidpi_factor());

    macro_rules! get_char_dim {
        ($scale:ident) => {
            CharDim {
                w: {
                    // We currently assume the font is monospaced.
                    let em_space_char = '\u{2003}';
                    let h_metrics = font.glyph(em_space_char).scaled($scale).h_metrics();

                    h_metrics.advance_width
                },
                h: {
                    let v_metrics = font.v_metrics($scale);

                    v_metrics.ascent + -v_metrics.descent + v_metrics.line_gap
                },
            }
        };
    }

    let text_char_dim = get_char_dim!(text_scale);
    let status_char_dim = get_char_dim!(status_scale);

    let (mut view, mut _cmd) = update_and_render(Input::SetSizes(Sizes! {
        screen_w: dimensions.width as f32,
        screen_h: dimensions.height as f32,
        text_char_dim: text_char_dim,
        status_char_dim: status_char_dim,
    }));

    let (mut mouse_x, mut mouse_y) = (0.0, 0.0);

    use std::sync::mpsc::channel;

    // into the editor thread
    let (in_tx, in_rx) = channel();
    // out of the editor thread
    let (out_tx, out_rx) = channel();

    let join_handle = std::thread::Builder::new()
        .name("editor".to_string())
        .spawn(move || {
            while let Ok(input) = in_rx.recv() {
                let pair = update_and_render(input);
                let _hope_it_gets_there = out_tx.send(pair);
                if let Input::Quit = input {
                    return;
                }
            }
        })
        .expect("Could not start editor thread!");

    while running {
        loop_helper.loop_start();

        perf_viz::start_record!("while running");
        events.poll_events(|event| {
            perf_viz::record_guard!("events.poll_events");
            use glutin::*;
            if let Event::WindowEvent { event, .. } = event {
                macro_rules! call_u_and_r {
                    ($input:expr) => {
                        let _hope_it_gets_there = in_tx.send($input);
                    };
                }

                use platform_types::Move;
                match event {
                    WindowEvent::CloseRequested => running = false,
                    WindowEvent::Resized(size) => {
                        let dpi = window.get_hidpi_factor();
                        window.resize(size.to_physical(dpi));
                        if let Some(ls) = window.get_inner_size() {
                            dimensions = ls.to_physical(dpi);
                            call_u_and_r!(Input::SetSizes(Sizes! {
                                screen_w: dimensions.width as f32,
                                screen_h: dimensions.height as f32,
                                text_char_dim: None,
                                status_char_dim: None,
                            }));
                            gl_layer::set_dimensions(dimensions.width as _, dimensions.height as _);

                            //if we don't reset the cache like this then we render a stretched
                            //version of the text on windoe resize.
                            let (t_w, t_h) = glyph_brush.texture_dimensions();
                            glyph_brush.resize_texture(t_w, t_h);
                        }
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(keypress),
                                modifiers: ModifiersState { ctrl: true, .. },
                                ..
                            },
                        ..
                    } => match keypress {
                        VirtualKeyCode::Key0 => {
                            call_u_and_r!(Input::ResetScroll);
                        }
                        VirtualKeyCode::Home => {
                            call_u_and_r!(Input::MoveAllCursors(Move::ToBufferStart));
                        }
                        VirtualKeyCode::End => {
                            call_u_and_r!(Input::MoveAllCursors(Move::ToBufferEnd));
                        }
                        _ => (),
                    },
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(keypress),
                                modifiers: ModifiersState { ctrl: false, .. },
                                ..
                            },
                        ..
                    } => match keypress {
                        VirtualKeyCode::Escape => {
                            call_u_and_r!(Input::Quit);
                            running = false;
                        }
                        VirtualKeyCode::Back => {
                            call_u_and_r!(Input::Delete);
                        }
                        VirtualKeyCode::Up => {
                            call_u_and_r!(Input::MoveAllCursors(Move::Up));
                        }
                        VirtualKeyCode::Down => {
                            call_u_and_r!(Input::MoveAllCursors(Move::Down));
                        }
                        VirtualKeyCode::Left => {
                            call_u_and_r!(Input::MoveAllCursors(Move::Left));
                        }
                        VirtualKeyCode::Right => {
                            call_u_and_r!(Input::MoveAllCursors(Move::Right));
                        }
                        VirtualKeyCode::Home => {
                            call_u_and_r!(Input::MoveAllCursors(Move::ToLineStart));
                        }
                        VirtualKeyCode::End => {
                            call_u_and_r!(Input::MoveAllCursors(Move::ToLineEnd));
                        }
                        _ => (),
                    },
                    WindowEvent::ReceivedCharacter(mut c) => {
                        if c != '\u{7f}' && c != '\u{8}' {
                            if c == '\r' {
                                c = '\n';
                            }
                            call_u_and_r!(Input::Insert(c));
                        }
                    }
                    WindowEvent::MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, y),
                        modifiers: ModifiersState { shift: false, .. },
                        ..
                    } => {
                        call_u_and_r!(Input::ScrollVertically(-y * scroll_multiplier));
                    }
                    WindowEvent::MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, y),
                        modifiers: ModifiersState { shift: true, .. },
                        ..
                    } => {
                        call_u_and_r!(Input::ScrollHorizontally(y * scroll_multiplier));
                    }
                    WindowEvent::CursorMoved {
                        position: LogicalPosition { x, y },
                        ..
                    } => {
                        mouse_x = x as f32;
                        mouse_y = y as f32;
                    }
                    WindowEvent::MouseInput {
                        button: MouseButton::Left,
                        ..
                    } => {
                        call_u_and_r!(Input::ReplaceCursors(ScreenSpaceXY {
                            x: mouse_x,
                            y: mouse_y
                        }));
                    }
                    _ => {}
                }
            }
        });

        if running {
            match out_rx.try_recv() {
                Ok((v, c)) => {
                    view = v;
                    _cmd = c;
                }
                _ => {}
            };
        }

        let mut status_line_position = None;
        let mut highlight_ranges = Vec::new();
        perf_viz::start_record!("for &BufferView");
        for &BufferView {
            kind,
            bounds,
            color,
            ref chars,
            screen_position,
            ref highlights,
        } in view.buffers.iter()
        {
            use platform_types::BufferViewKind;

            // Without a background the edit buffer(s) show through the status line(s)
            if let BufferViewKind::StatusLine = kind {
                status_line_position = Some(screen_position);
            }

            perf_viz::record_guard!("glyph_brush.queue");
            glyph_brush.queue(Section {
                text: &chars
                    .chars()
                    .map(|c| {
                        // map unprinatbles to symbols for themselves
                        if c < 0x20 as char {
                            std::char::from_u32(c as u32 | 0x2400u32).unwrap_or(c)
                        } else {
                            c
                        }
                    })
                    .collect::<String>(),
                scale: if let BufferViewKind::StatusLine = kind {
                    status_scale
                } else {
                    text_scale
                },
                screen_position,
                bounds,
                color,
                layout: Layout::default_single_line(),
                z: match kind {
                    BufferViewKind::Edit => gl_layer::EDIT_Z,
                    BufferViewKind::Cursor => gl_layer::CURSOR_Z,
                    BufferViewKind::StatusLine => gl_layer::STATUS_Z,
                },
                ..Section::default()
            });

            let mut rect_bounds: Bounds = d!();
            rect_bounds.max = bounds.into();

            highlight_ranges.extend(highlights.iter().map(|h| {
                let (min, max) = h.get();

                let mut pixel_coords: PixelCoords = d!();
                pixel_coords.min.x =
                    (min.offset.0 as f32 * text_char_dim.w + screen_position.0) as i32;

                pixel_coords.max.x =
                    (max.offset.0 as f32 * text_char_dim.w + screen_position.0) as i32;
                pixel_coords.max.y = (text_char_dim.h + screen_position.1) as i32;

                if_changed::dbg!(HighlightRange {
                    pixel_coords,
                    bounds: rect_bounds,
                    color: [0.9, 0.9, 0.9, 0.3],
                    z: gl_layer::HIGHLIGHT_Z,
                })
            }));
        }
        perf_viz::end_record!("for &BufferView");

        let width = dimensions.width as u32;
        let height = dimensions.height as f32;

        if_changed::println!("{:?}", status_line_position);

        gl_layer::render(
            &mut gl_state,
            &mut glyph_brush,
            width as _,
            height as _,
            status_line_position,
            highlight_ranges,
        )?;

        window.swap_buffers()?;

        if let Some(rate) = loop_helper.report_rate() {
            window.set_title(&format!(
                "{} {:.0} FPS {:?}",
                title,
                rate,
                (mouse_x, mouse_y)
            ));
        }

        perf_viz::end_record!("while running");
        loop_helper.loop_sleep();
    }

    // If we got here, we assume that we've sent a Quit input to the editor thread so it will stop.
    join_handle.join().expect("Could not join editor tread!");

    perf_viz::output!();

    gl_layer::cleanup(gl_state)
}
