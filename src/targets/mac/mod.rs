use cocoa::appkit::NSScreen;
use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use core_graphics_helmer_fork::display::{CGDirectDisplayID, CGDisplay, CGMainDisplayID};
use core_graphics_helmer_fork::window::CGWindowID;
use objc::{msg_send, sel, sel_impl};
use screencapturekit::sc_shareable_content::SCShareableContent;

use super::{Display, Target};

fn get_display_name(display_id: CGDirectDisplayID) -> String {
    unsafe {
        // Get all screens
        let screens: id = NSScreen::screens(nil);
        let count: u64 = msg_send![screens, count];

        for i in 0..count {
            let screen: id = msg_send![screens, objectAtIndex: i];
            let device_description: id = msg_send![screen, deviceDescription];
            let display_id_number: id = msg_send![device_description, objectForKey: NSString::alloc(nil).init_str("NSScreenNumber")];
            let display_id_number: u32 = msg_send![display_id_number, unsignedIntValue];

            if display_id_number == display_id {
                let localized_name: id = msg_send![screen, localizedName];
                let name: *const i8 = msg_send![localized_name, UTF8String];
                return std::ffi::CStr::from_ptr(name)
                    .to_string_lossy()
                    .into_owned();
            }
        }

        format!("Unknown Display {}", display_id)
    }
}

pub fn get_all_targets() -> Vec<Target> {
    let mut targets: Vec<Target> = Vec::new();

    let content = SCShareableContent::current();

    // Add displays to targets
    for display in content.displays {
        let id: CGDirectDisplayID = display.display_id;
        let raw_handle = CGDisplay::new(id);
        let title = get_display_name(id);

        let target = Target::Display(super::Display {
            id,
            title,
            raw_handle,
        });

        targets.push(target);
    }

    // Add windows to targets
    for window in content.windows {
        if window.title.is_some() {
            let id = window.window_id;
            let title = window.title.expect("Window title not found");
            let raw_handle: CGWindowID = id;

            let target = Target::Window(super::Window {
                id,
                title,
                raw_handle,
            });
            targets.push(target);
        }
    }

    targets
}

pub fn get_main_display() -> Display {
    let id = unsafe { CGMainDisplayID() };
    let title = get_display_name(id);

    Display {
        id,
        title,
        raw_handle: CGDisplay::new(id),
    }
}

// scap-vc patch: the original implementations resolved windows via
// NSApp().windowWithWindowNumber:, which can only see windows owned by the
// CALLING process. For any other app's window the lookup returns nil, and
// messaging nil yields zeroed scalars — so cross-process window capture got a
// 0x0 SCStreamConfiguration and always failed to start. Window dimensions now
// come from ScreenCaptureKit's own shareable-content listing (cross-process
// by design, same permission gate as capture itself).

pub fn get_scale_factor(target: &Target) -> f64 {
    match target {
        // Window bounds are reported in points; capture output is scaled by
        // the backing scale. Without a reliable cross-process window->display
        // mapping in this crate, use the main display's scale (correct for
        // single-display and uniform-DPI setups; on mixed-DPI multi-monitor
        // rigs a window on a non-main display gets the main display's scale,
        // which changes output resolution but not correctness).
        Target::Window(_) => {
            let mode = CGDisplay::new(unsafe { CGMainDisplayID() })
                .display_mode()
                .expect("main display has a display mode");
            (mode.pixel_width() / mode.width()) as f64
        }
        Target::Display(display) => {
            let mode = display.raw_handle.display_mode().unwrap();
            (mode.pixel_width() / mode.width()) as f64
        }
    }
}

pub fn get_target_dimensions(target: &Target) -> (u64, u64) {
    match target {
        Target::Window(window) => SCShareableContent::current()
            .windows
            .into_iter()
            .find(|w| w.window_id == window.id)
            // A vanished window yields 0x0; with capture start returning a
            // Result (scap-vc patch), that surfaces as an error, not a panic.
            .map_or((0, 0), |w| (w.width as u64, w.height as u64)),
        Target::Display(display) => {
            let mode = display.raw_handle.display_mode().unwrap();
            (mode.width(), mode.height())
        }
    }
}
