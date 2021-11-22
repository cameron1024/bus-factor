use std::collections::HashMap;

use reqwest::{
    header::{ACCEPT, AUTHORIZATION, USER_AGENT},
    RequestBuilder,
};
use secrecy::ExposeSecret;

use crate::{errors::Error, model::*};

#[async_trait]
pub trait GithubClient: Clone {
    async fn list_repositories<Q: Into<Query> + Send>(
        &self,
        query: Q,
    ) -> Result<Vec<Repository>, Error>;

    async fn list_contributors(&self, repository: &Repository) -> Result<Vec<Contributor>, Error>;
}

#[derive(Clone)]
pub struct DefaultClient {
    api_key: ApiKey,
}

const REPO_SEARCH_URL: &str = "https://api.github.com/search/repositories";
const V3_API_STR: &str = "application/vnd.github.v3+json";

impl DefaultClient {
    pub fn create(api_key: ApiKey) -> Self {
        Self { api_key }
    }

    fn build_default_request(&self, s: impl AsRef<str>) -> RequestBuilder {
        debug!("creating request builder for url: {}", s.as_ref());
        let auth = format!("token {}", self.api_key.expose_secret());

        reqwest::Client::new()
            .get(s.as_ref())
            .header(USER_AGENT, "rust") // github requires user agent headers
            .header(ACCEPT, V3_API_STR) // explicitly set v3 API, recommended
            .header(AUTHORIZATION, auth)
    }

    fn make_repo_query_params(query: Query) -> HashMap<&'static str, String> {
        let query_string = format!("language:{} sort:stars", query.language);
        let mut map = HashMap::with_capacity(1);
        map.insert("q", query_string);
        map.insert("per_page", query.limit.to_string());
        map
    }

    fn get_contributors_url(repo: &Repository) -> String {
        format!(
            "https://api.github.com/repos/{}/{}/contributors",
            repo.owner.login, repo.name
        )
    }
}

#[async_trait]
impl GithubClient for DefaultClient {
    async fn list_repositories<Q: Into<Query> + Send>(
        &self,
        query: Q,
    ) -> Result<Vec<Repository>, Error> {
        #[derive(serde::Deserialize)]
        struct Response {
            items: Vec<Repository>,
        }

        let query = Self::make_repo_query_params(query.into());
        let response: Response = self
            .build_default_request(REPO_SEARCH_URL)
            .query(&query)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.items)
    }

    async fn list_contributors(&self, repository: &Repository) -> Result<Vec<Contributor>, Error> {
        let url = Self::get_contributors_url(repository);
        let response = self.build_default_request(url).send().await?.json().await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use microtype::SecretMicrotype;

    use super::*;

    #[test]
    fn correctly_generates_contributors_url() {
        let repo = Repository {
            owner: Owner {
                login: "owner".into(),
            },
            name: "repo_name".into(),
        };

        let url = DefaultClient::get_contributors_url(&repo);
        assert_eq!(url, "https://api.github.com/repos/owner/repo_name/contributors");
    }

    #[test]
    fn correct_query_params() {
        let query = Query {
            limit: 20,
            language: "rust".into(),
        };
        let map = DefaultClient::make_repo_query_params(query);
        assert_eq!(map.get("q"), Some(&"language:rust sort:stars".to_string()));
        assert_eq!(map.get("per_page"), Some(&"20".to_string()));
    }
}
