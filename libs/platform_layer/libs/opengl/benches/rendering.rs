use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use glyph_brush::GlyphBrush;

use editor::{update_and_render, State};
use opengl::{get_glyph_brush, render_buffer_view, FontInfo};
use platform_types::{Input, Move};

fn slipsum_buffer() -> State {
    include_str!("../../../../../text/slipsum.txt").into()
}

fn render_buffer_view_benchmark(c: &mut Criterion) {
    c.bench_function("full highlight", |b| {
        b.iter_batched(
            || {
                let (full_highlight_view, _) = update_and_render(
                    &mut slipsum_buffer(),
                    Input::ExtendSelectionForAllCursors(Move::ToBufferEnd),
                );;
                let font_info = FontInfo::new(2.0).unwrap();
                let glyph_brush: GlyphBrush<()> = get_glyph_brush(&font_info);
                (glyph_brush, full_highlight_view, font_info)
            },
            |(mut glyph_brush, full_highlight_view, font_info)| {
                render_buffer_view(
                    &mut black_box(glyph_brush),
                    &full_highlight_view,
                    &font_info,
                )
            },
            BatchSize::LargeInput,
        )
    });
}

criterion_group!(render_buffer_view_group, render_buffer_view_benchmark);
criterion_main!(render_buffer_view_group);
