use slint::SharedString;
use sysinfo::{System, SystemExt, DiskExt};
use crate::{ui::DiskData, ram::round2};

pub fn update_storage_data(sys: &mut System) -> Vec<DiskData> {
    let mut data: Vec<DiskData> = Vec::default();
    sys.refresh_disks();
    for disk in sys.disks() {
        let available_space = round2(bytes_to_gb(disk.available_space()) as f32);
        let total_space = round2(bytes_to_gb(disk.total_space()) as f32);
        let used = 100 - (available_space / total_space) as i32;
        let mut name = "Unknown";
        let mut os_name = disk.name().to_str();
        match os_name {
            None => {}
            Some(raw_name) => {name = raw_name}
        }
        data.push(DiskData { name: SharedString::from(name), used: used, volume: total_space })
    }
    return data;
}

fn bytes_to_gb(bytes: u64) -> u64 {
    bytes / 1000000000
}