use criterion::{black_box, criterion_group, criterion_main, Criterion};
use memguise::*;
use sysinfo::{CpuRefreshKind, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

fn criterion_benchmark(c: &mut Criterion) {
    let g_sys = System::new_with_specifics(
        RefreshKind::new()
            .with_memory()
            .with_cpu(CpuRefreshKind::everything())
            .with_processes(ProcessRefreshKind::everything()),
    );
    let mut list_of_processes: Vec<&str> = g_sys
        .processes()
        .iter()
        .map(|(_, process)| process.name())
        .collect();

    list_of_processes.sort();
    list_of_processes.dedup();

    println!("--------------------------------");
    println!(
        "Starting the benchmark with {} processes",
        list_of_processes.len()
    );
    println!("--------------------------------");

    c.bench_function("Single Thread", |b| {
        b.iter(|| get_multi_process_info_single_t(black_box(list_of_processes.clone())))
    });

    c.bench_function("Rayon", |b| {
        b.iter(|| get_multi_process_info(black_box(list_of_processes.clone())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
