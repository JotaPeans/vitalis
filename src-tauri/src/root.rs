#![allow(special_module_name)]

mod app;
mod jobs;
mod lib;
mod shared;

use tauri_plugin_autostart::MacosLauncher;
use tracing_subscriber::EnvFilter;

use app::cleaner::presentation::cleaner_controller::{force_quit_apps, get_clean_candidates, quit_apps};
use app::disk::presentation::disk_controller::get_disk;
use app::memory::presentation::memory_controller::get_memory;
use app::processes::presentation::processes_controller::list_top_apps;
use app::settings::presentation::settings_controller::{list_protectable_apps, set_protected_apps};
use app::temperature::presentation::temperature_controller::get_temperatures;

pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .setup(shared::bootstrap::configure)
        .invoke_handler(tauri::generate_handler![
            get_temperatures,
            get_memory,
            get_disk,
            list_top_apps,
            get_clean_candidates,
            quit_apps,
            force_quit_apps,
            set_protected_apps,
            list_protectable_apps,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Vitalis");
}
