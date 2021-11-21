use std::collections::HashMap;

use reqwest::{
    header::{ACCEPT, AUTHORIZATION, USER_AGENT},
    RequestBuilder,
};
use secrecy::ExposeSecret;

use crate::{errors::Error, model::*};

#[async_trait]
pub trait GithubClient : Clone{
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
        let auth = format!("token {}", self.api_key.expose_secret());

        reqwest::Client::new()
            .get(s.as_ref())
            .header(USER_AGENT, "rust") // github requires user agent headers
            .header(ACCEPT, V3_API_STR) // explicitly set v3 API, recommended
            .header(AUTHORIZATION, auth)
    }

    fn make_repo_query(query: Query) -> HashMap<String, String> {
        let query_string = format!("language:{}", query.language);
        let mut map = HashMap::with_capacity(1);
        map.insert("q".into(), query_string);
        map
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

        let query = Self::make_repo_query(query.into());
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
        let response = self
            .build_default_request(&repository.collaborators_url)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}
