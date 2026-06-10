use std::fmt::Display;


pub struct SystemInfo {
    pub os_name: Option<String>,
    pub kernel_version: Option<String>,
    pub os_version: Option<String>,
    pub(crate) host_name: Option<String>,
}
pub struct MemoryInfo {
    pub total_memory: u64,
    pub used_memory: u64,
    pub free_memory: u64,
}
pub struct CpuInfo {
    pub cpu_num: usize,
    pub model_name: String,
    pub usage_percentage: f32,
    pub frequency: u64,
}
pub struct DiskInfo {
    pub total_space: u64,
    pub used_space: u64,
    pub free_space: u64,
}


pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
}
pub struct SystemData {
    pub system_info: SystemInfo,
    pub memory_info: MemoryInfo,
    pub cpu_info: CpuInfo,
    pub disk_info: Vec<DiskInfo>,
}
impl Display for SystemInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r"
        OS INFO
OS Name:           {}
Kernel Version:    {}
OS Version:        {}
Host Name:         {}",
            self.os_name.as_deref().unwrap_or("Unkown"),
            self.kernel_version.as_deref().unwrap_or("Unkown"),
            self.os_version.as_deref().unwrap_or("Unkown"),
            self.host_name.as_deref().unwrap_or("Unkown")
        )
    }
}
impl Display for MemoryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r"
        MEMORY INFO
Total Memory:     {} MB
Used Memory:      {} MB
Free Memory:      {} MB",
            self.total_memory / 1024 / 1024 ,
            self.used_memory / 1024 / 1024,
            self.free_memory / 1024 / 1024
        )
    }
}

impl Display for CpuInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r"
        CPU INFO
CPU Num:          {}
Model Name:       {}
Usage Percentage: {:.2}%
Frequency:        {} MHz",
            self.cpu_num, self.model_name, self.usage_percentage, self.frequency
        )
    }
}
impl Display for DiskInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r"
        DISK INFO
Total Space:     {} GB
Used Space:      {} GB 
Free Space:      {} GB",
            self.total_space / 1024 / 1024 / 1024,
            self.used_space / 1024 / 1024 / 1024,
            self.free_space / 1024 / 1024 / 1024
        )
    }
}

impl Display for ProcessInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r"

NAME:{:^35}|PID:{:^9}|CPU USAGE:{:^5.2}|MEMORY USAGE:{:>8} KB     |",
            self.name,
            self.pid,
            self.cpu_usage,
            self.memory_usage / 1024 
        )
    }
}


impl Display for SystemData{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let disks:Vec<String> = self.disk_info
            .iter()
            .map(|d| format!("{}", d))
            .collect();
        let disks_str=disks.join("\n");


        write!(f,
        "{} {} {} {}",self.system_info,self.memory_info,self.cpu_info,disks_str)
    }
}
