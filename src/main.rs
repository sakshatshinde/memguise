use human_bytes::human_bytes;
use sysinfo::{
    CpuRefreshKind, PidExt, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt,
};

fn main() {
    // ? Creating a global system
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_memory()
            .with_cpu(CpuRefreshKind::everything())
            .with_processes(ProcessRefreshKind::everything()),
    );

    let l_process = get_process_info(&mut sys, "firefox.exe");
    println!("{:?}", l_process);
}

/// Returns a vec of processes, usually only one process is included but sometimes multiple processes can
/// spawn with the same name. Like chrome spawing its child process for every tab
/// ### Arguments
///
/// * `sys` - The system initialized using `sysinfo::new()` or any of its variants
/// * `process_exact_name` - The exact name of the process, on Windows with `.exe`
fn get_process_info(sys: &mut System, process_exact_name: &str) -> Vec<ProcessInfo> {
    // ? Only refreshing what we requested
    sys.refresh_specifics(
        RefreshKind::new()
            .with_memory()
            .with_cpu(CpuRefreshKind::everything())
            .with_processes(ProcessRefreshKind::everything()),
    );

    let mut vec_p_info = Vec::<ProcessInfo>::new();

    for process in sys.processes_by_exact_name(process_exact_name) {
        let p_info = ProcessInfo {
            pid: process.pid().as_u32(),
            name: process.name().to_string(),
            cpu_usage: process.cpu_usage(),
            ram_usage: human_bytes(process.memory() as f64),
            virtual_memory_usage: human_bytes((process.virtual_memory()) as f64),
        };

        vec_p_info.push(p_info);
    }
    vec_p_info
}

/// ProcessInfo is a struct to just keep track of a singular process
#[derive(Debug)]
struct ProcessInfo {
    pid: u32,
    name: String,
    cpu_usage: f32,
    ram_usage: String,
    virtual_memory_usage: String,
}

fn core_to_core_latency() {
    todo!()
}

fn get_amd_gpu_info() {
    todo!()
    // ! machine_info: https://docs.rs/machine-info/latest/machine_info/
}

fn get_data_bandwidth_ram_gpu() {
    todo!()
    // ! ufe_abe: https://lib.rs/crates/ufe_abe
}
// * https://www.reddit.com/r/windows/comments/10p6ea0/can_i_have_some_help_understanding_memory_values/
/* Private bytes is the amount of memory the program has asked for that it doesn't share with any other programs. This doesn't include shared memory, which can be significant -- especially for web browsers, which run as multiple programs combined into one UI.

Working set is the amount of private memory that the OS has currently assigned to the program. If the program has allocated some memory but hasn't touched it in a while, Windows can swap it out of the working set and to disk to free up memory for other programs. If you're short on memory and the system is swapping, the working set matters more than the private size since the working set shows you which program is elbowing others out of RAM.

The Processes tab of Task Manager shows private working set for each program, but the total at the top includes private working sets and shared memory. Shared memory doesn't have a line item and thus confusingly the line items don't add up to the total. The Details tab should show you a lot of the same data as Process Explorer if you enable the columns, though perhaps with different names.
 */

// println!(
//     "\n pid: {} \n name: {} \n cpu: {}% \n ram: {} \n virt memory: {}",
//     process.pid(),
//     process.name(),
//     process.cpu_usage(),
//     human_bytes(process.memory() as f64),
//     human_bytes((process.virtual_memory()) as f64),
// );

// impl ProcessInfo {
//     fn default() -> ProcessInfo {
//         ProcessInfo {
//             pid: 0,
//             name: String::from("default"),
//             cpu_usage: 0.0,
//             ram_usage: String::from("default"),
//             virtual_memory_usage: String::from("default"),
//         }
//     }
// }
