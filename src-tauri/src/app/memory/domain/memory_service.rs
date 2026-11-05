use std::sync::Arc;

use crate::shared::errors::AppResult;

use super::memory_entity::{MemoryEntity, MemoryPressure, RawMemoryStatsEntity};
use super::memory_interfaces::IMemoryRepository;

pub struct MemoryService {
    repository: Arc<dyn IMemoryRepository>,
}

impl MemoryService {
    pub fn new(repository: Arc<dyn IMemoryRepository>) -> Self {
        Self { repository }
    }

    pub fn snapshot(&self) -> AppResult<MemoryEntity> {
        let stats = self.repository.read_stats()?;
        Ok(build_memory(&stats))
    }
}

fn build_memory(stats: &RawMemoryStatsEntity) -> MemoryEntity {
    let page = stats.page_size;
    let app_bytes = stats.internal_pages.saturating_sub(stats.purgeable_pages) * page;
    let wired_bytes = stats.wired_pages * page;
    let compressed_bytes = stats.compressor_pages * page;
    let cached_bytes = (stats.external_pages + stats.purgeable_pages) * page;
    let accounted = app_bytes + wired_bytes + compressed_bytes + cached_bytes;
    MemoryEntity {
        total_bytes: stats.total_bytes,
        app_bytes,
        wired_bytes,
        compressed_bytes,
        cached_bytes,
        free_bytes: stats.total_bytes.saturating_sub(accounted),
        swap_used_bytes: stats.swap_used_bytes,
        swap_total_bytes: stats.swap_total_bytes,
        pressure: MemoryPressure::from_kernel_level(stats.pressure_level),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeRepository(RawMemoryStatsEntity);

    impl IMemoryRepository for FakeRepository {
        fn read_stats(&self) -> AppResult<RawMemoryStatsEntity> {
            Ok(self.0.clone())
        }
    }

    #[test]
    fn computes_activity_monitor_breakdown() {
        let service = MemoryService::new(Arc::new(FakeRepository(RawMemoryStatsEntity {
            page_size: 10,
            total_bytes: 1_000,
            internal_pages: 30,
            external_pages: 15,
            purgeable_pages: 5,
            wired_pages: 10,
            compressor_pages: 5,
            pressure_level: 2,
            ..Default::default()
        })));
        let memory = service.snapshot().unwrap();
        assert_eq!(memory.app_bytes, 250);
        assert_eq!(memory.wired_bytes, 100);
        assert_eq!(memory.compressed_bytes, 50);
        assert_eq!(memory.cached_bytes, 200);
        assert_eq!(memory.free_bytes, 400);
        assert_eq!(memory.used_bytes(), 400);
        assert_eq!(memory.pressure, MemoryPressure::Warning);
    }
}
