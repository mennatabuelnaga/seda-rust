use super::Promise;
use crate::{HttpAction, PromiseAction};

pub fn http_fetch(url: &str) -> Promise {
    println!("http_fetch, {url}");
    Promise::new(PromiseAction::Http(HttpAction { url: url.into() }))
}
