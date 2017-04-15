use hyper::Client as RawClient;
use hyper::method::Method;
use hyper::net::HttpsConnector;
use hyper::header::{Authorization, Bearer, ContentType};
use hyper::status::StatusCode;
use hyper_native_tls::NativeTlsClient;
use serde::Deserialize;
use serde_json;
use url::{Url, form_urlencoded};

use Result;
use types::*;

pub struct MastodonConfig {
  pub server: String,
  pub client_id: String,
  pub client_secret: String,
  access_token: Option<String>,
}

impl MastodonConfig {
  pub fn new<S, I, C>(server: S, client_id: I, client_secret: C) -> Self
    where S: Into<String>,
          I: Into<String>,
          C: Into<String>
  {
    MastodonConfig {
      server: server.into(),
      client_id: client_id.into(),
      client_secret: client_secret.into(),
      access_token: None,
    }
  }
}

/// Mastodon API client.
pub struct Mastodon {
  client: RawClient,
  config: MastodonConfig,
}

impl Mastodon {
  pub fn new(config: MastodonConfig) -> Result<Self> {
    let client = NativeTlsClient::new().map(HttpsConnector::new)
      .map(RawClient::with_connector)?;

    Ok(Mastodon {
         client: client,
         config: config,
       })
  }

  pub fn authenticate<U, P>(mut self, username: U, password: P) -> Result<Self>
    where U: AsRef<str>,
          P: AsRef<str>
  {
    let url = Url::parse(&self.config.server).and_then(|u| u.join("/oauth/token"))?;
    let body = form_urlencoded::Serializer::new(String::new())
      .append_pair("client_id", &self.config.client_id)
      .append_pair("client_secret", &self.config.client_secret)
      .append_pair("grant_type", "password")
      .append_pair("username", username.as_ref())
      .append_pair("password", password.as_ref())
      .append_pair("scope", "read write follow")
      .finish();

    let response = self.client
      .post(url)
      .header(ContentType::form_url_encoded())
      .body(&body)
      .send()?;
    if response.status != StatusCode::Ok {
      Err(format!("Failed at POST: {:?}", response))?;
    }

    #[derive(Deserialize)]
    struct AuthResponse {
      access_token: String,
    }
    let response: AuthResponse = serde_json::from_reader(response)?;
    self.config.access_token = Some(response.access_token);

    Ok(self)
  }

  pub fn get_timeline_home(&self) -> Result<Vec<Status>> {
    self.request(Method::Get, "/api/v1/timelines/home")
  }
}

impl Mastodon {
  fn request<T>(&self, method: Method, path: &str) -> Result<T>
    where T: Deserialize
  {
    let url = Url::parse(&self.config.server).and_then(|u| u.join(path))?;
    let token = self.config
      .access_token
      .clone()
      .ok_or("Access token is empty")?;

    let response = self.client
      .request(method, url)
      .header(Authorization(Bearer { token: token }))
      .send()?;
    if response.status != StatusCode::Ok {
      Err(format!("bad request: {:?}", response.status))?;
    }

    serde_json::from_reader(response).map_err(Into::into)
  }
}
