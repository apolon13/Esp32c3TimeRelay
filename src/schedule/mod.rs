use chrono::{NaiveDateTime, Timelike};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

#[derive(Eq, Hash, PartialEq)]
pub struct Job {
    run_at_hour: u32,
    cb: fn(),
}

impl Job {
    pub fn new(run_at_hour: u32, cb: fn()) -> Self {
        Self { run_at_hour, cb }
    }

    pub fn run(&self, current_hour: u32) -> bool {
        if self.run_at_hour == current_hour {
            (self.cb)();
            return true;
        }
        false
    }
}

pub struct Scheduler {
    jobs: Vec<Job>,
    current_time: NaiveDateTime,
}

impl Scheduler {
    pub fn new(current_time: NaiveDateTime, jobs: Vec<Job>) -> Self {
        Self { jobs, current_time }
    }

    pub fn run(&self) {
        let mut happened_map: HashMap<&Job, bool> = HashMap::new();
        let mut current_time = self.current_time;
        loop {
            sleep(Duration::from_secs(1));
            current_time = current_time + Duration::from_secs(1);
            for job in self.jobs.iter() {
                match happened_map.get(job) {
                    Some(happened) => {
                        if !*happened {
                            happened_map.insert(job, job.run(current_time.hour()));
                        }
                    }
                    None => {
                        happened_map.insert(job, job.run(current_time.hour()));
                    }
                }
            }
        }
    }
}
