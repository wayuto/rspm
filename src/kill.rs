use sysinfo::{self, Pid};

pub fn kill_process_by_pid(pid: u32) -> u8 {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    if let Some(process) = sys.process(Pid::from_u32(pid)) {
        if process.kill() {
            0;
        } else {
            println!("Failed to kill process '{}'", pid);
            1;
        }
    }

    println!("PID '{}' not found", pid);
    1
}

pub fn kill_process_by_name(name: String) -> u8 {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    for (_, process) in sys.processes() {
        if process.name().to_string_lossy() == name {
            if process.kill() {
                0;
            } else {
                println!("Failed to kill process '{}'", name);
                1;
            }
        }
    }

    println!("Process '{}' not found", name);
    1
}
