#![no_main]

use exports::wasi::config::store::{Error, Guest};
use std::collections::HashMap;
use wasi::config::store;

pub(crate) struct ConfigDefaults;

impl Guest for ConfigDefaults {
    fn get(key: String) -> Result<Option<String>, Error> {
        let mut value = store::get(&key).map_err(config_err_map)?;
        if value.is_none() {
            value = defaults::get(&key).map_err(defaults_config_err_map)?;
        }
        Ok(value)
    }

    fn get_all() -> Result<Vec<(String, String)>, Error> {
        let mut config = HashMap::new();

        for (key, value) in defaults::get_all().map_err(defaults_config_err_map)? {
            config.insert(key, value);
        }
        for (key, value) in store::get_all().map_err(config_err_map)? {
            config.insert(key, value);
        }

        Ok(config.into_iter().collect())
    }
}

fn config_err_map(err: store::Error) -> Error {
    match err {
        store::Error::Upstream(err) => Error::Upstream(err),
        store::Error::Io(err) => Error::Io(err),
    }
}

fn defaults_config_err_map(err: defaults::Error) -> Error {
    match err {
        defaults::Error::Upstream(err) => Error::Upstream(err),
        defaults::Error::Io(err) => Error::Io(err),
    }
}

wit_bindgen::generate!({
    path: "../../wit",
    world: "defaults",
    generate_all
});

export!(ConfigDefaults);
