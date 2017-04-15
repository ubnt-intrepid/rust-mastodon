extern crate mastodon;
extern crate clap;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::fs::OpenOptions;
use std::io::Write;
use mastodon::{Mastodon, MastodonConfig};
use mastodon::register::{RegisterConfig, Application};

#[allow(dead_code)]
fn register_app() {
  let app = RegisterConfig::new("https://pawoo.net", "rustydon")
    .redirect_uris("urn:ietf:wg:oauth:2.0:oob")
    .scopes("read write follow")
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
  let ref matches = clap::App::new("mstdn")
    .arg_from_usage("--register 'Register App'")
    .get_matches();
  if matches.is_present("register") {
    register_app();
    return;
  }

  let f = OpenOptions::new()
    .read(true)
    .open(std::env::home_dir().unwrap().join("mastodon.json"))
    .unwrap();

  #[derive(Deserialize)]
  struct Config {
    username: String,
    password: String,
    server: String,
    app: Application,
  }
  let c: Config = serde_json::from_reader(f).unwrap();
  let ref username = c.username;
  let ref password = c.password;

  let config = MastodonConfig::new(c.server, c.app.client_id, c.app.client_secret);
  let cli = Mastodon::new(config)
    .and_then(|cli| cli.authenticate(username, password))
    .unwrap();

  let timeline = cli.get_timeline_home().unwrap();
  for status in timeline {
    println!("{:?}", status);
  }
}
