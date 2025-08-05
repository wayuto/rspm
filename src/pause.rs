use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use sysinfo::{self, System};

pub fn pause_process_by_pid(pid: u32) -> () {
    let mut sys = System::new_all();
    sys.refresh_all();

    if let Some(_) = sys.process(sysinfo::Pid::from_u32(pid)) {
        signal::kill(Pid::from_raw(pid as i32), Signal::SIGSTOP).unwrap();
    }
}

pub fn resume_process_by_pid(pid: u32) -> () {
    let mut sys = System::new_all();
    sys.refresh_all();

    if let Some(_) = sys.process(sysinfo::Pid::from_u32(pid)) {
        signal::kill(Pid::from_raw(pid as i32), Signal::SIGCONT).unwrap();
    }
}

pub fn pause_process_by_name(name: String) -> () {
    let mut sys = System::new_all();
    sys.refresh_all();

    for (pid, process) in sys.processes() {
        if process.name().to_string_lossy() == name {
            signal::kill(Pid::from_raw(pid.as_u32() as i32), Signal::SIGSTOP).unwrap();
        }
    }
}

pub fn resume_process_by_name(name: String) -> () {
    let mut sys = System::new_all();
    sys.refresh_all();

    for (pid, process) in sys.processes() {
        if process.name().to_string_lossy() == name {
            signal::kill(Pid::from_raw(pid.as_u32() as i32), Signal::SIGCONT).unwrap();
        }
    }
}
