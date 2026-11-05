use std::sync::Arc;

use crate::lib::config::CONFIG;
use crate::shared::errors::AppResult;

use super::disk_entity::{DiskEntity, DiskLevel, RawDiskCapacityEntity};
use super::disk_interfaces::IDiskRepository;

pub struct DiskService {
    repository: Arc<dyn IDiskRepository>,
}

impl DiskService {
    pub fn new(repository: Arc<dyn IDiskRepository>) -> Self {
        Self { repository }
    }

    pub fn usage(&self) -> AppResult<DiskEntity> {
        let capacity = self.repository.read_capacity(CONFIG.disk_path)?;
        Ok(build_disk(&capacity))
    }
}

fn build_disk(capacity: &RawDiskCapacityEntity) -> DiskEntity {
    let available_bytes = capacity.available_bytes.min(capacity.total_bytes);
    let used_bytes = capacity.total_bytes - available_bytes;
    let used_ratio = match capacity.total_bytes {
        0 => 0.0,
        total => used_bytes as f64 / total as f64,
    };
    DiskEntity {
        total_bytes: capacity.total_bytes,
        available_bytes,
        used_bytes,
        used_ratio,
        level: level_for(used_ratio),
    }
}

fn level_for(ratio: f64) -> DiskLevel {
    let thresholds = &CONFIG.thresholds;
    match ratio {
        r if r >= thresholds.disk_critical_ratio => DiskLevel::Critical,
        r if r >= thresholds.disk_warning_ratio => DiskLevel::Warning,
        _ => DiskLevel::Ok,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeRepository(u64, u64);

    impl IDiskRepository for FakeRepository {
        fn read_capacity(&self, _path: &str) -> AppResult<RawDiskCapacityEntity> {
            Ok(RawDiskCapacityEntity { total_bytes: self.0, available_bytes: self.1 })
        }
    }

    #[test]
    fn classifies_disk_levels() {
        let level = |total, available| {
            DiskService::new(Arc::new(FakeRepository(total, available))).usage().unwrap().level
        };
        assert_eq!(level(100, 50), DiskLevel::Ok);
        assert_eq!(level(100, 20), DiskLevel::Warning);
        assert_eq!(level(100, 10), DiskLevel::Critical);
    }
}
