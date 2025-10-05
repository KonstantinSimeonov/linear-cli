use reqwest::blocking::Client;
use std::env;

pub fn get_client() -> Client {
    let api_key =
        env::var("LINEAR_API_KEY").expect("LINEAR_API_KEY not set in .env or environment");

    let client = Client::builder()
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&api_key).unwrap(),
            ))
            .collect(),
        )
        .build()
        .unwrap();

    client
}
