
use libc;
use std::mem::zeroed;
use std::ffi::{CString};

pub const HUMAN_BASE_1000 : u64 = 1000;
pub const HUMAN_BASE_1024 : u64 = 1024;
pub const HUMAN_BASE : u64 = 1;

pub fn human_readable_size(value: u64, base: u64) -> String {
    let mut v = value;
    let mut unit = "B";
    if base != HUMAN_BASE {        
        if value >= base * base * base {
            v = value/base/base/base;
            unit = "GB";
        } else if value >= base * base {
            v = value/base/base;
            unit = "MB";
        } else if value >= base {
            v = value/base;
            unit = "KB";
        }
    }
    format!("{}{}", v, unit)
}

#[test]
fn test_human_readable_size() {
    const KB : u64 = 1024;
    const MB : u64 = 1024 * 1024;
    const GB : u64 = 1024 * 1024 * 1024;

    assert_eq!(human_readable_size(0, HUMAN_BASE), "0B");
    assert_eq!(human_readable_size(0, HUMAN_BASE_1000), "0B");
    assert_eq!(human_readable_size(0, HUMAN_BASE_1024), "0B");

    assert_eq!(human_readable_size(999, HUMAN_BASE), "999B");
    assert_eq!(human_readable_size(999, HUMAN_BASE_1000), "999B");
    assert_eq!(human_readable_size(999, HUMAN_BASE_1024), "999B");

    assert_eq!(human_readable_size(KB, HUMAN_BASE), "1024B");
    assert_eq!(human_readable_size(KB, HUMAN_BASE_1000), "1KB");
    assert_eq!(human_readable_size(KB, HUMAN_BASE_1024), "1KB");

    assert_eq!(human_readable_size(MB, HUMAN_BASE), "1048576B");
    assert_eq!(human_readable_size(MB, HUMAN_BASE_1000), "1MB");
    assert_eq!(human_readable_size(MB, HUMAN_BASE_1024), "1MB");

    assert_eq!(human_readable_size(GB, HUMAN_BASE), "1073741824B");
    assert_eq!(human_readable_size(GB, HUMAN_BASE_1000), "1GB");
    assert_eq!(human_readable_size(GB, HUMAN_BASE_1024), "1GB");
}
pub struct DiskStats {
    pub used : u64,
    pub avail : u64,
    pub capacity: u8
}

impl DiskStats {
    fn new(used : u64, avail: u64) -> DiskStats {
        DiskStats{  used : used, 
                    avail : avail,
                    capacity : 100 - (avail*100/used) as u8
                    }
    }
}
pub fn diskspace(mount_point: &str) -> Result<DiskStats, String> {
    unsafe {
        let mountp = CString::new(mount_point).unwrap();
        let mut stats: libc::statvfs = zeroed();
        if libc::statvfs(mountp.as_ptr(), &mut stats) != 0 {
            return Err(format!("Unable to retrieve mount point information {}", mount_point));
        }
        let disk_stats = DiskStats::new( stats.f_frsize * u64::from(stats.f_blocks), 
                                         stats.f_frsize * u64::from(stats.f_bfree));
        Ok(disk_stats)
    }
}