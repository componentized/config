#![no_main]

use exports::wasi::config::store::{Error, Guest};
use wasi::config::store;

const PREFIX_KEY: &str = "prefix";

pub(crate) struct ConfigScope;

impl ConfigScope {
    fn get_prefix() -> Result<String, Error> {
        match scope::get(PREFIX_KEY) {
            Err(e) => Err(scope_err_map(e)),
            Ok(Some(scope)) => Ok(scope),
            Ok(None) => Err(Error::Upstream(String::from(format!(
                "scope config must contain '{PREFIX_KEY}' key"
            )))),
        }
    }
}

impl Guest for ConfigScope {
    fn get(key: String) -> Result<Option<String>, Error> {
        let scope = ConfigScope::get_prefix()?;
        store::get(&format!("{scope}{key}")).map_err(store_err_map)
    }

    fn get_all() -> Result<Vec<(String, String)>, Error> {
        let scope = ConfigScope::get_prefix()?;

        let mut items: Vec<(String, String)> = vec![];
        for (key, value) in store::get_all().map_err(store_err_map)? {
            if key.starts_with(&scope) {
                items.push((key[scope.len()..].to_string(), value));
            }
        }

        Ok(items)
    }
}

fn store_err_map(err: store::Error) -> Error {
    match err {
        store::Error::Upstream(err) => Error::Upstream(err),
        store::Error::Io(err) => Error::Io(err),
    }
}

fn scope_err_map(err: scope::Error) -> Error {
    match err {
        scope::Error::Upstream(err) => Error::Upstream(err),
        scope::Error::Io(err) => Error::Io(err),
    }
}

wit_bindgen::generate!({
    path: "../../wit",
    world: "scope",
    generate_all
});

export!(ConfigScope);
