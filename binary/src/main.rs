use library::prelude::*;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::from_args();
    let api_key = get_api_key(&args.key_file).expect(MISSING_AUTH_MESSAGE);
    let client = DefaultClient::create(api_key);
    execute_query(client, std::io::stdout(), args).await?;
    Ok(())
}

const MISSING_AUTH_MESSAGE: &str = r#"
No API key provided
Go to https://github.com/settings/tokens to generate a token, then provide it via
 - the BUS_FACTOR_AUTH environment variable
 - a file passed via the --key_file argument
"#;
