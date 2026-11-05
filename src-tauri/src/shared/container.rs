use std::sync::Arc;

use tauri::{AppHandle, Wry};

use crate::app::cleaner::domain::cleaner_service::CleanerService;
use crate::app::cleaner::infrastructure::cg_window_repository::CgWindowRepository;
use crate::app::cleaner::infrastructure::nsworkspace_terminator_repository::NsWorkspaceTerminatorRepository;
use crate::app::disk::domain::disk_service::DiskService;
use crate::app::disk::infrastructure::nsurl_disk_repository::NsUrlDiskRepository;
use crate::app::memory::domain::memory_service::MemoryService;
use crate::app::memory::infrastructure::mach_memory_repository::MachMemoryRepository;
use crate::app::processes::domain::processes_service::ProcessesService;
use crate::app::processes::infrastructure::libproc_process_repository::LibprocProcessRepository;
use crate::app::processes::infrastructure::nsworkspace_app_repository::NsWorkspaceAppRepository;
use crate::app::settings::domain::settings_service::SettingsService;
use crate::app::settings::infrastructure::store_settings_repository::StoreSettingsRepository;
use crate::app::temperature::domain::temperature_service::TemperatureService;
use crate::app::temperature::infrastructure::hid_temperature_repository::HidTemperatureRepository;
use crate::app::temperature::infrastructure::smc_temperature_repository::SmcTemperatureRepository;

#[derive(Clone)]
pub struct Container {
    pub temperature: Arc<TemperatureService>,
    pub memory: Arc<MemoryService>,
    pub disk: Arc<DiskService>,
    pub processes: Arc<ProcessesService>,
    pub settings: Arc<SettingsService>,
    pub cleaner: Arc<CleanerService>,
}

impl Container {
    pub fn build(app: AppHandle<Wry>) -> Self {
        let app_repository = Arc::new(NsWorkspaceAppRepository::default());
        let memory = Arc::new(MemoryService::new(Arc::new(MachMemoryRepository)));
        let processes = Arc::new(ProcessesService::new(
            Arc::new(LibprocProcessRepository),
            app_repository.clone(),
        ));
        let settings = Arc::new(SettingsService::new(
            Arc::new(StoreSettingsRepository::new(app)),
            app_repository,
        ));
        let cleaner = Arc::new(CleanerService::new(
            processes.clone(),
            settings.clone(),
            memory.clone(),
            Arc::new(CgWindowRepository),
            Arc::new(NsWorkspaceTerminatorRepository),
            std::process::id() as i32,
        ));
        Self {
            temperature: Arc::new(TemperatureService::new(vec![
                Arc::new(SmcTemperatureRepository::new()),
                Arc::new(HidTemperatureRepository::new()),
            ])),
            memory,
            disk: Arc::new(DiskService::new(Arc::new(NsUrlDiskRepository))),
            processes,
            settings,
            cleaner,
        }
    }
}
