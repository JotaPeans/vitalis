use crate::shared::errors::AppResult;

use super::disk_entity::RawDiskCapacityEntity;

pub trait IDiskRepository: Send + Sync {
    fn read_capacity(&self, path: &str) -> AppResult<RawDiskCapacityEntity>;
}
