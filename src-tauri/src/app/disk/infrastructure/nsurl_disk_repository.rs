use crate::app::disk::domain::disk_entity::RawDiskCapacityEntity;
use crate::app::disk::domain::disk_interfaces::IDiskRepository;
use crate::lib::ffi::appkit;
use crate::shared::errors::{AppError, AppResult};

pub struct NsUrlDiskRepository;

impl IDiskRepository for NsUrlDiskRepository {
    fn read_capacity(&self, path: &str) -> AppResult<RawDiskCapacityEntity> {
        let capacity = appkit::volume_capacity(path)
            .ok_or_else(|| AppError::Platform("Falha ao ler capacidade do disco".into()))?;
        Ok(RawDiskCapacityEntity {
            total_bytes: capacity.total,
            available_bytes: capacity.available_for_important_usage,
        })
    }
}
