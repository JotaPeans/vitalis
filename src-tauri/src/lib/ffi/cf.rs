use std::ffi::{c_char, c_void, CStr, CString};

pub type CFTypeRef = *const c_void;
pub type CFIndex = isize;

const UTF8_ENCODING: u32 = 0x0800_0100;
const NUMBER_SINT32_TYPE: CFIndex = 3;
const NUMBER_SINT64_TYPE: CFIndex = 4;

#[repr(C)]
pub struct CFDictionaryCallBacks {
    _private: [u8; 0],
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFTypeDictionaryKeyCallBacks: CFDictionaryCallBacks;
    static kCFTypeDictionaryValueCallBacks: CFDictionaryCallBacks;

    fn CFRelease(cf: CFTypeRef);
    fn CFGetTypeID(cf: CFTypeRef) -> usize;
    fn CFStringGetTypeID() -> usize;
    fn CFNumberGetTypeID() -> usize;
    fn CFStringCreateWithCString(alloc: CFTypeRef, s: *const c_char, encoding: u32) -> CFTypeRef;
    fn CFStringGetCString(s: CFTypeRef, buf: *mut c_char, size: CFIndex, encoding: u32) -> bool;
    fn CFNumberCreate(alloc: CFTypeRef, kind: CFIndex, value: *const c_void) -> CFTypeRef;
    fn CFNumberGetValue(n: CFTypeRef, kind: CFIndex, value: *mut c_void) -> bool;
    fn CFDictionaryCreate(
        alloc: CFTypeRef,
        keys: *const CFTypeRef,
        values: *const CFTypeRef,
        count: CFIndex,
        key_callbacks: *const CFDictionaryCallBacks,
        value_callbacks: *const CFDictionaryCallBacks,
    ) -> CFTypeRef;
    fn CFDictionaryGetValue(dict: CFTypeRef, key: CFTypeRef) -> CFTypeRef;
    fn CFArrayGetCount(array: CFTypeRef) -> CFIndex;
    fn CFArrayGetValueAtIndex(array: CFTypeRef, index: CFIndex) -> CFTypeRef;
}

pub struct CfOwned(CFTypeRef);

impl CfOwned {
    pub fn from_create(ptr: CFTypeRef) -> Option<Self> {
        (!ptr.is_null()).then(|| Self(ptr))
    }

    pub fn as_ptr(&self) -> CFTypeRef {
        self.0
    }
}

impl Drop for CfOwned {
    fn drop(&mut self) {
        unsafe { CFRelease(self.0) }
    }
}

pub fn string(value: &str) -> Option<CfOwned> {
    let c = CString::new(value).ok()?;
    CfOwned::from_create(unsafe {
        CFStringCreateWithCString(std::ptr::null(), c.as_ptr(), UTF8_ENCODING)
    })
}

pub fn number_i32(value: i32) -> Option<CfOwned> {
    CfOwned::from_create(unsafe {
        CFNumberCreate(
            std::ptr::null(),
            NUMBER_SINT32_TYPE,
            (&value as *const i32).cast(),
        )
    })
}

pub fn dictionary(pairs: &[(&CfOwned, &CfOwned)]) -> Option<CfOwned> {
    let keys: Vec<CFTypeRef> = pairs.iter().map(|(k, _)| k.as_ptr()).collect();
    let values: Vec<CFTypeRef> = pairs.iter().map(|(_, v)| v.as_ptr()).collect();
    CfOwned::from_create(unsafe {
        CFDictionaryCreate(
            std::ptr::null(),
            keys.as_ptr(),
            values.as_ptr(),
            pairs.len() as CFIndex,
            &kCFTypeDictionaryKeyCallBacks,
            &kCFTypeDictionaryValueCallBacks,
        )
    })
}

/// # Safety
/// `ptr` must be null or a valid CFTypeRef.
pub unsafe fn to_rust_string(ptr: CFTypeRef) -> Option<String> {
    if ptr.is_null() || CFGetTypeID(ptr) != CFStringGetTypeID() {
        return None;
    }
    let mut buf = [0 as c_char; 256];
    if !CFStringGetCString(ptr, buf.as_mut_ptr(), buf.len() as CFIndex, UTF8_ENCODING) {
        return None;
    }
    Some(CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned())
}

/// # Safety
/// `ptr` must be null or a valid CFTypeRef.
pub unsafe fn to_i64(ptr: CFTypeRef) -> Option<i64> {
    if ptr.is_null() || CFGetTypeID(ptr) != CFNumberGetTypeID() {
        return None;
    }
    let mut value: i64 = 0;
    CFNumberGetValue(ptr, NUMBER_SINT64_TYPE, (&mut value as *mut i64).cast())
        .then_some(value)
}

/// # Safety
/// `dict` must be a valid CFDictionaryRef.
pub unsafe fn dictionary_get(dict: CFTypeRef, key: &CfOwned) -> CFTypeRef {
    CFDictionaryGetValue(dict, key.as_ptr())
}

/// # Safety
/// `array` must be a valid CFArrayRef; returned items are borrowed from it.
pub unsafe fn array_items(array: CFTypeRef) -> Vec<CFTypeRef> {
    (0..CFArrayGetCount(array))
        .map(|i| CFArrayGetValueAtIndex(array, i))
        .collect()
}
