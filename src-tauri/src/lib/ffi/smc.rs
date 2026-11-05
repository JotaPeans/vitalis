use std::ffi::{c_char, c_void, CString};

const KERNEL_INDEX_SMC: u32 = 2;
const CMD_READ_BYTES: u8 = 5;
const CMD_READ_INDEX: u8 = 8;
const CMD_READ_KEY_INFO: u8 = 9;
const KERN_SUCCESS: i32 = 0;
const FLOAT_TYPE: u32 = u32::from_be_bytes(*b"flt ");
const KEY_COUNT_KEY: u32 = u32::from_be_bytes(*b"#KEY");

#[repr(C)]
#[derive(Default, Clone, Copy)]
struct SmcVersion {
    major: u8,
    minor: u8,
    build: u8,
    reserved: u8,
    release: u16,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
struct SmcPLimitData {
    version: u16,
    length: u16,
    cpu_p_limit: u32,
    gpu_p_limit: u32,
    mem_p_limit: u32,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
struct SmcKeyInfo {
    data_size: u32,
    data_type: u32,
    data_attributes: u8,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
struct SmcKeyData {
    key: u32,
    version: SmcVersion,
    p_limit_data: SmcPLimitData,
    key_info: SmcKeyInfo,
    result: u8,
    status: u8,
    data8: u8,
    data32: u32,
    bytes: [u8; 32],
}

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOServiceMatching(name: *const c_char) -> *mut c_void;
    fn IOServiceGetMatchingService(main_port: u32, matching: *mut c_void) -> u32;
    fn IOServiceOpen(service: u32, owning_task: u32, kind: u32, connect: *mut u32) -> i32;
    fn IOServiceClose(connect: u32) -> i32;
    fn IOObjectRelease(object: u32) -> i32;
    static mach_task_self_: u32;
    fn IOConnectCallStructMethod(
        connect: u32,
        selector: u32,
        input: *const c_void,
        input_size: usize,
        output: *mut c_void,
        output_size: *mut usize,
    ) -> i32;
}

pub struct RawSmcTemperature {
    pub key: String,
    pub celsius: f64,
}

struct SmcKey {
    code: u32,
    name: String,
    info: SmcKeyInfo,
}

pub struct SmcTemperatureClient {
    connection: u32,
    keys: Vec<SmcKey>,
}

impl SmcTemperatureClient {
    pub fn new(key_prefixes: &[&str]) -> Option<Self> {
        let connection = open_connection()?;
        let mut client = Self { connection, keys: Vec::new() };
        client.keys = client.discover_float_keys(key_prefixes);
        Some(client)
    }

    pub fn read_all(&self) -> Vec<RawSmcTemperature> {
        self.keys
            .iter()
            .filter_map(|key| {
                let celsius = self.read_float(key)?;
                Some(RawSmcTemperature { key: key.name.clone(), celsius: f64::from(celsius) })
            })
            .collect()
    }

    fn discover_float_keys(&self, prefixes: &[&str]) -> Vec<SmcKey> {
        let Some(count) = self.key_count() else {
            return Vec::new();
        };
        (0..count)
            .filter_map(|index| self.key_at(index))
            .filter(|(_, name)| prefixes.iter().any(|p| name.starts_with(p)))
            .filter_map(|(code, name)| {
                let info = self.key_info(code)?;
                (info.data_type == FLOAT_TYPE).then_some(SmcKey { code, name, info })
            })
            .collect()
    }

    fn key_count(&self) -> Option<u32> {
        let info = self.key_info(KEY_COUNT_KEY)?;
        let output = self.call(SmcKeyData { key: KEY_COUNT_KEY, key_info: info, data8: CMD_READ_BYTES, ..Default::default() })?;
        Some(u32::from_be_bytes([output.bytes[0], output.bytes[1], output.bytes[2], output.bytes[3]]))
    }

    fn key_at(&self, index: u32) -> Option<(u32, String)> {
        let output = self.call(SmcKeyData { data8: CMD_READ_INDEX, data32: index, ..Default::default() })?;
        let name = String::from_utf8_lossy(&output.key.to_be_bytes()).into_owned();
        Some((output.key, name))
    }

    fn key_info(&self, code: u32) -> Option<SmcKeyInfo> {
        self.call(SmcKeyData { key: code, data8: CMD_READ_KEY_INFO, ..Default::default() })
            .map(|output| output.key_info)
    }

    fn read_float(&self, key: &SmcKey) -> Option<f32> {
        let output = self.call(SmcKeyData { key: key.code, key_info: key.info, data8: CMD_READ_BYTES, ..Default::default() })?;
        Some(f32::from_le_bytes([output.bytes[0], output.bytes[1], output.bytes[2], output.bytes[3]]))
    }

    fn call(&self, input: SmcKeyData) -> Option<SmcKeyData> {
        let mut output = SmcKeyData::default();
        let mut output_size = size_of::<SmcKeyData>();
        let result = unsafe {
            IOConnectCallStructMethod(
                self.connection,
                KERNEL_INDEX_SMC,
                (&input as *const SmcKeyData).cast(),
                size_of::<SmcKeyData>(),
                (&mut output as *mut SmcKeyData).cast(),
                &mut output_size,
            )
        };
        (result == KERN_SUCCESS && output.result == 0).then_some(output)
    }
}

impl Drop for SmcTemperatureClient {
    fn drop(&mut self) {
        unsafe { IOServiceClose(self.connection) };
    }
}

fn open_connection() -> Option<u32> {
    let name = CString::new("AppleSMC").ok()?;
    let service = unsafe { IOServiceGetMatchingService(0, IOServiceMatching(name.as_ptr())) };
    if service == 0 {
        return None;
    }
    let mut connection = 0u32;
    let result = unsafe { IOServiceOpen(service, mach_task_self_, 0, &mut connection) };
    unsafe { IOObjectRelease(service) };
    (result == KERN_SUCCESS).then_some(connection)
}
