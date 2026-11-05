use std::time::Duration;

pub struct SamplingConfig {
    pub visible_interval: Duration,
    pub hidden_interval: Duration,
}

pub struct ThresholdsConfig {
    pub temperature_warning: f64,
    pub temperature_critical: f64,
    pub disk_warning_ratio: f64,
    pub disk_critical_ratio: f64,
    pub sensor_min_valid: f64,
    pub sensor_max_valid: f64,
}

pub struct CleanerConfig {
    pub recommend_min_bytes: u64,
    pub quit_wait: Duration,
    pub quit_poll: Duration,
    pub always_protected: &'static [&'static str],
}

pub struct Config {
    pub sampling: SamplingConfig,
    pub thresholds: ThresholdsConfig,
    pub cleaner: CleanerConfig,
    pub top_apps_limit: usize,
    pub breakdown_process_limit: usize,
    pub icon_side: f64,
    pub settings_store_file: &'static str,
    pub disk_path: &'static str,
    pub popover_blur_settle: Duration,
}

pub const CONFIG: Config = Config {
    sampling: SamplingConfig {
        visible_interval: Duration::from_secs(2),
        hidden_interval: Duration::from_secs(5),
    },
    thresholds: ThresholdsConfig {
        temperature_warning: 60.0,
        temperature_critical: 80.0,
        disk_warning_ratio: 0.70,
        disk_critical_ratio: 0.85,
        sensor_min_valid: 1.0,
        sensor_max_valid: 150.0,
    },
    cleaner: CleanerConfig {
        recommend_min_bytes: 100 * 1024 * 1024,
        quit_wait: Duration::from_secs(5),
        quit_poll: Duration::from_millis(250),
        always_protected: &[
            "com.apple.finder",
            "com.apple.dock",
            "com.apple.systemuiserver",
            "com.apple.loginwindow",
            "com.apple.controlcenter",
            "dev.joao.vitalis",
        ],
    },
    top_apps_limit: 10,
    breakdown_process_limit: 12,
    icon_side: 64.0,
    settings_store_file: "settings.json",
    disk_path: "/",
    popover_blur_settle: Duration::from_millis(300),
};
