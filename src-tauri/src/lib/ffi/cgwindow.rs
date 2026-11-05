use std::collections::HashSet;

use super::cf::{self, CFTypeRef, CfOwned};

const EXCLUDE_DESKTOP_ELEMENTS: u32 = 1 << 4;
const NORMAL_WINDOW_LAYER: i64 = 0;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGWindowListCopyWindowInfo(option: u32, relative_to: u32) -> CFTypeRef;
}

pub fn pids_with_windows() -> HashSet<i32> {
    let (Some(owner_key), Some(layer_key)) = (cf::string("kCGWindowOwnerPID"), cf::string("kCGWindowLayer"))
    else {
        return HashSet::new();
    };
    let Some(windows) = CfOwned::from_create(unsafe {
        CGWindowListCopyWindowInfo(EXCLUDE_DESKTOP_ELEMENTS, 0)
    }) else {
        return HashSet::new();
    };
    unsafe { cf::array_items(windows.as_ptr()) }
        .into_iter()
        .filter_map(|window| unsafe {
            let layer = cf::to_i64(cf::dictionary_get(window, &layer_key))?;
            let pid = cf::to_i64(cf::dictionary_get(window, &owner_key))?;
            (layer == NORMAL_WINDOW_LAYER).then_some(pid as i32)
        })
        .collect()
}
