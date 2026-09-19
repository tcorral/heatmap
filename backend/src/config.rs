#[derive(Clone, Debug)]
pub struct AppConfig {
    project_keys: Vec<String>,
    expose_test_events: bool,
}

impl AppConfig {
    pub fn with_project_keys(keys: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            project_keys: keys.into_iter().map(Into::into).collect(),
            expose_test_events: false,
        }
    }

    pub fn with_test_events(mut self, enabled: bool) -> Self {
        self.expose_test_events = enabled;
        self
    }

    pub fn expose_test_events(&self) -> bool {
        self.expose_test_events
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
