#[derive(Debug, Clone, Default)]
pub struct SettingsEntity {
    pub protected_apps: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ProtectableAppEntity {
    pub pid: i32,
    pub bundle_id: String,
    pub name: String,
    pub icon_png_base64: Option<String>,
    pub is_protected: bool,
}

impl SettingsEntity {
    pub fn is_protected(&self, bundle_id: &str) -> bool {
        self.protected_apps.iter().any(|b| b == bundle_id)
    }
}
