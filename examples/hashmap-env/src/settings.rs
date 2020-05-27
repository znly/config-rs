use config::{Config, ConfigError, Environment, File};
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    my_name: String,
    my_map: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct Test {
    my_name: String,
    // Can be encoded in hashmap with `export "APP_MY_MAP"="version=1.0;process=processname"`
    my_map: Option<std::collections::HashMap<String, String>>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("APP"))?;

        s.try_into()
    }
}
