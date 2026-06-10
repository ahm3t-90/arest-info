use std::{thread, time::Duration};

use crate::data_models::*;
use sysinfo::{Disks, System};
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        os_name: System::name(),
        kernel_version: System::kernel_version(),
        os_version: System::os_version(),
        host_name: System::host_name(),
    }
}
pub fn get_memory_info() -> MemoryInfo {
    let mut sys = System::new_all();
    sys.refresh_memory();

    MemoryInfo {
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        free_memory: sys.free_memory(),
    }
}
pub fn get_cpus_info() -> Vec<CpuInfo> {
    let mut sys = System::new_all();
    sys.refresh_cpu_all();
    thread::sleep(Duration::from_millis(500));
    sys.refresh_cpu_all();
    sys.cpus()
        .iter()
        .enumerate()
        .map(|(i, cpu)| CpuInfo {
            cpu_num: i,
            model_name: cpu.brand().to_string(),
            usage_percentage: cpu.cpu_usage(),
            frequency: cpu.frequency(),
        })
        .collect()
}
pub fn get_disks_info() -> Vec<DiskInfo> {
    let disks = Disks::new_with_refreshed_list();
    disks
        .iter()
        .map(|disk| DiskInfo {
            total_space: disk.total_space(),
            used_space: disk.total_space() - disk.available_space(),
            free_space: disk.available_space(),
        })
        .collect()
}

pub fn get_processes_info() -> Vec<ProcessInfo> {
    let mut sys = System::new();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
    let  processes = sys.processes();
    processes
        .iter()
        .map(|(pid, process)| ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().to_string(),
            cpu_usage: process.cpu_usage(),
            memory_usage: process.memory(),
        })
        .collect()
}
pub fn get_system_data() -> SystemData {
    let cpus = get_cpus_info();
    let total_usage: f32 = cpus.iter().map(|cpu| cpu.usage_percentage).sum();
    let avarage_usage = total_usage / cpus.len() as f32;
    let cpu_name = &cpus[0].model_name;
    let cpu_freaquency = cpus[0].frequency;

    SystemData {
        system_info: get_system_info(),
        memory_info: get_memory_info(),
        cpu_info: CpuInfo {
            cpu_num: 0,
            model_name: cpu_name.to_string(),
            usage_percentage: avarage_usage,
            frequency: cpu_freaquency,
        },
        disk_info: get_disks_info(),
    }
}
