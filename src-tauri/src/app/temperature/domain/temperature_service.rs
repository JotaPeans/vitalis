use std::collections::BTreeMap;
use std::sync::Arc;

use crate::lib::config::CONFIG;
use crate::shared::errors::{AppError, AppResult};

use super::temperature_entity::{
    SensorReadingEntity, TemperatureGroupEntity, TemperatureKind, TemperatureLevel,
};
use super::temperature_interfaces::ITemperatureRepository;

pub struct TemperatureService {
    repositories: Vec<Arc<dyn ITemperatureRepository>>,
}

impl TemperatureService {
    pub fn new(repositories: Vec<Arc<dyn ITemperatureRepository>>) -> Self {
        Self { repositories }
    }

    pub fn list_groups(&self) -> AppResult<Vec<TemperatureGroupEntity>> {
        let results: Vec<AppResult<Vec<SensorReadingEntity>>> =
            self.repositories.iter().map(|r| r.read_sensors()).collect();
        if results.iter().all(Result::is_err) {
            return Err(AppError::Platform("Sensores de temperatura indisponíveis".into()));
        }
        let readings: Vec<SensorReadingEntity> = results.into_iter().flatten().flatten().collect();
        Ok(group_readings(&readings))
    }

    pub fn hottest(groups: &[TemperatureGroupEntity], kind: TemperatureKind) -> Option<f64> {
        groups.iter().find(|g| g.kind == kind).map(|g| g.max_celsius)
    }
}

fn group_readings(readings: &[SensorReadingEntity]) -> Vec<TemperatureGroupEntity> {
    let thresholds = &CONFIG.thresholds;
    let mut buckets: BTreeMap<TemperatureKind, Vec<f64>> = BTreeMap::new();
    readings
        .iter()
        .filter(|r| (thresholds.sensor_min_valid..=thresholds.sensor_max_valid).contains(&r.celsius))
        .filter_map(|r| TemperatureKind::from_sensor_name(&r.name).map(|kind| (kind, r.celsius)))
        .for_each(|(kind, celsius)| buckets.entry(kind).or_default().push(celsius));
    buckets
        .into_iter()
        .map(|(kind, values)| build_group(kind, &values))
        .collect()
}

fn build_group(kind: TemperatureKind, values: &[f64]) -> TemperatureGroupEntity {
    let max_celsius = values.iter().copied().fold(f64::MIN, f64::max);
    let average_celsius = values.iter().sum::<f64>() / values.len() as f64;
    TemperatureGroupEntity {
        kind,
        max_celsius,
        average_celsius,
        sensor_count: values.len(),
        level: level_for(max_celsius),
    }
}

fn level_for(celsius: f64) -> TemperatureLevel {
    let thresholds = &CONFIG.thresholds;
    match celsius {
        c if c >= thresholds.temperature_critical => TemperatureLevel::Critical,
        c if c >= thresholds.temperature_warning => TemperatureLevel::Warning,
        _ => TemperatureLevel::Normal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeRepository(Vec<(&'static str, f64)>);

    impl ITemperatureRepository for FakeRepository {
        fn read_sensors(&self) -> AppResult<Vec<SensorReadingEntity>> {
            Ok(self
                .0
                .iter()
                .map(|(name, celsius)| SensorReadingEntity { name: name.to_string(), celsius: *celsius })
                .collect())
        }
    }

    #[test]
    fn merges_sources_and_ignores_pmu_and_unrelated_sensors() {
        let service = TemperatureService::new(vec![
            Arc::new(FakeRepository(vec![("Tp0C", 50.0), ("Te04", 70.0), ("Tg0K", 45.0), ("TVmS", 41.0), ("Tp01", -1.0)])),
            Arc::new(FakeRepository(vec![
                ("PMU tdie1", 90.0),
                ("NAND CH0 temp", 40.0),
                ("gas gauge battery", 30.0),
            ])),
        ]);
        let groups = service.list_groups().unwrap();
        let cpu = groups.iter().find(|g| g.kind == TemperatureKind::Cpu).unwrap();
        assert_eq!(groups.len(), 4);
        assert_eq!(cpu.sensor_count, 2);
        assert_eq!(cpu.max_celsius, 70.0);
        assert_eq!(cpu.average_celsius, 60.0);
        assert_eq!(cpu.level, TemperatureLevel::Warning);
    }
}
