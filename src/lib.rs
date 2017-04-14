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



pub struct Config {
  pub server: String,
  pub cliend_id: String,
  pub client_secret: String,
  pub access_token: String,
}

pub struct Client {
  client: hyper::Client,
  config: Config,
}

impl Client {
  fn get<T, U>(&self, url: U) -> Result<T>
    where T: serde::Deserialize + Default,
          U: hyper::client::IntoUrl
  {
    let builder = self.client.get(url);
    let response = builder.send()?;
    serde_json::from_reader(response).map_err(Into::into)
  }

  fn post<T, U>(&self, url: U) -> Result<T>
    where T: serde::Deserialize + Default,
          U: hyper::client::IntoUrl
  {
    let builder = self.client.post(url);
    let response = builder.send()?;
    serde_json::from_reader(response).map_err(Into::into)
  }
}

impl Client {
  pub fn new(config: Config) -> Result<Self> {
    use hyper::net::HttpsConnector;
    use hyper_native_tls::NativeTlsClient;

    let client = NativeTlsClient::new().map(HttpsConnector::new)
      .map(hyper::Client::with_connector)?;
    Ok(Client {
         client: client,
         config: config,
       })
  }

  pub fn authenticate<U, P>(&self, username: U, password: P) -> Result<()>
    where U: AsRef<str>,
          P: AsRef<str>
  {
    Ok(())
  }
}
