use crate::model::Query;

#[derive(StructOpt, Debug, Clone, PartialEq, Eq)]
#[structopt(name = "bus-factor")]
pub struct Args {
    #[structopt(long = "project_count")]  // by default, structopt renames this to "project-count"
    project_count: u32,

    #[structopt(long)]
    language: String,
}

impl From<Args> for Query {
    fn from(Args { project_count, language }: Args) -> Self {
        Self {
            limit: project_count,
            language,
        } 
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use structopt::StructOpt;

    use super::*;

    #[test]
    fn correct_usage() {
        assert_eq!(
            Args::from_iter(["bus-factor", "--project_count", "10", "--language", "rust"]),
            Args {
                project_count: 10,
                language: "rust".to_string(),
            }
        );
    }

    fn assert_fails_parse<T: IntoIterator<Item = I>, I: Into<OsString> + Clone>(t: T) {
        let result = Args::from_iter_safe(t);
        assert!(result.is_err());
    }

    #[test]
    fn misspelled_option() {
        assert_fails_parse(["bus-factor", "--project-coun", "10", "--language", "rust"]);
    }

    #[test]
    fn additional_option() {
        assert_fails_parse([
            "bus-factor",
            "--project-count",
            "10",
            "--language",
            "rust",
            "--whats-this",
            "not-sure",
        ]);
    }
    #[test]
    fn missing_option() {
        assert_fails_parse(["bus-factor", "--project-count", "10"]);
    }

    #[test]
    fn can_convert_to_query() {
        let args = 
            Args::from_iter(["bus-factor", "--project_count", "10", "--language", "rust"]);
        let query: Query = args.into();
        assert_eq!(query, Query {
            language: "rust".into(),
            limit: 10,
        });
    }
}
