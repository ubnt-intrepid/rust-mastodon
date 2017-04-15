extern crate mastodon;
extern crate clap;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use mastodon::{Mastodon, MastodonConfig};
use mastodon::register::{App, AppConfig};

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


fn main() {
  //register_app();

  let f = OpenOptions::new()
    .read(true)
    .open(std::env::home_dir().unwrap().join("mastodon.json"))
    .unwrap();

  #[derive(Deserialize)]
  struct Config {
    username: String,
    password: String,
    server: String,
    app: App,
  }
  let c: Config = serde_json::from_reader(f).unwrap();

  let config = MastodonConfig::new(c.server, c.app.client_id, c.app.client_secret);
  let mut cli = Mastodon::new(config).unwrap();
  cli.authenticate(&c.username, &c.password).unwrap();
}
