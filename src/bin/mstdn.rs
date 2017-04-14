extern crate mastodon;
extern crate clap;

fn register_app() {
  use mastodon::register::AppConfig;
  let app = AppConfig::new("https://pawoo.net", "rustydon")
    .redirect_uris("urn:ietf:wg:oauth:2.0:oob")
    .scopes("read")
    .register()
    .unwrap();

  use std::fs::OpenOptions;
  use std::io::Write;
  OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open("app.json")
    .and_then(|mut f| f.write_all(app.to_string().as_bytes()))
    .unwrap();
}

fn main() {
  use mastodon::register::App;

  //register_app();
  let app: App = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/app.json"))
    .parse()
    .unwrap();
  println!("{:?}", app);
}
