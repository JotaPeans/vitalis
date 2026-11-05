use std::sync::Mutex;

use crate::app::temperature::domain::temperature_entity::SensorReadingEntity;
use crate::app::temperature::domain::temperature_interfaces::ITemperatureRepository;
use crate::lib::ffi::iohid::HidTemperatureClient;
use crate::shared::errors::{AppError, AppResult};

pub struct HidTemperatureRepository {
    client: Option<Mutex<HidTemperatureClient>>,
}

impl HidTemperatureRepository {
    pub fn new() -> Self {
        Self {
            client: HidTemperatureClient::new().map(Mutex::new),
        }
    }
}

impl ITemperatureRepository for HidTemperatureRepository {
    fn read_sensors(&self) -> AppResult<Vec<SensorReadingEntity>> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| AppError::Platform("Sensores de temperatura indisponíveis".into()))?;
        let client = client
            .lock()
            .map_err(|_| AppError::Platform("Leitor de temperatura bloqueado".into()))?;
        Ok(client
            .read_all()
            .into_iter()
            .map(|raw| SensorReadingEntity { name: raw.name, celsius: raw.celsius })
            .collect())
    }
}
