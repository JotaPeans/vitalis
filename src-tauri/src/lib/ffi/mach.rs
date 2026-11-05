use std::ffi::{c_void, CString};

#[repr(C)]
#[derive(Default)]
pub struct VmStatistics64 {
    pub free_count: u32,
    pub active_count: u32,
    pub inactive_count: u32,
    pub wire_count: u32,
    pub zero_fill_count: u64,
    pub reactivations: u64,
    pub pageins: u64,
    pub pageouts: u64,
    pub faults: u64,
    pub cow_faults: u64,
    pub lookups: u64,
    pub hits: u64,
    pub purges: u64,
    pub purgeable_count: u32,
    pub speculative_count: u32,
    pub decompressions: u64,
    pub compressions: u64,
    pub swapins: u64,
    pub swapouts: u64,
    pub compressor_page_count: u32,
    pub throttled_count: u32,
    pub external_page_count: u32,
    pub internal_page_count: u32,
    pub total_uncompressed_pages_in_compressor: u64,
}

#[repr(C)]
#[derive(Default)]
pub struct SwapUsage {
    pub total: u64,
    pub available: u64,
    pub used: u64,
    pub page_size: u32,
    pub encrypted: bool,
}

const HOST_VM_INFO64: i32 = 4;
const KERN_SUCCESS: i32 = 0;

extern "C" {
    fn mach_host_self() -> u32;
    fn host_statistics64(host: u32, flavor: i32, info: *mut i32, count: *mut u32) -> i32;
    fn host_page_size(host: u32, page_size: *mut usize) -> i32;
}

pub fn vm_statistics() -> Option<VmStatistics64> {
    let mut stats = VmStatistics64::default();
    let mut count = (size_of::<VmStatistics64>() / size_of::<i32>()) as u32;
    let result = unsafe {
        host_statistics64(
            mach_host_self(),
            HOST_VM_INFO64,
            (&mut stats as *mut VmStatistics64).cast(),
            &mut count,
        )
    };
    (result == KERN_SUCCESS).then_some(stats)
}

pub fn page_size() -> u64 {
    let mut size: usize = 0;
    match unsafe { host_page_size(mach_host_self(), &mut size) } {
        KERN_SUCCESS => size as u64,
        _ => 16_384,
    }
}

pub fn sysctl_value<T: Default>(name: &str) -> Option<T> {
    let c_name = CString::new(name).ok()?;
    let mut value = T::default();
    let mut size = size_of::<T>();
    let result = unsafe {
        libc::sysctlbyname(
            c_name.as_ptr(),
            (&mut value as *mut T).cast::<c_void>(),
            &mut size,
            std::ptr::null_mut(),
            0,
        )
    };
    (result == 0).then_some(value)
}
