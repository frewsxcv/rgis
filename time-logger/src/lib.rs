#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

pub struct TimeLogger {
    title: String,
    duration: instant::Instant,
}

impl TimeLogger {
    #[inline]
    pub fn start(title: impl Into<String>) -> Self {
        let title: String = title.into();
        bevy_log::info!("{}: started", title);

        TimeLogger {
            title,
            duration: instant::Instant::now(),
        }
    }

    #[inline]
    pub fn finish(self) {
        bevy_log::info!("{}: done ({:?})", self.title, self.duration.elapsed());
    }
}

#[macro_export]
macro_rules! start {
    ($($args:tt)*) => {{
        time_logger::TimeLogger::start(format!($($args)*))
    }};
}
