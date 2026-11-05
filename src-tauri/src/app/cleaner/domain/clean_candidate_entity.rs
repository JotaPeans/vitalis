#[derive(Debug, Clone)]
pub struct CleanCandidateEntity {
    pub pid: i32,
    pub name: String,
    pub bundle_id: Option<String>,
    pub footprint_bytes: u64,
    pub icon_png_base64: Option<String>,
    pub has_visible_window: bool,
    pub is_hidden: bool,
    pub recommended: bool,
}

#[derive(Debug, Clone)]
pub struct QuitOutcomeEntity {
    pub pid: i32,
    pub name: String,
    pub footprint_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct QuitResultEntity {
    pub closed: Vec<QuitOutcomeEntity>,
    pub refused: Vec<QuitOutcomeEntity>,
    pub freed_bytes: u64,
}
