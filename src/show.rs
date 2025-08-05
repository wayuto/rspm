use chrono::{self, DateTime, Local};
use sysinfo::System;

pub fn show_processes() -> u8 {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("PID\tUID\tCPU\tMEM\tSTART\t\tNAME");
    for (pid, proc) in sys.processes() {
        let timestamp: i64 = proc.start_time() as i64;
        let start_time: DateTime<Local> = DateTime::from_timestamp(timestamp, 0).unwrap().into();
        println!(
            "{}\t{}\t{:.2}%\t{}MB\t{}\t{}",
            pid,
            **proc.user_id().unwrap(),
            proc.cpu_usage(),
            proc.memory() / 1024 / 1024,
            start_time.format("%H:%M:%S"),
            proc.name().to_string_lossy()
        );
    }

    return 0;
}
