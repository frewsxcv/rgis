use std::sync;

const COLORS: [colorous::Color; 10] = colorous::CATEGORY10;

pub fn next() -> rgx::color::Rgba8 {
    colorous_color_to_rgx_color(next_colorous_color())
}

fn next_colorous_color() -> colorous::Color {
    COLORS[next_color_index()]
}

fn next_color_index() -> usize {
    static COUNTER: sync::atomic::AtomicUsize = sync::atomic::AtomicUsize::new(0);
    COUNTER.fetch_add(1, sync::atomic::Ordering::Relaxed) % COLORS.len()
}

fn colorous_color_to_rgx_color(c: colorous::Color) -> rgx::color::Rgba8 {
    rgx::color::Rgba8 { r: c.r, g: c.g, b: c.b, a: 255 }
}
