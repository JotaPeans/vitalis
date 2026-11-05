#[derive(Debug, Clone)]
pub struct ProcessSampleEntity {
    pub pid: i32,
    pub responsible_pid: i32,
    pub name: String,
    pub footprint_bytes: u64,
}

#[derive(Debug, Clone, Default)]
pub struct ProcessDetailEntity {
    pub path: Option<String>,
    pub arguments: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RunningAppEntity {
    pub pid: i32,
    pub name: String,
    pub bundle_id: Option<String>,
    pub bundle_path: Option<String>,
    pub is_regular: bool,
    pub is_active: bool,
    pub is_hidden: bool,
}

#[derive(Debug, Clone)]
pub struct ProcessBreakdownEntity {
    pub pid: i32,
    pub label: String,
    pub footprint_bytes: u64,
    pub is_own: bool,
}

#[derive(Debug, Clone)]
pub struct AppBreakdownEntity {
    pub own_bytes: u64,
    pub launched_bytes: u64,
    pub processes: Vec<ProcessBreakdownEntity>,
}

#[derive(Debug, Clone)]
pub struct AppUsageEntity {
    pub pid: i32,
    pub name: String,
    pub bundle_id: Option<String>,
    pub footprint_bytes: u64,
    pub process_count: usize,
    pub icon_png_base64: Option<String>,
    pub app: Option<RunningAppEntity>,
    pub members: Vec<ProcessSampleEntity>,
    pub breakdown: Option<AppBreakdownEntity>,
}
