use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::sync::Arc;

use crate::app::disk::presentation::disk_dto::DiskDto;
use crate::app::memory::presentation::memory_dto::MemoryDto;
use crate::app::processes::presentation::app_usage_dto::AppUsageDto;
use crate::app::temperature::domain::temperature_entity::TemperatureKind;
use crate::app::temperature::domain::temperature_service::TemperatureService;
use crate::app::temperature::presentation::temperature_dto::TemperatureGroupDto;
use crate::lib::config::CONFIG;
use crate::shared::container::Container;
use crate::shared::popover_state::PopoverState;

use super::vitals_dto::VitalsDto;
use super::vitals_publisher::VitalsPublisher;

pub struct VitalsWorker {
    container: Container,
    publisher: VitalsPublisher,
    state: Arc<PopoverState>,
}

impl VitalsWorker {
    pub fn new(container: Container, publisher: VitalsPublisher, state: Arc<PopoverState>) -> Self {
        Self { container, publisher, state }
    }

    pub fn spawn(self, wake: Receiver<()>) {
        std::thread::Builder::new()
            .name("vitals-worker".into())
            .spawn(move || self.run(wake))
            .expect("failed to spawn vitals worker");
    }

    fn run(self, wake: Receiver<()>) {
        loop {
            let visible = self.state.is_visible();
            self.tick(visible);
            let interval = match visible {
                true => CONFIG.sampling.visible_interval,
                false => CONFIG.sampling.hidden_interval,
            };
            if let Err(RecvTimeoutError::Disconnected) = wake.recv_timeout(interval) {
                tracing::info!("vitals worker stopped");
                return;
            }
        }
    }

    fn tick(&self, visible: bool) {
        let temperatures = self.container.temperature.list_groups().unwrap_or_else(|error| {
            tracing::warn!(%error, "temperature sampling failed");
            Vec::new()
        });
        let memory = self.container.memory.snapshot().ok();
        self.publisher.update_tray(
            TemperatureService::hottest(&temperatures, TemperatureKind::Cpu),
            memory.as_ref().map(|m| m.used_ratio()),
        );
        if !visible {
            return;
        }
        self.publisher.publish(&VitalsDto {
            temperatures: temperatures.iter().map(TemperatureGroupDto::from).collect(),
            memory: memory.as_ref().map(MemoryDto::from),
            disk: self.container.disk.usage().ok().as_ref().map(DiskDto::from),
            top_apps: self
                .container
                .processes
                .top_apps(CONFIG.top_apps_limit)
                .iter()
                .map(AppUsageDto::from)
                .collect(),
        });
    }
}
