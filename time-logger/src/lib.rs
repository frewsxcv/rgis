pub struct TimeLogger {
    title: String,
    duration: instant::Instant,
}

fn new_instant() -> instant::Instant {
    instant::Instant::now()
}

impl TimeLogger {
    pub fn start(title: impl Into<String>) -> Self {
        let title: String = title.into();
        bevy_log::info!("{}: started", title);

        TimeLogger {
            title,
            duration: new_instant(),
        }
    }

    pub fn finish(self) {
        bevy_log::info!("{}: done ({:?})", self.title, self.duration.elapsed());
    }
}

pub fn start(title: impl Into<String>) -> TimeLogger {
    TimeLogger::start(title)
}
