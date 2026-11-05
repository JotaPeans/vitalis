#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TemperatureKind {
    Cpu,
    Gpu,
    Ssd,
    Battery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemperatureLevel {
    Normal,
    Warning,
    Critical,
}

#[derive(Debug, Clone)]
pub struct SensorReadingEntity {
    pub name: String,
    pub celsius: f64,
}

#[derive(Debug, Clone)]
pub struct TemperatureGroupEntity {
    pub kind: TemperatureKind,
    pub max_celsius: f64,
    pub average_celsius: f64,
    pub sensor_count: usize,
    pub level: TemperatureLevel,
}

pub const CPU_CORE_KEY_PREFIXES: &[&str] = &["Tp", "Te"];
pub const GPU_KEY_PREFIXES: &[&str] = &["Tg"];

const SMC_KEY_LENGTH: usize = 4;

impl TemperatureKind {
    pub fn from_sensor_name(name: &str) -> Option<Self> {
        let is_smc_key = |prefixes: &[&str]| name.len() == SMC_KEY_LENGTH && prefixes.iter().any(|p| name.starts_with(p));
        let lower = name.to_lowercase();
        match () {
            _ if is_smc_key(CPU_CORE_KEY_PREFIXES) => Some(Self::Cpu),
            _ if is_smc_key(GPU_KEY_PREFIXES) => Some(Self::Gpu),
            _ if lower.contains("nand") => Some(Self::Ssd),
            _ if lower.contains("battery") => Some(Self::Battery),
            _ => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Cpu => "CPU",
            Self::Gpu => "GPU",
            Self::Ssd => "SSD",
            Self::Battery => "Bateria",
        }
    }

    pub fn key(&self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::Gpu => "gpu",
            Self::Ssd => "ssd",
            Self::Battery => "battery",
        }
    }
}

impl TemperatureLevel {
    pub fn key(&self) -> &'static str {
        match self {
            Self::Normal => "ok",
            Self::Warning => "warning",
            Self::Critical => "critical",
        }
    }
}
