use args::Args;
use client::{DefaultClient, GithubClient};
use errors::Error;
use structopt::StructOpt;
use environment::get_api_key;

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
mod model;
mod client;
mod environment;
mod errors;
mod calculate;

#[tokio::main]
async fn main() -> Result<(), Error>  {
    env_logger::init();
    let result = main_result().await;
    match &result {
        Ok(_) => {},
        Err(Error(errors::ErrorKind::MissingAuth, _)) => println!("{}", MISSING_AUTH_ERROR),
        Err(e) => println!("{}", e),
    }

    result
}

const MISSING_AUTH_ERROR: &str = r#"No Github API key provided.
Go to https://github.com/settings/tokens to generate a new token, then provide it by:
 1) setting the BUS_FACTOR_AUTH environment variable
 2) providing it in a file at ~/.bus_factor_auth"#;


// wrapper function to print better error messages
async fn main_result() -> Result<(), Error> {
    let api_key = get_api_key()?;
    let client = DefaultClient::create(api_key);
    let args = Args::from_args();

    calculate::calculate(client, std::io::stdout(), args).await?;

    Ok(())
}
