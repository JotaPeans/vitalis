use tauri::{AppHandle, Wry};
use tauri_plugin_store::StoreExt;

use crate::app::settings::domain::settings_entity::SettingsEntity;
use crate::app::settings::domain::settings_interfaces::ISettingsRepository;
use crate::lib::config::CONFIG;
use crate::shared::errors::{AppError, AppResult};

const PROTECTED_APPS_KEY: &str = "protectedApps";

pub struct StoreSettingsRepository {
    app: AppHandle<Wry>,
}

impl StoreSettingsRepository {
    pub fn new(app: AppHandle<Wry>) -> Self {
        Self { app }
    }

    fn store(&self) -> AppResult<std::sync::Arc<tauri_plugin_store::Store<Wry>>> {
        self.app
            .store(CONFIG.settings_store_file)
            .map_err(|e| AppError::Platform(format!("Falha ao abrir configurações: {e}")))
    }
}

impl ISettingsRepository for StoreSettingsRepository {
    fn load(&self) -> AppResult<SettingsEntity> {
        let protected_apps = self
            .store()?
            .get(PROTECTED_APPS_KEY)
            .and_then(|value| serde_json::from_value::<Vec<String>>(value).ok())
            .unwrap_or_default();
        Ok(SettingsEntity { protected_apps })
    }

    fn save(&self, settings: &SettingsEntity) -> AppResult<()> {
        let store = self.store()?;
        store.set(PROTECTED_APPS_KEY, serde_json::json!(settings.protected_apps));
        store
            .save()
            .map_err(|e| AppError::Platform(format!("Falha ao salvar configurações: {e}")))
    }
}
