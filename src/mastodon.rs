use hyper::Client as RawClient;
use hyper::client::Body;
use hyper::net::HttpsConnector;
use hyper::header::ContentType;
use hyper::status::StatusCode;
use hyper_native_tls::NativeTlsClient;
use serde::Deserialize;
use serde_json;
use url::{Url, form_urlencoded};
use Result;


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

  pub fn authenticate<U, P>(&mut self, username: U, password: P) -> Result<()>
    where U: AsRef<str>,
          P: AsRef<str>
  {
    let body = form_urlencoded::Serializer::new(String::new())
      .append_pair("client_id", &self.config.client_id)
      .append_pair("client_secret", &self.config.client_secret)
      .append_pair("grant_type", "password")
      .append_pair("username", username.as_ref())
      .append_pair("password", password.as_ref())
      .append_pair("scope", "read write follow")
      .finish();

    #[derive(Deserialize)]
    struct AuthResponse {
      access_token: String,
    }
    let response: AuthResponse = self.post("/oauth/token", &body)?;
    self.config.access_token = Some(response.access_token);

    Ok(())
  }
}

impl Mastodon {
  fn post<'a, T, B>(&'a self, path: &str, body: B) -> Result<T>
    where T: Deserialize,
          B: Into<Body<'a>>
  {
    let url = Url::parse(&self.config.server).and_then(|u| u.join(path))?;
    let response = self.client
      .post(url)
      .header(ContentType::form_url_encoded())
      .body(body)
      .send()?;
    if response.status != StatusCode::Ok {
      Err(format!("Failed at POST: {:?}", response))?;
    }
    serde_json::from_reader(response).map_err(Into::into)
  }
}
