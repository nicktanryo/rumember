use std::time::Duration;

use clap::Parser;
use job_scheduler::{Job, JobScheduler, Schedule};

use rumember::args::args::Args;
use rumember::backup::backup::create_backup;

fn main() {
    env_logger::init();
    let args = Args::parse();

    let job_schedule: Schedule = args.schedule.parse().unwrap();
    let backup_job = || create_backup(&args);

    let mut scheduler = JobScheduler::new();
    scheduler.add(Job::new(job_schedule, backup_job));

    loop {
        scheduler.tick();
        std::thread::sleep(Duration::from_secs(1));
    }
}
