use reqwest::header::InvalidHeaderValue;


error_chain! {

    foreign_links {
        Io(std::io::Error);
        Request(reqwest::Error);
        Headers(InvalidHeaderValue);
    }

    errors {
        MissingAuth {
            description("no Github auth token provided")
        }
    }
}

