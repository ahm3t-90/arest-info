use crate::get_system_info::*;
use clap::Parser;
mod data_models;
mod get_system_info;
fn main() {
    let args=Args::parse();
    if args.systeminfo{
        let system_info = get_system_info();
        println!("{}", system_info);
    }
    if args.memoryinfo{
        let memory_info = get_memory_info();
        println!("{}",memory_info);
    }
    if args.cpuinfo{
        let cpu_info = get_cpus_info();
        for cpu in cpu_info {
            println!("{}", cpu);
        }
    }
    if args.diskinfo{
        let disk_info = get_disks_info();
        for disk in disk_info{
            println!("{}",disk);
        }
    }
    if args.processinfo{
        for process in get_processes_info(){
        println!("{}",process)}
    }
    if args.all  {
        println!("{}",get_system_data())
    }



}
#[derive(Parser)]

struct Args{
    #[arg(short, long)]
    systeminfo:bool,
    #[arg(short, long)]
    memoryinfo:bool,
    #[arg(short, long)]
    cpuinfo:bool,
    #[arg(short, long)]
    diskinfo:bool,
    #[arg(short,long)]
    processinfo:bool,
    #[arg(short, long)]
    all:bool,
}

