use objc2::rc::{autoreleasepool, Retained};
use objc2::runtime::AnyObject;
use objc2::{class, msg_send};
use objc2_foundation::{NSPoint, NSRect, NSSize, NSString};

#[link(name = "AppKit", kind = "framework")]
extern "C" {}

const COMPOSITING_COPY: usize = 1;
const BITMAP_PNG: usize = 4;

pub struct RawRunningApp {
    pub pid: i32,
    pub name: String,
    pub bundle_id: Option<String>,
    pub bundle_path: Option<String>,
    pub activation_policy: isize,
    pub is_active: bool,
    pub is_hidden: bool,
}

pub fn running_apps() -> Vec<RawRunningApp> {
    autoreleasepool(|_| unsafe {
        let workspace: Retained<AnyObject> = msg_send![class!(NSWorkspace), sharedWorkspace];
        let apps: Retained<AnyObject> = msg_send![&workspace, runningApplications];
        let count: usize = msg_send![&apps, count];
        (0..count)
            .map(|i| {
                let app: Retained<AnyObject> = msg_send![&apps, objectAtIndex: i];
                read_running_app(&app)
            })
            .collect()
    })
}

unsafe fn read_running_app(app: &AnyObject) -> RawRunningApp {
    let name: Option<Retained<NSString>> = msg_send![app, localizedName];
    let bundle_id: Option<Retained<NSString>> = msg_send![app, bundleIdentifier];
    RawRunningApp {
        pid: msg_send![app, processIdentifier],
        name: name.map(|n| n.to_string()).unwrap_or_default(),
        bundle_id: bundle_id.map(|b| b.to_string()),
        bundle_path: bundle_path(app),
        activation_policy: msg_send![app, activationPolicy],
        is_active: msg_send![app, isActive],
        is_hidden: msg_send![app, isHidden],
    }
}

unsafe fn bundle_path(app: &AnyObject) -> Option<String> {
    let url: Option<Retained<AnyObject>> = msg_send![app, bundleURL];
    let path: Option<Retained<NSString>> = msg_send![&url?, path];
    path.map(|p| p.to_string())
}

unsafe fn running_app(pid: i32) -> Option<Retained<AnyObject>> {
    msg_send![class!(NSRunningApplication), runningApplicationWithProcessIdentifier: pid]
}

pub fn terminate(pid: i32, force: bool) -> bool {
    autoreleasepool(|_| unsafe {
        let Some(app) = running_app(pid) else {
            return false;
        };
        match force {
            true => msg_send![&app, forceTerminate],
            false => msg_send![&app, terminate],
        }
    })
}

pub fn icon_png(pid: i32, side: f64) -> Option<Vec<u8>> {
    autoreleasepool(|_| unsafe {
        let app = running_app(pid)?;
        let image: Option<Retained<AnyObject>> = msg_send![&app, icon];
        let image = image?;
        render_png(&image, side)
    })
}

unsafe fn render_png(image: &AnyObject, side: f64) -> Option<Vec<u8>> {
    let pixels = side as isize;
    let color_space = NSString::from_str("NSDeviceRGBColorSpace");
    let rep: Option<Retained<AnyObject>> = msg_send![
        msg_send![class!(NSBitmapImageRep), alloc],
        initWithBitmapDataPlanes: std::ptr::null_mut::<*mut u8>(),
        pixelsWide: pixels,
        pixelsHigh: pixels,
        bitsPerSample: 8isize,
        samplesPerPixel: 4isize,
        hasAlpha: true,
        isPlanar: false,
        colorSpaceName: &*color_space,
        bytesPerRow: 0isize,
        bitsPerPixel: 0isize
    ];
    let rep = rep?;
    draw_into(image, &rep, side)?;
    let properties: Retained<AnyObject> = msg_send![class!(NSDictionary), dictionary];
    let data: Option<Retained<AnyObject>> =
        msg_send![&rep, representationUsingType: BITMAP_PNG, properties: &*properties];
    let data = data?;
    let length: usize = msg_send![&data, length];
    let bytes: *const u8 = msg_send![&data, bytes];
    Some(std::slice::from_raw_parts(bytes, length).to_vec())
}

unsafe fn draw_into(image: &AnyObject, rep: &AnyObject, side: f64) -> Option<()> {
    let context: Option<Retained<AnyObject>> =
        msg_send![class!(NSGraphicsContext), graphicsContextWithBitmapImageRep: rep];
    let context = context?;
    let _: () = msg_send![class!(NSGraphicsContext), saveGraphicsState];
    let _: () = msg_send![class!(NSGraphicsContext), setCurrentContext: &*context];
    let target = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(side, side));
    let source = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(0.0, 0.0));
    let _: () = msg_send![
        image,
        drawInRect: target,
        fromRect: source,
        operation: COMPOSITING_COPY,
        fraction: 1.0f64
    ];
    let _: () = msg_send![class!(NSGraphicsContext), restoreGraphicsState];
    Some(())
}

pub struct RawVolumeCapacity {
    pub total: u64,
    pub available_for_important_usage: u64,
}

pub fn volume_capacity(path: &str) -> Option<RawVolumeCapacity> {
    autoreleasepool(|_| unsafe {
        let path = NSString::from_str(path);
        let url: Retained<AnyObject> = msg_send![class!(NSURL), fileURLWithPath: &*path, isDirectory: true];
        Some(RawVolumeCapacity {
            total: resource_number(&url, "NSURLVolumeTotalCapacityKey")?,
            available_for_important_usage: resource_number(
                &url,
                "NSURLVolumeAvailableCapacityForImportantUsageKey",
            )?,
        })
    })
}

unsafe fn resource_number(url: &AnyObject, key: &str) -> Option<u64> {
    let key = NSString::from_str(key);
    let mut value: Option<Retained<AnyObject>> = None;
    let ok: bool = msg_send![
        url,
        getResourceValue: &mut value,
        forKey: &*key,
        error: std::ptr::null_mut::<*mut AnyObject>()
    ];
    let value = value.filter(|_| ok)?;
    let number: i64 = msg_send![&value, longLongValue];
    u64::try_from(number).ok()
}
