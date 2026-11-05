use tauri::State;

use crate::lib::config::CONFIG;
use crate::shared::container::Container;
use crate::shared::errors::{AppError, AppResult};

use super::app_usage_dto::AppUsageDto;

#[tauri::command]
pub async fn list_top_apps(container: State<'_, Container>) -> AppResult<Vec<AppUsageDto>> {
    let processes = container.processes.clone();
    let apps = tauri::async_runtime::spawn_blocking(move || processes.top_apps(CONFIG.top_apps_limit))
        .await
        .map_err(|e| AppError::Platform(e.to_string()))?;
    Ok(apps.iter().map(AppUsageDto::from).collect())
}
