use serde::Serialize;

use crate::app::temperature::domain::temperature_entity::TemperatureGroupEntity;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemperatureGroupDto {
    pub kind: &'static str,
    pub label: &'static str,
    pub max_celsius: f64,
    pub average_celsius: f64,
    pub sensor_count: usize,
    pub level: &'static str,
}

impl From<&TemperatureGroupEntity> for TemperatureGroupDto {
    fn from(entity: &TemperatureGroupEntity) -> Self {
        Self {
            kind: entity.kind.key(),
            label: entity.kind.label(),
            max_celsius: entity.max_celsius,
            average_celsius: entity.average_celsius,
            sensor_count: entity.sensor_count,
            level: entity.level.key(),
        }
    }
}
