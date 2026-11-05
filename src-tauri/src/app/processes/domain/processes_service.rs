use std::collections::HashMap;
use std::sync::Arc;

use crate::lib::config::CONFIG;

use super::app_usage_entity::{AppUsageEntity, ProcessSampleEntity, RunningAppEntity};
use super::process_breakdown::build_breakdown;
use super::processes_interfaces::{IAppRepository, IProcessRepository};

pub struct ProcessesService {
    process_repository: Arc<dyn IProcessRepository>,
    app_repository: Arc<dyn IAppRepository>,
}

impl ProcessesService {
    pub fn new(
        process_repository: Arc<dyn IProcessRepository>,
        app_repository: Arc<dyn IAppRepository>,
    ) -> Self {
        Self { process_repository, app_repository }
    }

    pub fn usage_by_app(&self) -> Vec<AppUsageEntity> {
        let samples = self.process_repository.list_samples();
        let apps = self.app_repository.running_apps();
        aggregate(&samples, &apps)
    }

    pub fn top_apps(&self, limit: usize) -> Vec<AppUsageEntity> {
        let mut usage = self.usage_by_app();
        usage.truncate(limit);
        self.with_icons(usage)
            .into_iter()
            .map(|mut entry| {
                let breakdown = build_breakdown(&entry, CONFIG.breakdown_process_limit, |pid| {
                    self.process_repository.describe(pid)
                });
                entry.breakdown = Some(breakdown);
                entry
            })
            .collect()
    }

    pub fn with_icons(&self, usage: Vec<AppUsageEntity>) -> Vec<AppUsageEntity> {
        usage
            .into_iter()
            .map(|mut entry| {
                let cache_key = entry.bundle_id.clone().unwrap_or_else(|| entry.name.clone());
                entry.icon_png_base64 = entry
                    .app
                    .as_ref()
                    .and_then(|_| self.app_repository.icon_base64(entry.pid, &cache_key));
                entry
            })
            .collect()
    }

    pub fn is_alive(&self, pid: i32) -> bool {
        self.process_repository.is_alive(pid)
    }
}

fn aggregate(samples: &[ProcessSampleEntity], apps: &[RunningAppEntity]) -> Vec<AppUsageEntity> {
    let apps_by_pid: HashMap<i32, &RunningAppEntity> = apps.iter().map(|a| (a.pid, a)).collect();
    let names_by_pid: HashMap<i32, &str> = samples.iter().map(|s| (s.pid, s.name.as_str())).collect();
    let mut groups: HashMap<i32, AppUsageEntity> = HashMap::new();
    for sample in samples {
        let owner = sample.responsible_pid;
        let entry = groups.entry(owner).or_insert_with(|| {
            new_usage(owner, apps_by_pid.get(&owner).copied(), names_by_pid.get(&owner).copied(), sample)
        });
        entry.footprint_bytes += sample.footprint_bytes;
        entry.process_count += 1;
        entry.members.push(sample.clone());
    }
    let mut usage: Vec<AppUsageEntity> = groups.into_values().collect();
    usage.sort_by(|a, b| b.footprint_bytes.cmp(&a.footprint_bytes));
    usage
}

fn new_usage(
    owner: i32,
    app: Option<&RunningAppEntity>,
    owner_name: Option<&str>,
    sample: &ProcessSampleEntity,
) -> AppUsageEntity {
    let name = app
        .map(|a| a.name.clone())
        .filter(|n| !n.is_empty())
        .or_else(|| owner_name.map(str::to_string))
        .unwrap_or_else(|| sample.name.clone());
    AppUsageEntity {
        pid: owner,
        name,
        bundle_id: app.and_then(|a| a.bundle_id.clone()),
        footprint_bytes: 0,
        process_count: 0,
        icon_png_base64: None,
        app: app.cloned(),
        members: Vec::new(),
        breakdown: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::processes::domain::app_usage_entity::ProcessDetailEntity;

    struct FakeProcesses(Vec<ProcessSampleEntity>);
    struct FakeApps(Vec<RunningAppEntity>);

    impl IProcessRepository for FakeProcesses {
        fn list_samples(&self) -> Vec<ProcessSampleEntity> {
            self.0.clone()
        }
        fn describe(&self, _pid: i32) -> ProcessDetailEntity {
            ProcessDetailEntity::default()
        }
        fn is_alive(&self, _pid: i32) -> bool {
            true
        }
    }

    impl IAppRepository for FakeApps {
        fn running_apps(&self) -> Vec<RunningAppEntity> {
            self.0.clone()
        }
        fn icon_base64(&self, _pid: i32, _cache_key: &str) -> Option<String> {
            None
        }
    }

    fn sample(pid: i32, responsible_pid: i32, name: &str, footprint_bytes: u64) -> ProcessSampleEntity {
        ProcessSampleEntity { pid, responsible_pid, name: name.into(), footprint_bytes }
    }

    #[test]
    fn aggregates_helpers_into_responsible_app() {
        let service = ProcessesService::new(
            Arc::new(FakeProcesses(vec![
                sample(10, 10, "Google Chrome", 300),
                sample(11, 10, "Google Chrome Helper", 500),
                sample(20, 20, "mds", 100),
            ])),
            Arc::new(FakeApps(vec![RunningAppEntity {
                pid: 10,
                name: "Chrome".into(),
                bundle_id: Some("com.google.Chrome".into()),
                bundle_path: Some("/Applications/Google Chrome.app".into()),
                is_regular: true,
                is_active: false,
                is_hidden: false,
            }])),
        );
        let usage = service.usage_by_app();
        assert_eq!(usage[0].name, "Chrome");
        assert_eq!(usage[0].footprint_bytes, 800);
        assert_eq!(usage[0].process_count, 2);
        assert_eq!(usage[1].name, "mds");
    }
}
