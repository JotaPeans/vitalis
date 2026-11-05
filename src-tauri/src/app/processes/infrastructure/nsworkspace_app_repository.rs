use std::collections::HashMap;
use std::sync::Mutex;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use crate::app::processes::domain::app_usage_entity::RunningAppEntity;
use crate::app::processes::domain::processes_interfaces::IAppRepository;
use crate::lib::config::CONFIG;
use crate::lib::ffi::appkit;

const REGULAR_ACTIVATION_POLICY: isize = 0;

#[derive(Default)]
pub struct NsWorkspaceAppRepository {
    icon_cache: Mutex<HashMap<String, Option<String>>>,
}

impl IAppRepository for NsWorkspaceAppRepository {
    fn running_apps(&self) -> Vec<RunningAppEntity> {
        appkit::running_apps()
            .into_iter()
            .map(|raw| RunningAppEntity {
                pid: raw.pid,
                name: raw.name,
                bundle_id: raw.bundle_id,
                bundle_path: raw.bundle_path,
                is_regular: raw.activation_policy == REGULAR_ACTIVATION_POLICY,
                is_active: raw.is_active,
                is_hidden: raw.is_hidden,
            })
            .collect()
    }

    fn icon_base64(&self, pid: i32, cache_key: &str) -> Option<String> {
        let Ok(mut cache) = self.icon_cache.lock() else {
            return None;
        };
        cache
            .entry(cache_key.to_string())
            .or_insert_with(|| appkit::icon_png(pid, CONFIG.icon_side).map(|png| STANDARD.encode(png)))
            .clone()
    }
}
