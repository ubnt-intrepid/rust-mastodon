use std::str::FromStr;
use std::string::ToString;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::Client;
use hyper::header::ContentType;
use hyper::status::StatusCode;
use serde_json;
use url::{Url, form_urlencoded};
use Result;


/// Configurations of registering application
pub struct RegisterConfig {
  pub server_url: String,
  pub client_name: String,
  pub redirect_uris: String,
  pub scopes: String,
  pub website: Option<String>,
}

impl RegisterConfig {
  pub fn new<S, C>(server_url: S, client_name: C) -> Self
    where S: Into<String>,
          C: Into<String>
  {
    RegisterConfig {
      server_url: server_url.into(),
      client_name: client_name.into(),
      redirect_uris: "urn:ietf:wg:oauth:2.0:oob".into(),
      scopes: "read".into(),
      website: None,
    }
  }

  pub fn redirect_uris<S>(mut self, uris: S) -> Self
    where S: Into<String>
  {
    self.redirect_uris = uris.into();
    self
  }

  pub fn scopes<S>(mut self, scopes: S) -> Self
    where S: Into<String>
  {
    self.scopes = scopes.into();
    self
  }

  pub fn website<S>(mut self, url: S) -> Self
    where S: Into<String>
  {
    self.website = Some(url.into());
    self
  }

  pub fn register(self) -> Result<Application> {
    register(self)
  }

  fn into_form_url_encoded(self) -> String {
    let mut ser = form_urlencoded::Serializer::new(String::new());
    ser.append_pair("client_name", &self.client_name);
    if self.redirect_uris == "" {
      ser.append_pair("redirect_uris", "urn:ietf:wg:oauth:2.0:oob");
    } else {
      ser.append_pair("redirect_uris", &self.redirect_uris);
    }
    ser.append_pair("scopes", &self.scopes);
    ser.append_pair("website",
                    self.website.as_ref().map(|s| s.as_str()).unwrap_or(""));
    ser.finish()
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
  pub id: i64,
  pub redirect_uri: String,
  pub client_id: String,
  pub client_secret: String,
}

impl FromStr for Application {
  type Err = ::Error;
  fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
    serde_json::from_str(s).map_err(Into::into)
  }
}

impl ToString for Application {
  fn to_string(&self) -> String {
    serde_json::to_string_pretty(self).unwrap()
  }
}


/// register a mastodon application to server.
pub fn register(config: RegisterConfig) -> Result<Application> {
  let url = Url::parse(&config.server_url).and_then(|u| u.join("/api/v1/apps"))?;

  let client: Client = NativeTlsClient::new().map(HttpsConnector::new)
    .map(Client::with_connector)?;

  let response = client.post(url)
    .header(ContentType::form_url_encoded())
    .body(&config.into_form_url_encoded())
    .send()?;

  if response.status != StatusCode::Ok {
    Err(format!("bad request: {:?}", response.status))?;
  }

  serde_json::from_reader(response).map_err(Into::into)
}
