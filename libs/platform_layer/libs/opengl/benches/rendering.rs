use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use glutin::{Api, ContextTrait, GlProfile, GlRequest};
use glyph_brush::GlyphBrush;

use editor::{update_and_render, State};
use gl_layer::{RenderExtras, Vertex};
use opengl::{get_glyph_brush, render_buffer_view, FontInfo};
use platform_types::{Input, Move};

fn slipsum_buffer() -> State {
    include_str!("../../../../../text/slipsum.txt").into()
}

fn render_buffer_view_benchmark(c: &mut Criterion) {
    c.bench_function("render_buffer full highlight", |b| {
        b.iter_batched(
            || {
                let (full_highlight_view, _) = update_and_render(
                    &mut slipsum_buffer(),
                    Input::ExtendSelectionForAllCursors(Move::ToBufferEnd),
                );;
                let font_info = FontInfo::new(2.0).unwrap();
                let glyph_brush: GlyphBrush<Vertex> = get_glyph_brush(&font_info);
                (glyph_brush, full_highlight_view, font_info)
            },
            |(mut glyph_brush, view, font_info)| {
                render_buffer_view(&mut black_box(glyph_brush), &view, &font_info)
            },
            BatchSize::LargeInput,
        )
    });
}

fn gl_layer_benchmark(c: &mut Criterion) {
    const SCREEN_SIZE: u32 = 2048;

    fn render_frame(
        (gl_state, glyph_brush, window, extras): (
            &mut gl_layer::State,
            &mut GlyphBrush<Vertex>,
            &mut glutin::WindowedContext,
            RenderExtras,
        ),
    ) {
        gl_layer::render(
            black_box(gl_state),
            black_box(glyph_brush),
            2048 as _,
            2048 as _,
            extras.clone(),
        )
        .unwrap();
        window.swap_buffers().unwrap();
    };

    c.bench_function("gl_layer full highlight", |b| {
        b.iter_batched(
            || {
                let mut buffer = slipsum_buffer();
                let (full_highlight_view, _) = update_and_render(
                    &mut buffer,
                    Input::ExtendSelectionForAllCursors(Move::ToBufferEnd),
                );
                let font_info = FontInfo::new(2.0).unwrap();

                let mut window = glutin::WindowedContext::new_windowed(
                    glutin::WindowBuilder::new()
                        .with_dimensions((SCREEN_SIZE, SCREEN_SIZE).into())
                        .with_title(SCREEN_SIZE.to_string()),
                    glutin::ContextBuilder::new()
                        .with_gl_profile(GlProfile::Core)
                        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 2)))
                        .with_srgb(true),
                    &glutin::EventsLoop::new(),
                )
                .unwrap();
                unsafe { window.make_current() }.unwrap();
                let mut glyph_brush: GlyphBrush<Vertex> = get_glyph_brush(&font_info);

                let mut gl_state =
                    gl_layer::init(&glyph_brush, |symbol| window.get_proc_address(symbol) as _)
                        .unwrap();

                {
                    let extras =
                        render_buffer_view(&mut glyph_brush, &full_highlight_view, &font_info);

                    // fill the glyph cache
                    render_frame((&mut gl_state, &mut glyph_brush, &mut window, extras))
                }

                {
                    let (no_highlight_view, _) =
                        update_and_render(&mut buffer, Input::MoveAllCursors(Move::ToBufferStart));
                    let extras =
                        render_buffer_view(&mut glyph_brush, &no_highlight_view, &font_info);

                    // remove highlight frame from cache
                    render_frame((&mut gl_state, &mut glyph_brush, &mut window, extras));
                }

                let extras = render_buffer_view(&mut glyph_brush, &full_highlight_view, &font_info);
                (gl_state, glyph_brush, window, extras)
            },
            |(mut gl_state, mut glyph_brush, mut window, extras)| {
                render_frame((&mut gl_state, &mut glyph_brush, &mut window, extras))
            },
            BatchSize::LargeInput,
        )
    });
}

criterion_group!(render_buffer_view_group, render_buffer_view_benchmark);
criterion_group!(gl_layer_group, gl_layer_benchmark);
criterion_main!(render_buffer_view_group, gl_layer_group);
