#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGSMainConnectionID() -> i32;
    fn CGSGetActiveSpace(connection: i32) -> u64;
}

pub fn active_space_id() -> u64 {
    unsafe { CGSGetActiveSpace(CGSMainConnectionID()) }
}
