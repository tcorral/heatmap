#[derive(Clone, Debug)]
pub struct AppConfig {
    project_keys: Vec<String>,
}

impl AppConfig {
    pub fn with_project_keys(keys: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            project_keys: keys.into_iter().map(Into::into).collect(),
        }
    }

    pub fn is_valid_project_key(&self, key: &str) -> bool {
        self.project_keys.iter().any(|candidate| candidate == key)
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::with_project_keys([super::TEST_PROJECT_KEY])
    }
}
