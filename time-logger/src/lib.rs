pub struct TimeLogger {
    title: String,
    duration: web_time::Instant,
}

impl TimeLogger {
    #[inline]
    pub fn start(title: impl Into<String>) -> Self {
        let title: String = title.into();
        bevy_log::info!("{}: started", title);

        TimeLogger {
            title,
            duration: web_time::Instant::now(),
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
