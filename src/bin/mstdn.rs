extern crate mastodon;
extern crate clap;

use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use mastodon::{Mastodon, MastodonConfig};
use mastodon::register::{App, AppConfig};


fn main() {
  //register_app();
  let app: App = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/app.json"))
    .parse()
    .unwrap();

  let username = env::args().nth(1).unwrap();
  let password = env::args().nth(2).unwrap();

  let config = MastodonConfig::new("https://pawoo.net", app.client_id, app.client_secret);
  let mut cli = Mastodon::new(config).unwrap();
  cli.authenticate(&username, &password).unwrap();
}

#[allow(dead_code)]
fn register_app() {
  let app = AppConfig::new("https://pawoo.net", "rustydon")
    .redirect_uris("urn:ietf:wg:oauth:2.0:oob")
    .scopes("read")
    .register()
    .unwrap();

  OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open("app.json")
    .and_then(|mut f| f.write_all(app.to_string().as_bytes()))
    .unwrap();
}
