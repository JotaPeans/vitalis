use crate::shared::errors::AppResult;

use super::memory_entity::RawMemoryStatsEntity;

pub trait IMemoryRepository: Send + Sync {
    fn read_stats(&self) -> AppResult<RawMemoryStatsEntity>;
}
