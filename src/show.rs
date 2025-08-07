use chrono::{self, DateTime, Local};
use sysinfo::System;

pub fn show_processes() -> u8 {
    println!("PID\tUID\tCPU\tMEM\tSTART\t\tNAME");
    for process in get_processes() {
        println!("{}", process);
    }
    0
}

pub fn get_processes() -> Vec<String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut processes = Vec::new();
    for (pid, proc) in sys.processes() {
        let timestamp: i64 = proc.start_time() as i64;
        let start_time: DateTime<Local> = DateTime::from_timestamp(timestamp, 0).unwrap().into();
        processes.push(format!(
            "{}\t{}\t{:.2}%\t{}MB\t{}\t{}",
            pid,
            **proc.user_id().unwrap(),
            proc.cpu_usage(),
            proc.memory() / 1024 / 1024,
            start_time.format("%H:%M:%S"),
            proc.name().to_string_lossy()
        ));
    }
    processes
}
