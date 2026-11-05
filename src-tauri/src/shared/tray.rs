use std::sync::Arc;

use tauri::image::Image;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, WebviewWindow, Wry};
use tauri_plugin_positioner::{Position, WindowExt};

use crate::lib::ffi::spaces;

use super::popover_state::PopoverState;

pub const TRAY_ID: &str = "vitalis-tray";
pub const POPOVER_LABEL: &str = "popover";
const QUIT_MENU_ID: &str = "quit";

pub fn build(app: &AppHandle<Wry>, state: Arc<PopoverState>) -> tauri::Result<()> {
    let quit = MenuItem::with_id(app, QUIT_MENU_ID, "Sair do Vitalis", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit])?;
    TrayIconBuilder::with_id(TRAY_ID)
        .icon(Image::from_bytes(include_bytes!("../../icons/tray.png"))?)
        .icon_as_template(true)
        .title("--°")
        .tooltip("Vitalis")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            if event.id() == QUIT_MENU_ID {
                app.exit(0);
            }
        })
        .on_tray_icon_event(move |tray, event| {
            tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                toggle(tray.app_handle(), &state);
            }
        })
        .build(app)?;
    Ok(())
}

fn toggle(app: &AppHandle<Wry>, state: &PopoverState) {
    let Some(window) = app.get_webview_window(POPOVER_LABEL) else {
        return;
    };
    match window.is_visible().unwrap_or(false) {
        true => hide(&window, state),
        false if !state.was_just_hidden() => show(&window, state),
        false => {}
    }
}

pub fn show(window: &WebviewWindow<Wry>, state: &PopoverState) {
    if let Err(error) = window.move_window(Position::TrayCenter) {
        tracing::warn!(%error, "failed to position popover");
    }
    let _ = window.show();
    let _ = window.set_focus();
    state.mark_shown(spaces::active_space_id());
}

pub fn hide(window: &WebviewWindow<Wry>, state: &PopoverState) {
    let _ = window.hide();
    state.mark_hidden();
}
