use serde::{Deserialize, Serialize};

use crate::app::cleaner::domain::clean_candidate_entity::{
    CleanCandidateEntity, QuitOutcomeEntity, QuitResultEntity,
};
use crate::shared::errors::{AppError, AppResult};

const MAX_PIDS_PER_REQUEST: usize = 100;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuitAppsDto {
    pub pids: Vec<i32>,
}

impl QuitAppsDto {
    pub fn validate(self) -> AppResult<Vec<i32>> {
        match self.pids.len() {
            0 => Err(AppError::BadRequest("Selecione ao menos um app".into())),
            n if n > MAX_PIDS_PER_REQUEST => Err(AppError::BadRequest("Apps demais selecionados".into())),
            _ if self.pids.iter().any(|pid| *pid <= 1) => {
                Err(AppError::BadRequest("Processo inválido".into()))
            }
            _ => Ok(self.pids),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanCandidateDto {
    pub pid: i32,
    pub name: String,
    pub bundle_id: Option<String>,
    pub footprint_bytes: u64,
    pub icon: Option<String>,
    pub has_visible_window: bool,
    pub is_hidden: bool,
    pub recommended: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuitOutcomeDto {
    pub pid: i32,
    pub name: String,
    pub footprint_bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuitResultDto {
    pub closed: Vec<QuitOutcomeDto>,
    pub refused: Vec<QuitOutcomeDto>,
    pub freed_bytes: u64,
}

impl From<&CleanCandidateEntity> for CleanCandidateDto {
    fn from(entity: &CleanCandidateEntity) -> Self {
        Self {
            pid: entity.pid,
            name: entity.name.clone(),
            bundle_id: entity.bundle_id.clone(),
            footprint_bytes: entity.footprint_bytes,
            icon: entity.icon_png_base64.clone(),
            has_visible_window: entity.has_visible_window,
            is_hidden: entity.is_hidden,
            recommended: entity.recommended,
        }
    }
}

impl From<&QuitOutcomeEntity> for QuitOutcomeDto {
    fn from(entity: &QuitOutcomeEntity) -> Self {
        Self { pid: entity.pid, name: entity.name.clone(), footprint_bytes: entity.footprint_bytes }
    }
}

impl From<&QuitResultEntity> for QuitResultDto {
    fn from(entity: &QuitResultEntity) -> Self {
        Self {
            closed: entity.closed.iter().map(QuitOutcomeDto::from).collect(),
            refused: entity.refused.iter().map(QuitOutcomeDto::from).collect(),
            freed_bytes: entity.freed_bytes,
        }
    }
}
