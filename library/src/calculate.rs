use crate::{
    client::GithubClient,
    model::{Contributor, Repository},
};
use std::io::Write;

use crate::{
    errors::Error,
    model::{Query, RepositorySummary},
};

/// execute a full query, writing a sumamry to the output provided
pub async fn execute_query<C: GithubClient, W: Write, Q: Into<Query> + Send>(
    client: C,
    output: W,
    query: Q,
) -> Result<(), Error> {
    let repos = client.list_repositories(query).await?;
    info!("found {} matching repositories", repos.len());
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
fn summarize(
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
    writeln!(
        output,
        "{0: <20} | {1: <20} | {2: <20}",
        "project", "user", "percentage"
    )?;
    writeln!(output, "{}", "-".repeat(60))?;
    for repo in results {
        if is_bus_factor_1(&repo) {
            let RepositorySummary {
                repo_name,
                lead_contributor,
                percentage,
            } = repo;
            writeln!(
                output,
                "{0: <20} | {1: <20} | {2:.2}",
                repo_name, lead_contributor, percentage
            )?;
        }
    }

    Ok(())
}

fn is_bus_factor_1(repo: &RepositorySummary) -> bool {
    repo.percentage >= 0.75
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn identifies_bus_factor_1_repos() {
        assert!(is_bus_factor_1(&RepositorySummary {
            repo_name: "name".into(),
            lead_contributor: "name".into(),
            percentage: 0.75
        }));
        assert!(is_bus_factor_1(&RepositorySummary {
            repo_name: "name".into(),
            lead_contributor: "name".into(),
            percentage: 0.9
        }));
        assert!(!is_bus_factor_1(&RepositorySummary {
            repo_name: "name".into(),
            lead_contributor: "name".into(),
            percentage: 0.6
        }));
    }

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
        let line = string.lines().skip(2).next().unwrap(); // skip 2 lines of header
        assert!(line.contains("ripgrep"));
        assert!(line.contains("burntsushi"));
        assert!(line.contains("0.89"));
    }

    #[test]
    fn format_results_ignores_non_bus_factor_1() {
        let summary = RepositorySummary {
            repo_name: "".into(),
            lead_contributor: "".into(),
            percentage: 0.74,
        };

        let mut output = vec![];
        format_results(&mut output, [summary]).unwrap();
        let s = String::from_utf8(output).unwrap();
        assert_eq!(s.lines().collect::<Vec<_>>().len(), 2);  // 2 lines from header, rest should be empty
    }

    #[test]
    fn format_results_ignores_non_bus_factor_1_multiple_items() {
        let ignored_summary = RepositorySummary {
            repo_name: "".into(),
            lead_contributor: "".into(),
            percentage: 0.74,
        };
        let printed_sumamry = RepositorySummary {
            repo_name: "repo".into(),
            lead_contributor: "contributor".into(),
            percentage: 0.76,
        };
        let mut both = vec![];
        let mut only_last = vec![];

        format_results(&mut both, [ignored_summary.clone(), printed_sumamry.clone()]).unwrap();
        format_results(&mut only_last, [printed_sumamry]).unwrap();

        assert_eq!(both, only_last);
    }

    fn make_contributors(contributions: impl IntoIterator<Item = u64>) -> Vec<Contributor> {
        let mut v = vec![];
        for (index, number) in contributions.into_iter().enumerate() {
            v.push(Contributor {
                login: format!("user{}", index),
                contributions: number,
            });
        }
        v
    }

    #[test]
    fn correctly_summarizes_repos() {
        let name = "repo name".to_string();
        let contributors = make_contributors([1, 2, 3]);
        let summary = summarize(name, contributors);
        assert_eq!(
            summary,
            RepositorySummary {
                repo_name: "repo name".to_string(),
                lead_contributor: "user2".to_string(),
                percentage: 0.5,
            }
        );
    }

    #[test]
    #[should_panic]
    fn should_panic_when_no_contributors() {
        summarize("".into(), make_contributors([]));
    }
}
