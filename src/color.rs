use std::sync;

const COLORS: [colorous::Color; 10] = colorous::CATEGORY10;

pub fn next() -> pathfinder_canvas::ColorU {
    colorous_color_to_pathfinder_color(next_colorous_color())
}

fn next_colorous_color() -> colorous::Color {
    COLORS[next_color_index()]
}

fn next_color_index() -> usize {
    static COUNTER: sync::atomic::AtomicUsize = sync::atomic::AtomicUsize::new(0);
    COUNTER.fetch_add(1, sync::atomic::Ordering::AcqRel) % COLORS.len()
}

fn colorous_color_to_pathfinder_color(c: colorous::Color) -> pathfinder_canvas::ColorU {
    pathfinder_canvas::ColorU::new(c.r, c.g, c.b, 255)
}
