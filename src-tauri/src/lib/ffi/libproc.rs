use std::ffi::{c_char, c_int, c_void, CStr};

const RUSAGE_INFO_V4: c_int = 4;
const PHYS_FOOTPRINT_INDEX: usize = 9;
const PATH_MAX_SIZE: usize = 4 * 1024;
const NAME_MAX_SIZE: usize = 256;

extern "C" {
    fn proc_listallpids(buffer: *mut c_void, size: c_int) -> c_int;
    fn proc_pid_rusage(pid: c_int, flavor: c_int, buffer: *mut c_void) -> c_int;
    fn proc_pidpath(pid: c_int, buffer: *mut c_void, size: u32) -> c_int;
    fn proc_name(pid: c_int, buffer: *mut c_void, size: u32) -> c_int;
    fn responsibility_get_pid_responsible_for_pid(pid: c_int) -> c_int;
}

pub fn all_pids() -> Vec<i32> {
    let capacity = unsafe { proc_listallpids(std::ptr::null_mut(), 0) }.max(0) as usize + 64;
    let mut pids = vec![0i32; capacity];
    let bytes = (capacity * size_of::<i32>()) as c_int;
    let count = unsafe { proc_listallpids(pids.as_mut_ptr().cast(), bytes) }.max(0) as usize;
    pids.truncate(count);
    pids.retain(|pid| *pid > 0);
    pids
}

pub fn phys_footprint(pid: i32) -> Option<u64> {
    let mut buffer = [0u64; 64];
    let result = unsafe { proc_pid_rusage(pid, RUSAGE_INFO_V4, buffer.as_mut_ptr().cast()) };
    (result == 0).then_some(buffer[PHYS_FOOTPRINT_INDEX])
}

pub fn responsible_pid(pid: i32) -> i32 {
    match unsafe { responsibility_get_pid_responsible_for_pid(pid) } {
        r if r > 0 => r,
        _ => pid,
    }
}

pub fn path(pid: i32) -> Option<String> {
    let mut buffer = [0 as c_char; PATH_MAX_SIZE];
    let len = unsafe { proc_pidpath(pid, buffer.as_mut_ptr().cast(), PATH_MAX_SIZE as u32) };
    (len > 0).then(|| unsafe { CStr::from_ptr(buffer.as_ptr()) }.to_string_lossy().into_owned())
}

pub fn name(pid: i32) -> Option<String> {
    let mut buffer = [0 as c_char; NAME_MAX_SIZE];
    let len = unsafe { proc_name(pid, buffer.as_mut_ptr().cast(), NAME_MAX_SIZE as u32) };
    (len > 0).then(|| unsafe { CStr::from_ptr(buffer.as_ptr()) }.to_string_lossy().into_owned())
}

pub fn is_alive(pid: i32) -> bool {
    unsafe { libc::kill(pid, 0) == 0 }
}

const ARGUMENTS_BUFFER_SIZE: usize = 256 * 1024;

pub fn arguments(pid: i32) -> Vec<String> {
    let mut mib = [libc::CTL_KERN, libc::KERN_PROCARGS2, pid];
    let mut buffer = vec![0u8; ARGUMENTS_BUFFER_SIZE];
    let mut size = buffer.len();
    let result = unsafe {
        libc::sysctl(mib.as_mut_ptr(), 3, buffer.as_mut_ptr().cast(), &mut size, std::ptr::null_mut(), 0)
    };
    if result != 0 || size < size_of::<i32>() {
        return Vec::new();
    }
    parse_procargs(&buffer[..size])
}

// Layout: argc (i32), executable path, NUL padding, then argc NUL-terminated arguments.
fn parse_procargs(raw: &[u8]) -> Vec<String> {
    let argc = i32::from_ne_bytes([raw[0], raw[1], raw[2], raw[3]]).max(0) as usize;
    let rest = &raw[size_of::<i32>()..];
    let after_path = rest.iter().position(|b| *b == 0).unwrap_or(rest.len());
    let args_start = rest[after_path..].iter().position(|b| *b != 0).map_or(rest.len(), |i| after_path + i);
    rest[args_start..]
        .split(|b| *b == 0)
        .take(argc)
        .map(|arg| String::from_utf8_lossy(arg).into_owned())
        .collect()
}
