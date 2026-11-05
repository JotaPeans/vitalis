#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiskLevel {
    Ok,
    Warning,
    Critical,
}

#[derive(Debug, Clone)]
pub struct RawDiskCapacityEntity {
    pub total_bytes: u64,
    pub available_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct DiskEntity {
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub used_bytes: u64,
    pub used_ratio: f64,
    pub level: DiskLevel,
}

impl DiskLevel {
    pub fn key(&self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Warning => "warning",
            Self::Critical => "critical",
        }
    }
}
