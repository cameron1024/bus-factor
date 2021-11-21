use microtype::SecretMicrotype;

secret_microtype!(String => ApiKey);

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Repository {
    pub name: String,
    pub collaborators_url: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Contributor {
    pub login: String,
    pub contributions: u64,
}

#[derive(Debug, Clone)]
pub struct RepositorySummary {
    pub repo_name: String,
    pub lead_contributor: String,
    pub percentage: f64,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query {
    pub limit: u32,
    pub language: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_key_debug_should_be_redacted() {
        let key = ApiKey::new("asdf".into());
        let debug = format!("{:?}", key);
        assert!(debug.contains("REDACTED"));
        assert!(!debug.contains("asdf"));
    }
}
