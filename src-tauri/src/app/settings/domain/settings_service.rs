use std::sync::Arc;

use crate::app::processes::domain::processes_interfaces::IAppRepository;
use crate::lib::config::CONFIG;
use crate::shared::errors::AppResult;

use super::settings_entity::{ProtectableAppEntity, SettingsEntity};
use super::settings_interfaces::ISettingsRepository;

pub struct SettingsService {
    repository: Arc<dyn ISettingsRepository>,
    app_repository: Arc<dyn IAppRepository>,
}

impl SettingsService {
    pub fn new(repository: Arc<dyn ISettingsRepository>, app_repository: Arc<dyn IAppRepository>) -> Self {
        Self { repository, app_repository }
    }

    pub fn get(&self) -> AppResult<SettingsEntity> {
        self.repository.load()
    }

    pub fn set_protected_apps(&self, bundle_ids: Vec<String>) -> AppResult<SettingsEntity> {
        let mut protected_apps = bundle_ids;
        protected_apps.sort();
        protected_apps.dedup();
        let settings = SettingsEntity { protected_apps };
        self.repository.save(&settings)?;
        Ok(settings)
    }

    pub fn protectable_apps(&self) -> AppResult<Vec<ProtectableAppEntity>> {
        let settings = self.get()?;
        let mut apps: Vec<ProtectableAppEntity> = self
            .app_repository
            .running_apps()
            .into_iter()
            .filter(|app| app.is_regular)
            .filter_map(|app| {
                let bundle_id = app.bundle_id?;
                (!CONFIG.cleaner.always_protected.contains(&bundle_id.as_str())).then(|| ProtectableAppEntity {
                    pid: app.pid,
                    is_protected: settings.is_protected(&bundle_id),
                    icon_png_base64: self.app_repository.icon_base64(app.pid, &bundle_id),
                    bundle_id,
                    name: app.name,
                })
            })
            .collect();
        apps.sort_by_key(|app| app.name.to_lowercase());
        Ok(apps)
    }
}
