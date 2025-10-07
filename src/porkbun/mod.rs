use std::sync::LazyLock;

pub mod actions;
pub mod models;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const SECRET_KEY: LazyLock<String> =
    LazyLock::new(|| std::env::var("SECRET_KEY").expect("no secret key set"));

const API_KEY: LazyLock<String> =
    LazyLock::new(|| std::env::var("API_KEY").expect("no api key set"));
