extern crate libc;
use std::mem::zeroed;
use std::ffi::{CString};

struct DiskStats {
    used : u64,
    avail : u64,
    capacity: u8
}

impl DiskStats {
    fn new(used : u64, avail: u64) -> DiskStats {
        DiskStats{  used : used, 
                    avail : avail,
                    capacity : 100 - (avail*100/used) as u8
                    }
    }
}
fn diskspace(mount_point: &str) -> Result<DiskStats, String> {
    unsafe {
        let mountp = CString::new(mount_point).unwrap();
        let mut stats: libc::statvfs = zeroed();
        if libc::statvfs(mountp.as_ptr(), &mut stats) != 0 {
            return Err(format!("Unable to retrieve mount point information {}", mount_point));
        }
        let disk_stats = DiskStats::new( stats.f_frsize * stats.f_blocks as u64, 
                                         stats.f_frsize * stats.f_bfree as u64);
        Ok(disk_stats)
    }
}

fn main() {
    let stats = diskspace("/").unwrap();
    println!("Size {}GB, Avail {}GB, Capacity {}%", stats.used/1024/1024/1024,stats.avail/1024/1024/1024, stats.capacity);
}
