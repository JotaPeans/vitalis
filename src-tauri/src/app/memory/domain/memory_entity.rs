#[derive(Debug, Clone, Default)]
pub struct RawMemoryStatsEntity {
    pub page_size: u64,
    pub total_bytes: u64,
    pub internal_pages: u64,
    pub external_pages: u64,
    pub purgeable_pages: u64,
    pub wired_pages: u64,
    pub compressor_pages: u64,
    pub swap_used_bytes: u64,
    pub swap_total_bytes: u64,
    pub pressure_level: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPressure {
    Normal,
    Warning,
    Critical,
}

#[derive(Debug, Clone)]
pub struct MemoryEntity {
    pub total_bytes: u64,
    pub app_bytes: u64,
    pub wired_bytes: u64,
    pub compressed_bytes: u64,
    pub cached_bytes: u64,
    pub free_bytes: u64,
    pub swap_used_bytes: u64,
    pub swap_total_bytes: u64,
    pub pressure: MemoryPressure,
}

impl MemoryEntity {
    pub fn used_bytes(&self) -> u64 {
        self.app_bytes + self.wired_bytes + self.compressed_bytes
    }

    pub fn used_ratio(&self) -> f64 {
        match self.total_bytes {
            0 => 0.0,
            total => self.used_bytes() as f64 / total as f64,
        }
    }
}

impl MemoryPressure {
    pub fn from_kernel_level(level: i32) -> Self {
        match level {
            4 => Self::Critical,
            2 => Self::Warning,
            _ => Self::Normal,
        }
    }

    pub fn key(&self) -> &'static str {
        match self {
            Self::Normal => "ok",
            Self::Warning => "warning",
            Self::Critical => "critical",
        }
    }
}
