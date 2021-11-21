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

mod args;
mod model;
mod client;
mod environment;
mod errors;
mod calculate;

#[tokio::main]
async fn main() -> Result<(), Error>  {
    let result = main_result().await;
    match &result {
        Ok(_) => {},
        Err(e) => println!("{}", e),
    }

    result
}


// wrapper function to print better error messages
async fn main_result() -> Result<(), Error> {
    let api_key = get_api_key()?;
    let client = DefaultClient::create(api_key);
    let args = Args::from_args();

    calculate::calculate(client, std::io::stdout(), args).await?;

    Ok(())
}
