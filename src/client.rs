use reqwest::blocking::Client;
use std::env;

use crate::cli_config::LrConfig;

pub fn get_client(config: &LrConfig) -> Client {
    let api_key =
        env::var("LINEAR_API_KEY").ok().or_else(|| config.api_key.clone()).expect("No api key in config or LINEAR_API_KEY env var");

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
