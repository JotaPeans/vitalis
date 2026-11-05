use serde::Serialize;

use crate::app::processes::domain::app_usage_entity::{AppUsageEntity, ProcessBreakdownEntity};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessBreakdownDto {
    pub pid: i32,
    pub label: String,
    pub footprint_bytes: u64,
    pub is_own: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppUsageDto {
    pub pid: i32,
    pub name: String,
    pub bundle_id: Option<String>,
    pub footprint_bytes: u64,
    pub own_bytes: u64,
    pub launched_bytes: u64,
    pub process_count: usize,
    pub processes: Vec<ProcessBreakdownDto>,
    pub icon: Option<String>,
}

impl From<&ProcessBreakdownEntity> for ProcessBreakdownDto {
    fn from(entity: &ProcessBreakdownEntity) -> Self {
        Self {
            pid: entity.pid,
            label: entity.label.clone(),
            footprint_bytes: entity.footprint_bytes,
            is_own: entity.is_own,
        }
    }
}

impl From<&AppUsageEntity> for AppUsageDto {
    fn from(entity: &AppUsageEntity) -> Self {
        let breakdown = entity.breakdown.as_ref();
        Self {
            pid: entity.pid,
            name: entity.name.clone(),
            bundle_id: entity.bundle_id.clone(),
            footprint_bytes: entity.footprint_bytes,
            own_bytes: breakdown.map_or(entity.footprint_bytes, |b| b.own_bytes),
            launched_bytes: breakdown.map_or(0, |b| b.launched_bytes),
            process_count: entity.process_count,
            processes: breakdown
                .map(|b| b.processes.iter().map(ProcessBreakdownDto::from).collect())
                .unwrap_or_default(),
            icon: entity.icon_png_base64.clone(),
        }
    }
}
