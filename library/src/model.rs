use microtype::SecretMicrotype;

secret_microtype!(String => ApiKey);

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct Repository {
    pub name: String,
    pub owner: Owner,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct Owner {
    pub login: String,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Contributor {
    pub login: String,
    pub contributions: u64,
}

#[derive(Debug, Clone, PartialEq)]
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
    use serde_json::from_str;

    use super::*;

    #[test]
    fn api_key_debug_should_be_redacted() {
        let key = ApiKey::new("asdf".into());
        let debug = format!("{:?}", key);
        assert!(debug.contains("REDACTED"));
        assert!(!debug.contains("asdf"));
    }

    #[test]
    fn deserialize_test() {
        let repo: Repository =
            from_str(r#"{"name": "name", "owner": {"login": "owner"}}"#).unwrap();
        assert_eq!(
            repo,
            Repository {
                name: "name".into(),
                owner: Owner {
                    login: "owner".into()
                }
            }
        );

        let contributor: Contributor =
            from_str(r#"{"login": "login", "contributions": 53}"#).unwrap();
        assert_eq!(
            contributor,
            Contributor {
                login: "login".into(),
                contributions: 53,
            }
        );
    }
}
