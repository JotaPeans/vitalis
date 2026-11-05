use tauri::State;

use crate::shared::container::Container;
use crate::shared::errors::{AppError, AppResult};

use super::settings_dto::{ProtectableAppDto, SetProtectedAppsDto, SettingsDto};

#[tauri::command]
pub async fn set_protected_apps(
    container: State<'_, Container>,
    body: SetProtectedAppsDto,
) -> AppResult<SettingsDto> {
    let settings = container.settings.set_protected_apps(body.validate()?)?;
    Ok(SettingsDto::from(&settings))
}

#[tauri::command]
pub async fn list_protectable_apps(container: State<'_, Container>) -> AppResult<Vec<ProtectableAppDto>> {
    let settings = container.settings.clone();
    let apps = tauri::async_runtime::spawn_blocking(move || settings.protectable_apps())
        .await
        .map_err(|e| AppError::Platform(e.to_string()))??;
    Ok(apps.iter().map(ProtectableAppDto::from).collect())
}
