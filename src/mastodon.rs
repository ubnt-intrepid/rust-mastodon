use hyper;
use serde;
use serde_json;
use Result;

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
