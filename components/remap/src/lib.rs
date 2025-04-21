#![no_main]

use exports::wasi::config::store::{Error, Guest};
use std::collections::HashMap;
use wasi::config::store;

pub(crate) struct ConfigRemap;

impl Guest for ConfigRemap {
    fn get(key: String) -> Result<Option<String>, Error> {
        let mut key = key;
        if let Some(k) = mapping::get(&key).map_err(mapping_err_map)? {
            key = k;
        }
        store::get(&key).map_err(config_err_map)
    }

    fn get_all() -> Result<Vec<(String, String)>, Error> {
        let mut mappings = HashMap::new();
        for (key, value) in mapping::get_all().map_err(mapping_err_map)? {
            mappings.insert(key, value);
        }

        let mut config = HashMap::new();
        for (key, value) in store::get_all().map_err(config_err_map)? {
            config.insert(key, value);
        }

        let mut remapped = HashMap::new();
        for (key, value) in config.clone() {
            let mut key = key;
            let mut value = value;
            if let Some(mapped_key) = mappings.get(&key) {
                if let Some(mapped_value) = config.get(mapped_key) {
                    key = String::from(mapped_key);
                    value = String::from(mapped_value);
                }
            };
            remapped.insert(key, value);
        }

        Ok(remapped.into_iter().collect())
    }
}

fn config_err_map(err: store::Error) -> Error {
    match err {
        store::Error::Upstream(err) => Error::Upstream(err),
        store::Error::Io(err) => Error::Io(err),
    }
}

fn mapping_err_map(err: mapping::Error) -> Error {
    match err {
        mapping::Error::Upstream(err) => Error::Upstream(err),
        mapping::Error::Io(err) => Error::Io(err),
    }
}

wit_bindgen::generate!({
    path: "../../wit",
    world: "remap",
    generate_all
});

export!(ConfigRemap);
