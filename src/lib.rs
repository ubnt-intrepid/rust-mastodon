#[macro_use]
extern crate error_chain;
extern crate hyper;
extern crate hyper_native_tls;
extern crate native_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;

pub mod register;
mod mastodon;

pub use mastodon::{Mastodon, MastodonConfig};

use url::ParseError as UrlParseError;
use native_tls::Error as NativeTlsError;
use hyper::Error as HyperError;
use serde_json::Error as SerdeJsonError;
error_chain!{
  foreign_links {
    UrlParse(UrlParseError);
    NativeTls(NativeTlsError);
    Hyper(HyperError);
    SerdeJson(SerdeJsonError);
  }
}
