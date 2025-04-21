#![no_main]

use exports::wasi::config::store::{Error, Guest};
use std::collections::HashMap;
use wasi::config::store;

pub(crate) struct ConfigOverlay;

impl Guest for ConfigOverlay {
    fn get(key: String) -> Result<Option<String>, Error> {
        let mut value = overlay::get(&key).map_err(overlay_config_err_map)?;
        if value.is_none() {
            value = store::get(&key).map_err(config_err_map)?;
        }
        Ok(value)
    }

    fn get_all() -> Result<Vec<(String, String)>, Error> {
        let mut config = HashMap::new();

        for (key, value) in store::get_all().map_err(config_err_map)? {
            config.insert(key, value);
        }
        for (key, value) in overlay::get_all().map_err(overlay_config_err_map)? {
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

fn overlay_config_err_map(err: overlay::Error) -> Error {
    match err {
        overlay::Error::Upstream(err) => Error::Upstream(err),
        overlay::Error::Io(err) => Error::Io(err),
    }
}

wit_bindgen::generate!({
    path: "../../wit",
    world: "overlay",
    generate_all
});

export!(ConfigOverlay);
