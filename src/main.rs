use human_bytes::human_bytes;
use std::{thread, time::Duration};
use sysinfo::{CpuRefreshKind, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

fn main() {
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_memory()
            .with_cpu(CpuRefreshKind::everything())
            .with_processes(ProcessRefreshKind::everything()),
    );

    // ! For some reason task manager is showing a wild difference, but process explorer has +-1MB difference
    loop {
        sys.refresh_all();

        for process in sys.processes_by_exact_name("qbittorrent.exe") {
            println!(
                "\n pid: {} \n name: {} \n cpu: {}% \n ram: {} \n virt memory: {}",
                process.pid(),
                process.name(),
                process.cpu_usage(),
                human_bytes(process.memory() as f64),
                human_bytes((process.virtual_memory()) as f64),
            );
        }
        thread::sleep(Duration::from_millis(4000));
    }
}
