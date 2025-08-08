# bevy_job_fetch

A plugin for Bevy that provides a generic `Job` for fetching data from a URL.

This crate is a generic wrapper around `reqwest` and `bevy_jobs`. It allows you to fetch data from a URL and pass along arbitrary user data.

## Usage

First, add `bevy_job_fetch` to your `Cargo.toml`:

```toml
[dependencies]
bevy_job_fetch = { git = "https://github.com/your-repo/bevy_job_fetch" } # Or from crates.io
```

Then, add the `bevy_job_fetch::Plugin` to your Bevy app:

```rust
use bevy::prelude::*;
use bevy_job_fetch::Plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_jobs::JobSchedulerPlugin::default())
        .add_plugins(Plugin)
        .add_systems(Startup, setup)
        .run();
}
```

Now you can spawn `NetworkFetchJob`s. The job is generic over a `UserData` type, which can be any type that implements `Clone + Send + Sync + 'static`.

Here is an example of how to fetch a file and handle the result:

```rust
use bevy::prelude::*;
use bevy_jobs::{Job, JobScheduler, JobStatus};
use bevy_job_fetch::{FetchedFile, NetworkFetchJob};

#[derive(Clone, Send, Sync, Debug)]
struct MyUserData {
    id: u32,
}

fn setup(mut commands: Commands, mut job_scheduler: ResMut<JobScheduler>) {
    let user_data = MyUserData { id: 42 };

    let job = NetworkFetchJob {
        url: "https://www.rust-lang.org/static/images/rust-logo-blk.svg".to_string(),
        user_data,
        name: "Rust Logo".to_string(),
    };

    let job_id = job_scheduler.start(job);

    commands.spawn_empty().insert(job_id);
}

fn handle_completed_jobs(
    mut commands: Commands,
    mut completed_jobs: Query<(Entity, &JobStatus), Changed<JobStatus>>,
    mut job_scheduler: ResMut<JobScheduler>,
) {
    for (entity, job_status) in completed_jobs.iter_mut() {
        if let JobStatus::Completed(result) = job_status {
            let result: Result<FetchedFile<MyUserData>, _> = job_scheduler.result(result.id()).unwrap();
            match result {
                Ok(fetched_file) => {
                    println!(
                        "Fetched file '{}' ({} bytes) with user data: {:?}",
                        fetched_file.name,
                        fetched_file.bytes.len(),
                        fetched_file.user_data
                    );
                }
                Err(e) => {
                    eprintln!("Error fetching file: {}", e);
                }
            }
            commands.entity(entity).despawn();
        }
    }
}
```
