use std::sync::Mutex;

use crate::app::temperature::domain::temperature_entity::{
    SensorReadingEntity, CPU_CORE_KEY_PREFIXES, GPU_KEY_PREFIXES,
};
use crate::app::temperature::domain::temperature_interfaces::ITemperatureRepository;
use crate::lib::ffi::smc::SmcTemperatureClient;
use crate::shared::errors::{AppError, AppResult};

pub struct SmcTemperatureRepository {
    client: Option<Mutex<SmcTemperatureClient>>,
}

impl SmcTemperatureRepository {
    pub fn new() -> Self {
        let prefixes: Vec<&str> = CPU_CORE_KEY_PREFIXES.iter().chain(GPU_KEY_PREFIXES).copied().collect();
        Self {
            client: SmcTemperatureClient::new(&prefixes).map(Mutex::new),
        }
    }
}

impl ITemperatureRepository for SmcTemperatureRepository {
    fn read_sensors(&self) -> AppResult<Vec<SensorReadingEntity>> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| AppError::Platform("SMC indisponível".into()))?;
        let client = client
            .lock()
            .map_err(|_| AppError::Platform("Leitor do SMC bloqueado".into()))?;
        Ok(client
            .read_all()
            .into_iter()
            .map(|raw| SensorReadingEntity { name: raw.key, celsius: raw.celsius })
            .collect())
    }
}
