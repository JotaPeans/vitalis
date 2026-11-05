use std::sync::mpsc;
use std::sync::Arc;

use tauri::{App, Manager, WebviewWindow, WindowEvent, Wry};

use crate::jobs::vitals_publisher::VitalsPublisher;
use crate::jobs::vitals_worker::VitalsWorker;
use crate::lib::config::CONFIG;
use crate::lib::ffi::spaces;

use super::container::Container;
use super::popover_state::PopoverState;
use super::tray::{self, POPOVER_LABEL};

pub fn configure(app: &mut App<Wry>) -> Result<(), Box<dyn std::error::Error>> {
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    let handle = app.handle().clone();
    let container = Container::build(handle.clone());
    let (wake_tx, wake_rx) = mpsc::channel();
    let state = Arc::new(PopoverState::new(wake_tx));
    app.manage(container.clone());
    tray::build(&handle, state.clone())?;
    hide_popover_on_blur(app, state.clone());
    VitalsWorker::new(container, VitalsPublisher::new(handle), state).spawn(wake_rx);
    Ok(())
}

fn hide_popover_on_blur(app: &App<Wry>, state: Arc<PopoverState>) {
    let Some(window) = app.get_webview_window(POPOVER_LABEL) else {
        return;
    };
    let _ = window.set_visible_on_all_workspaces(true);
    let target = window.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::Focused(false) = event {
            let (window, state) = (target.clone(), state.clone());
            std::thread::spawn(move || settle_blur(&window, &state));
        }
    });
}

fn settle_blur(window: &WebviewWindow<Wry>, state: &PopoverState) {
    std::thread::sleep(CONFIG.popover_blur_settle);
    if !window.is_visible().unwrap_or(false) || window.is_focused().unwrap_or(false) {
        return;
    }
    match state.follow_space_change(spaces::active_space_id()) {
        true => {
            let _ = window.set_focus();
        }
        false => tray::hide(window, state),
    }
}
