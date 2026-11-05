use serde::Serialize;

use crate::app::disk::presentation::disk_dto::DiskDto;
use crate::app::memory::presentation::memory_dto::MemoryDto;
use crate::app::processes::presentation::app_usage_dto::AppUsageDto;
use crate::app::temperature::presentation::temperature_dto::TemperatureGroupDto;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VitalsDto {
    pub temperatures: Vec<TemperatureGroupDto>,
    pub memory: Option<MemoryDto>,
    pub disk: Option<DiskDto>,
    pub top_apps: Vec<AppUsageDto>,
}
