use tauri::{AppHandle, Emitter, Wry};

use crate::shared::tray::TRAY_ID;

use super::vitals_dto::VitalsDto;

pub const VITALS_EVENT: &str = "vitals";

pub struct VitalsPublisher {
    app: AppHandle<Wry>,
}

impl VitalsPublisher {
    pub fn new(app: AppHandle<Wry>) -> Self {
        Self { app }
    }

    pub fn publish(&self, vitals: &VitalsDto) {
        if let Err(error) = self.app.emit(VITALS_EVENT, vitals) {
            tracing::warn!(%error, "failed to emit vitals");
        }
    }

    pub fn update_tray(&self, soc_celsius: Option<f64>, memory_ratio: Option<f64>) {
        let Some(tray) = self.app.tray_by_id(TRAY_ID) else {
            return;
        };
        let title = tray_title(soc_celsius, memory_ratio);
        if let Err(error) = tray.set_title(Some(title)) {
            tracing::warn!(%error, "failed to update tray title");
        }
    }
}

fn tray_title(soc_celsius: Option<f64>, memory_ratio: Option<f64>) -> String {
    let temperature = soc_celsius.map_or("--°".to_string(), |c| format!("{c:.0}°"));
    let memory = memory_ratio.map_or("--%".to_string(), |r| format!("{:.0}%", r * 100.0));
    format!("{temperature} · {memory}")
}
