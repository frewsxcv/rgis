use std::time;

#[cfg(target_arch = "wasm32")]
type Instant = ();
#[cfg(not(target_arch = "wasm32"))]
type Instant = time::Instant;

pub struct TimeLogger {
    title: String,
    duration: Instant,
}

#[cfg(target_arch = "wasm32")]
fn new_instant() -> Instant {
    ()
}
#[cfg(not(target_arch = "wasm32"))]
fn new_instant() -> Instant {
    time::Instant::now()
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
        #[cfg(not(target_arch = "wasm32"))]
        {
            bevy_log::info!("{}: done ({:?})", self.title, self.duration.elapsed());
        }
    }
}

pub fn start(title: impl Into<String>) -> TimeLogger {
    TimeLogger::start(title)
}
