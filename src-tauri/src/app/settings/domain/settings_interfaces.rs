use crate::shared::errors::AppResult;

use super::settings_entity::SettingsEntity;

pub trait ISettingsRepository: Send + Sync {
    fn load(&self) -> AppResult<SettingsEntity>;
    fn save(&self, settings: &SettingsEntity) -> AppResult<()>;
}
