use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;

use crate::app::memory::domain::memory_service::MemoryService;
use crate::app::processes::domain::app_usage_entity::AppUsageEntity;
use crate::app::processes::domain::processes_service::ProcessesService;
use crate::app::settings::domain::settings_entity::SettingsEntity;
use crate::app::settings::domain::settings_service::SettingsService;
use crate::lib::config::CONFIG;
use crate::shared::errors::{AppError, AppResult};

use super::clean_candidate_entity::{CleanCandidateEntity, QuitOutcomeEntity, QuitResultEntity};
use super::cleaner_interfaces::{IAppTerminator, IWindowRepository};

pub struct CleanerService {
    processes: Arc<ProcessesService>,
    settings: Arc<SettingsService>,
    memory: Arc<MemoryService>,
    windows: Arc<dyn IWindowRepository>,
    terminator: Arc<dyn IAppTerminator>,
    self_pid: i32,
}

impl CleanerService {
    pub fn new(
        processes: Arc<ProcessesService>,
        settings: Arc<SettingsService>,
        memory: Arc<MemoryService>,
        windows: Arc<dyn IWindowRepository>,
        terminator: Arc<dyn IAppTerminator>,
        self_pid: i32,
    ) -> Self {
        Self { processes, settings, memory, windows, terminator, self_pid }
    }

    pub fn candidates(&self) -> AppResult<Vec<CleanCandidateEntity>> {
        let settings = self.settings.get()?;
        let visible = self.windows.pids_with_visible_windows();
        let eligible: Vec<AppUsageEntity> = self
            .processes
            .usage_by_app()
            .into_iter()
            .filter(|usage| self.is_eligible(usage, &settings))
            .collect();
        Ok(self
            .processes
            .with_icons(eligible)
            .into_iter()
            .map(|usage| to_candidate(usage, &visible))
            .collect())
    }

    pub fn quit(&self, pids: &[i32], force: bool) -> AppResult<QuitResultEntity> {
        let targets = self.resolve_targets(pids)?;
        let used_before = self.memory.snapshot()?.used_bytes();
        targets.iter().for_each(|t| {
            self.terminator.terminate(t.pid, force);
        });
        self.wait_for_exit(&targets);
        let used_after = self.memory.snapshot()?.used_bytes();
        let (closed, refused) = targets
            .into_iter()
            .partition(|t| !self.processes.is_alive(t.pid));
        Ok(QuitResultEntity {
            closed,
            refused,
            freed_bytes: used_before.saturating_sub(used_after),
        })
    }

    fn is_eligible(&self, usage: &AppUsageEntity, settings: &SettingsEntity) -> bool {
        let Some(app) = usage.app.as_ref().filter(|a| a.is_regular) else {
            return false;
        };
        let bundle_id = app.bundle_id.as_deref().unwrap_or_default();
        !app.is_active
            && app.pid != self.self_pid
            && !CONFIG.cleaner.always_protected.contains(&bundle_id)
            && !settings.is_protected(bundle_id)
    }

    fn resolve_targets(&self, pids: &[i32]) -> AppResult<Vec<QuitOutcomeEntity>> {
        if pids.is_empty() {
            return Err(AppError::BadRequest("Selecione ao menos um app".into()));
        }
        let candidates: HashMap<i32, CleanCandidateEntity> =
            self.candidates()?.into_iter().map(|c| (c.pid, c)).collect();
        let unique: HashSet<i32> = pids.iter().copied().collect();
        unique
            .into_iter()
            .map(|pid| {
                candidates
                    .get(&pid)
                    .map(|c| QuitOutcomeEntity { pid, name: c.name.clone(), footprint_bytes: c.footprint_bytes })
                    .ok_or_else(|| AppError::BadRequest(format!("O app {pid} não pode ser encerrado")))
            })
            .collect()
    }

    fn wait_for_exit(&self, targets: &[QuitOutcomeEntity]) {
        let started = Instant::now();
        while started.elapsed() < CONFIG.cleaner.quit_wait {
            if targets.iter().all(|t| !self.processes.is_alive(t.pid)) {
                return;
            }
            std::thread::sleep(CONFIG.cleaner.quit_poll);
        }
    }
}

fn to_candidate(usage: AppUsageEntity, visible: &HashSet<i32>) -> CleanCandidateEntity {
    let is_hidden = usage.app.as_ref().is_some_and(|a| a.is_hidden);
    let has_visible_window = visible.contains(&usage.pid);
    CleanCandidateEntity {
        recommended: (is_hidden || !has_visible_window)
            && usage.footprint_bytes >= CONFIG.cleaner.recommend_min_bytes,
        pid: usage.pid,
        name: usage.name,
        bundle_id: usage.bundle_id,
        footprint_bytes: usage.footprint_bytes,
        icon_png_base64: usage.icon_png_base64,
        has_visible_window,
        is_hidden,
    }
}
