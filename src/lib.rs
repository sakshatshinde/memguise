use human_bytes::human_bytes;
use rayon::prelude::*;
use rust_gpu_tools::*;
use sysinfo::{
    CpuRefreshKind, PidExt, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt,
};

/// Uses `get_process_info` under the hood
/// ### Arguments
/// * `processes` - A list containing the exact name of the processes
pub fn get_multi_process_info(processes: Vec<&str>) -> Vec<Vec<ProcessInfo>> {
    let multi_process_info = processes
        .par_chunks(8) // * chunk size 8 seems to give the best performance
        .map_init(
            || {
                // * spawning a System per thread, rayon will take care of spawning threads
                System::new_with_specifics(
                    RefreshKind::new()
                        .with_memory()
                        .with_cpu(CpuRefreshKind::everything())
                        .with_processes(ProcessRefreshKind::everything()),
                )
            },
            |sys, p| {
                p.iter()
                    .map(|p| get_process_info(sys, p))
                    .collect::<Vec<_>>()
            },
        )
        .flatten_iter()
        .collect();

    return multi_process_info;
}

pub fn get_multi_process_info_single_t(processes: Vec<&str>) -> Vec<Vec<ProcessInfo>> {
    let mut multi_process_info = Vec::<Vec<ProcessInfo>>::new();
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_memory()
            .with_cpu(CpuRefreshKind::everything())
            .with_processes(ProcessRefreshKind::everything()),
    );

    for process in processes {
        let each_process_info = get_process_info(&mut sys, process);
        multi_process_info.push(each_process_info);
    }

    return multi_process_info;
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
    return vec_p_info;
}

/// ProcessInfo is a struct to just keep track of a singular process
#[derive(Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub ram_usage: String,
    pub virtual_memory_usage: String,
}

#[allow(dead_code)]
fn core_to_core_latency() {
    todo!()
}

// ! fatal error LNK1181: cannot open input file 'OpenCL.lib' || some issue in linking? Need to install something i guess?
pub fn get_gpu_info_broken() {
    // ! rust_gpu_tools: https://docs.rs/rust-gpu-tools/latest/rust_gpu_tools/opencl/struct.Device.html
    let devices = Device::all();
    for device in devices.iter() {
        println!("Device : {:?}", device);
    }
}

#[allow(dead_code)]
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
