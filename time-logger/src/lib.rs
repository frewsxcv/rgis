use std::time;

pub struct TimeLogger {
    title: String,
    duration: time::Instant,
}

impl TimeLogger {
    pub fn start(title: impl Into<String>) -> Self {
        let title: String = title.into();
        log::info!("{}: started", title);
        TimeLogger {
            title: title,
            duration: time::Instant::now(),
        }
    }

    pub fn finish(self) {
        log::info!("{}: done ({:?})", self.title, self.duration.elapsed());
    }
}

pub fn start(title: impl Into<String>) -> TimeLogger {
    TimeLogger::start(title)
}
