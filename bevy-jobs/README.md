# bevy-jobs

A lightweight job framework for Bevy.

## Getting started

Defining a job:

```rust
pub struct FetchRequestJob {
    pub url: String,
}

impl bevy_jobs::Job for FetchRequestJob {
    type Outcome = Result<Vec<u8>, Error>;

    fn name(&self) -> String {
        format!("Fetching request: '{}'", url);
    }

    fn perform(self) -> bevy_jobs::AsyncReturn<Self::Outcome> {
        Box::pin(async move {
            fetch(&self.url).await
        })
    }
}
```

Spawning a job from a system:

```rust
fn some_spawn_system(
    mut job_spawner: bevy_jobs::JobSpawner,
) {
    job_spawner.spawn(FetchRequestJob {
        url: "https://example.com/".into(),
    });
}
```

Fetching job results from a system:

```rust
fn some_result_system(
    mut finished_tasks: bevy_jobs::FinishedJobs,
) {
    while let Some(result) = finished_tasks.take_next::<FetchRequestJob>() {
        // ...
    }
}
```

Querying in-progress jobs:

```rust
fn render_in_progress(
    query: Query<&bevy_jobs::InProgressJob>,
) {
    for job in query.iter() {
        println!("Job '{}' is running", job.name);
    }
}
```
