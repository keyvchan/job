use libproc::libproc::{
    bsd_info::BSDInfo,
    proc_pid::{listpids, name, pidinfo, ProcType},
};

fn main() {
    if let Ok(pids) = listpids(ProcType::ProcAllPIDS) {
        println!("Found {} processes using listpids()", pids.len());
        for pid in pids {
            println!(
                "Process: pid {}, {}",
                pid,
                name(pid as i32).unwrap_or_else(|_| "Unknown".to_string())
            );
        }
    }
}
