use std::collections::HashMap;

use library::prelude::*;

#[tokio::test]
async fn working_example() {
    let repos = vec![Repository {
        name: "repo1".into(),
        owner: Owner {
            login: "user1".into(),
        },
    }];

    let contributors = HashMap::from_iter([(
        "repo1".into(),
        vec![
            Contributor {
                login: "user1".into(),
                contributions: 1,
            },
            Contributor {
                login: "user2".into(),
                contributions: 9,
            },
        ],
    )]);

    let client = MockClient {
        repos,
        contributors,
    };
    let mut output = vec![];
    execute_query(
        client,
        &mut output,
        Query {
            limit: 1, // these are ignored by mock client
            language: "".into(),
        },
    )
    .await
    .unwrap();

    let actual_output = String::from_utf8(output).unwrap();

    let expected_output = format!(
        "{0: <20} | {1: <20} | {2: <20}\n",
        "project", "user", "percentage"
    );
    let expected_output = format!("{}{}\n", expected_output, "-".repeat(60));
    let expected_output = format!(
        "{}{1: <20} | {2: <20} | {3:.2}\n",
        expected_output, "repo1", "user2", 0.9
    );
    assert_eq!(actual_output, expected_output);
}

#[derive(Clone)]
struct MockClient {
    repos: Vec<Repository>,
    contributors: HashMap<String, Vec<Contributor>>,
}

#[async_trait::async_trait]
impl GithubClient for MockClient {
    async fn list_repositories<Q: Into<Query> + Send>(&self, _query: Q) -> Result<Vec<Repository>> {
        Ok(self.repos.clone())
    }

    async fn list_contributors(&self, repository: &Repository) -> Result<Vec<Contributor>> {
        Ok(self.contributors.get(&repository.name).unwrap().clone())
    }
}
