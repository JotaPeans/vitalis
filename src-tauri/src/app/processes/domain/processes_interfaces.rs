use super::app_usage_entity::{ProcessDetailEntity, ProcessSampleEntity, RunningAppEntity};

pub trait IProcessRepository: Send + Sync {
    fn list_samples(&self) -> Vec<ProcessSampleEntity>;
    fn describe(&self, pid: i32) -> ProcessDetailEntity;
    fn is_alive(&self, pid: i32) -> bool;
}

pub trait IAppRepository: Send + Sync {
    fn running_apps(&self) -> Vec<RunningAppEntity>;
    fn icon_base64(&self, pid: i32, cache_key: &str) -> Option<String>;
}
