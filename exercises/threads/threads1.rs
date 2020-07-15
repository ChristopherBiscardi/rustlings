// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

// I AM NOT DONE

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: AtomicU32,
}

fn main() {
    let status = Arc::new(JobStatus {
        jobs_completed: AtomicU32::new(0),
    });
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            status_shared.jobs_completed.fetch_add(1, Ordering::Relaxed);
        }
    });
    while status.jobs_completed.load(Ordering::Relaxed) < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
