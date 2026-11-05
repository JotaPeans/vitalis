use std::ffi::c_void;

use super::cf::{self, CFTypeRef, CfOwned};

const SENSOR_USAGE_PAGE: i32 = 0xff00;
const TEMPERATURE_USAGE: i32 = 5;
const EVENT_TYPE_TEMPERATURE: i64 = 15;
const TEMPERATURE_FIELD: i32 = 15 << 16;

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOHIDEventSystemClientCreate(alloc: CFTypeRef) -> CFTypeRef;
    fn IOHIDEventSystemClientSetMatching(client: CFTypeRef, matching: CFTypeRef) -> i32;
    fn IOHIDEventSystemClientCopyServices(client: CFTypeRef) -> CFTypeRef;
    fn IOHIDServiceClientCopyProperty(service: CFTypeRef, key: CFTypeRef) -> CFTypeRef;
    fn IOHIDServiceClientCopyEvent(
        service: CFTypeRef,
        event_type: i64,
        options: i32,
        timestamp: i64,
    ) -> *const c_void;
    fn IOHIDEventGetFloatValue(event: *const c_void, field: i32) -> f64;
}

pub struct RawTemperature {
    pub name: String,
    pub celsius: f64,
}

pub struct HidTemperatureClient {
    client: CfOwned,
    product_key: CfOwned,
}

// SAFETY: the client is only touched behind a Mutex in the repository.
unsafe impl Send for HidTemperatureClient {}

impl HidTemperatureClient {
    pub fn new() -> Option<Self> {
        let client = CfOwned::from_create(unsafe { IOHIDEventSystemClientCreate(std::ptr::null()) })?;
        let page_key = cf::string("PrimaryUsagePage")?;
        let usage_key = cf::string("PrimaryUsage")?;
        let page = cf::number_i32(SENSOR_USAGE_PAGE)?;
        let usage = cf::number_i32(TEMPERATURE_USAGE)?;
        let matching = cf::dictionary(&[(&page_key, &page), (&usage_key, &usage)])?;
        unsafe { IOHIDEventSystemClientSetMatching(client.as_ptr(), matching.as_ptr()) };
        Some(Self {
            client,
            product_key: cf::string("Product")?,
        })
    }

    pub fn read_all(&self) -> Vec<RawTemperature> {
        let Some(services) =
            CfOwned::from_create(unsafe { IOHIDEventSystemClientCopyServices(self.client.as_ptr()) })
        else {
            return Vec::new();
        };
        unsafe { cf::array_items(services.as_ptr()) }
            .into_iter()
            .filter_map(|service| self.read_service(service))
            .collect()
    }

    fn read_service(&self, service: CFTypeRef) -> Option<RawTemperature> {
        let name = CfOwned::from_create(unsafe {
            IOHIDServiceClientCopyProperty(service, self.product_key.as_ptr())
        })?;
        let name = unsafe { cf::to_rust_string(name.as_ptr()) }?;
        let event = CfOwned::from_create(unsafe {
            IOHIDServiceClientCopyEvent(service, EVENT_TYPE_TEMPERATURE, 0, 0)
        })?;
        let celsius = unsafe { IOHIDEventGetFloatValue(event.as_ptr(), TEMPERATURE_FIELD) };
        Some(RawTemperature { name, celsius })
    }
}
