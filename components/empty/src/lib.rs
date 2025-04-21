#![no_main]

use exports::wasi::config::store::{Error, Guest};

pub(crate) struct ConfigEmpty;

impl Guest for ConfigEmpty {
    fn get(_key: String) -> Result<Option<String>, Error> {
        Ok(None)
    }

    fn get_all() -> Result<Vec<(String, String)>, Error> {
        Ok(vec![])
    }
}

wit_bindgen::generate!({
    path: "../../wit",
    world: "empty",
    generate_all
});

export!(ConfigEmpty);
