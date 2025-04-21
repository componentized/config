#![no_main]

use exports::wasi::config::store::{Error, Guest};
use wasi::cli::environment::get_environment;

pub(crate) struct EnvironmentToConfig;

impl Guest for EnvironmentToConfig {
    fn get(key: String) -> Result<Option<String>, Error> {
        for (k, v) in get_environment() {
            if k == key {
                return Ok(Some(v));
            }
        }
        Ok(None)
    }

    fn get_all() -> Result<Vec<(String, String)>, Error> {
        Ok(get_environment())
    }
}

wit_bindgen::generate!({
    path: "../../wit",
    world: "from-environment",
    generate_all
});

export!(EnvironmentToConfig);
