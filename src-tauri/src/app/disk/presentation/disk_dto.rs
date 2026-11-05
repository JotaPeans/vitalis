use serde::Serialize;

use crate::app::disk::domain::disk_entity::DiskEntity;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskDto {
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub used_bytes: u64,
    pub used_ratio: f64,
    pub level: &'static str,
}

impl From<&DiskEntity> for DiskDto {
    fn from(entity: &DiskEntity) -> Self {
        Self {
            total_bytes: entity.total_bytes,
            available_bytes: entity.available_bytes,
            used_bytes: entity.used_bytes,
            used_ratio: entity.used_ratio,
            level: entity.level.key(),
        }
    }
}
