
use dotenvy::dotenv;
use once_cell::sync;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    config_debug: bool,
    config_port: String,
}

static CONFIG: sync::Lazy<Config> = sync::Lazy::new(||Config.default())

pub fn new() {
    once.call_once(

    )
    dotenv().ok();

    let cfg = envy::from_env::<Config>().expect("[config]: Config failed to initialized");
}
