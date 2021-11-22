#[macro_use]
extern crate structopt;

#[macro_use]
extern crate microtype;

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;

mod args;
mod calculate;
mod client;
mod environment;
mod errors;
mod model;

pub mod prelude {
    pub use super::args::Args;
    pub use super::calculate::execute_query;
    pub use super::client::{DefaultClient, GithubClient};
    pub use super::environment::get_api_key;
    pub use super::errors::*;
    pub use super::model::*;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    pub(crate) fn test_file_path(s: impl AsRef<str>) -> PathBuf {
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let test_dir = base.join("test");
        test_dir.join(s.as_ref())
    }
}
