use reqwest::header::InvalidHeaderValue;

#[derive(Debug)]
pub enum Error {
    MissingAuth,
    RequestError(reqwest::Error),
    Io(std::io::Error),
    Headers(InvalidHeaderValue),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use Error::*;

        match self {
            MissingAuth => None,
            RequestError(e) => Some(e),
            Io(e) => Some(e),
            Headers(e) => Some(e),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;

        match self {
            MissingAuth => f.write_str(API_KEY_MESSAGE),
            RequestError(e) => writeln!(f, "{}", e),
            Io(e) => writeln!(f, "{}", e),
            Headers(e) => writeln!(f, "{}", e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::RequestError(e)
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(e: InvalidHeaderValue) -> Self {
        Self::Headers(e)
    }
}
const API_KEY_MESSAGE: &str = r#"Error: No API key provided. A valid Github API key must be provided in either:
 1 the BUS_FACTOR_AUTH environment variable
 2) ~/.bus_factor_auth"#;
