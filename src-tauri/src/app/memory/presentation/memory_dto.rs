use serde::Serialize;

use crate::app::memory::domain::memory_entity::MemoryEntity;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryDto {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub used_ratio: f64,
    pub app_bytes: u64,
    pub wired_bytes: u64,
    pub compressed_bytes: u64,
    pub cached_bytes: u64,
    pub free_bytes: u64,
    pub swap_used_bytes: u64,
    pub swap_total_bytes: u64,
    pub pressure: &'static str,
}

impl From<&MemoryEntity> for MemoryDto {
    fn from(entity: &MemoryEntity) -> Self {
        Self {
            total_bytes: entity.total_bytes,
            used_bytes: entity.used_bytes(),
            used_ratio: entity.used_ratio(),
            app_bytes: entity.app_bytes,
            wired_bytes: entity.wired_bytes,
            compressed_bytes: entity.compressed_bytes,
            cached_bytes: entity.cached_bytes,
            free_bytes: entity.free_bytes,
            swap_used_bytes: entity.swap_used_bytes,
            swap_total_bytes: entity.swap_total_bytes,
            pressure: entity.pressure.key(),
        }
    }
}
