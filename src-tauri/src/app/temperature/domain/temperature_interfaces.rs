use crate::shared::errors::AppResult;

use super::temperature_entity::SensorReadingEntity;

pub trait ITemperatureRepository: Send + Sync {
    fn read_sensors(&self) -> AppResult<Vec<SensorReadingEntity>>;
}
