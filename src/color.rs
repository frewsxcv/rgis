use std::sync;

const COLORS: [colorous::Color; 10] = colorous::CATEGORY10;

lazy_static::lazy_static! {
    static ref COLOR_INDEX: sync::atomic::AtomicUsize = sync::atomic::AtomicUsize::new(0);
}

pub fn next() -> pathfinder_canvas::ColorU {
    let index = COLOR_INDEX.fetch_add(1, sync::atomic::Ordering::AcqRel) % COLORS.len();
    colorous_color_to_pathfinder_color(COLORS[index])
}

fn colorous_color_to_pathfinder_color(c: colorous::Color) -> pathfinder_canvas::ColorU {
    pathfinder_canvas::ColorU::new(c.r, c.g, c.b, 255)
}
