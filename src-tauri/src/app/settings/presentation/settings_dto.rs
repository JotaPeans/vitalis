use serde::{Deserialize, Serialize};

use crate::app::settings::domain::settings_entity::{ProtectableAppEntity, SettingsEntity};
use crate::shared::errors::{AppError, AppResult};

const MAX_PROTECTED_APPS: usize = 200;
const MAX_BUNDLE_ID_LENGTH: usize = 255;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsDto {
    pub protected_apps: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtectableAppDto {
    pub pid: i32,
    pub bundle_id: String,
    pub name: String,
    pub icon: Option<String>,
    pub is_protected: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetProtectedAppsDto {
    pub bundle_ids: Vec<String>,
}

impl SetProtectedAppsDto {
    pub fn validate(self) -> AppResult<Vec<String>> {
        if self.bundle_ids.len() > MAX_PROTECTED_APPS {
            return Err(AppError::BadRequest("Lista de apps protegidos muito grande".into()));
        }
        let invalid = self
            .bundle_ids
            .iter()
            .any(|id| id.trim().is_empty() || id.len() > MAX_BUNDLE_ID_LENGTH);
        match invalid {
            true => Err(AppError::BadRequest("Identificador de app inválido".into())),
            false => Ok(self.bundle_ids),
        }
    }
}

impl From<&SettingsEntity> for SettingsDto {
    fn from(entity: &SettingsEntity) -> Self {
        Self { protected_apps: entity.protected_apps.clone() }
    }
}

impl From<&ProtectableAppEntity> for ProtectableAppDto {
    fn from(entity: &ProtectableAppEntity) -> Self {
        Self {
            pid: entity.pid,
            bundle_id: entity.bundle_id.clone(),
            name: entity.name.clone(),
            icon: entity.icon_png_base64.clone(),
            is_protected: entity.is_protected,
        }
    }
}
