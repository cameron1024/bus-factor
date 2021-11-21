use crate::{
    model::{Contributor, Repository},
    GithubClient,
};
use std::io::Write;

use crate::{
    errors::Error,
    model::{Query, RepositorySummary},
};

/// execute a full query, writing a sumamry to the output provided
pub async fn calculate<C: GithubClient, W: Write, Q: Into<Query> + Send>(
    client: C,
    output: W,
    query: Q,
) -> Result<(), Error> {
    let repos = client.list_repositories(query).await?;
    let summary_futures = repos
        .into_iter()
        .map(|repo| process_repo(client.clone(), repo));
    let summaries = futures::future::try_join_all(summary_futures).await?;
    format_results(output, summaries)?;
    Ok(())
}

/// perform contributor request, and return a summary of the repo
async fn process_repo<C: GithubClient>(
    client: C,
    repo: Repository,
) -> Result<RepositorySummary, Error> {
    let contributors = client.list_contributors(&repo).await?;
    Ok(summarize(repo.name, contributors))
}

/// summarize the repo, calculating the ratio from the lead contributor
pub fn summarize(
    repo_name: String,
    contributors: impl IntoIterator<Item = Contributor>,
) -> RepositorySummary {
    let mut max_contributor: Option<Contributor> = None;
    let mut total_contributions = 0u64;

    for c in contributors {
        total_contributions += c.contributions;

        let current_max = match &max_contributor {
            None => 0,
            Some(c) => c.contributions,
        };

        if c.contributions > current_max {
            max_contributor = Some(c);
        }
    }

    let max_contributor = max_contributor.expect("no contributors found");

    RepositorySummary {
        repo_name,
        lead_contributor: max_contributor.login,
        percentage: max_contributor.contributions as f64 / total_contributions as f64,
    }
}

/// Format the results into a pretty-printed string, and write them to the provided output
fn format_results(
    mut output: impl Write,
    results: impl IntoIterator<Item = RepositorySummary>,
) -> Result<(), Error> {
    for RepositorySummary {
        repo_name,
        lead_contributor,
        percentage,
    } in results
    {
        writeln!(
            output,
            "project: {}\tuser: {}\tpercentage: {:.2}",
            repo_name, lead_contributor, percentage
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn writes_to_output_in_correct_format() {
        let mut output = vec![];
        format_results(
            &mut output,
            [RepositorySummary {
                repo_name: "ripgrep".to_string(),
                lead_contributor: "burntsushi".to_string(),
                percentage: 0.888888888,
            }],
        )
        .unwrap();
        let string = String::from_utf8(output).unwrap();
        assert_eq!(
            string,
            "project: ripgrep\tuser: burntsushi\tpercentage: 0.89\n"
        );
    }

    #[test]
    fn correctly_summarizes_repos() {
    }
}
