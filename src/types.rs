#[derive(Debug, Deserialize)]
pub struct AccountID(pub i64);

#[derive(Debug, Deserialize)]
pub struct AttachmentID(pub i64);

#[derive(Debug, Deserialize)]
pub enum AttachmentType {
  #[serde(rename="image")]
  Image,
  #[serde(rename="video")]
  Video,
  #[serde(rename="gifv")]
  Gifv,
}

#[derive(Debug, Deserialize)]
pub struct NotificationID(pub i64);

#[derive(Debug, Deserialize)]
pub enum NotificationType {
  #[serde(rename="reblog")]
  Reblog,
  #[serde(rename="favourite")]
  Favourite,
  #[serde(rename="follow")]
  Follow,
}

#[derive(Debug, Deserialize)]
pub struct StatusID(pub i64);

#[derive(Debug, Deserialize)]
pub enum StatusVisibility {
  #[serde(rename="public")]
  Public,
  #[serde(rename="unlisted")]
  Unlisted,
  #[serde(rename="private")]
  Private,
  #[serde(rename="direct")]
  Direct,
}


#[derive(Debug, Deserialize)]
pub struct Account {
  pub id: AccountID,
  pub username: String,
  pub acct: String,
  pub display_name: String,
  pub locked: bool,
  pub created_at: String,
  pub followers_count: usize,
  pub following_count: usize,
  pub statuses_count: usize,
  pub note: String,
  pub url: String,
  pub avatar: String,
  pub avatar_static: String,
  pub header: String,
  pub header_static: String,
}

#[derive(Debug, Deserialize)]
pub struct Application {
  pub name: String,
  pub website: String,
}

#[derive(Debug, Deserialize)]
pub struct Attachment {
  pub id: AttachmentID,
  #[serde(rename = "type")]
  pub type_: AttachmentType,
  pub url: String,
  pub remote_url: String,
  pub preview_url: String,
  pub text_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Card {
  pub url: String,
  pub title: String,
  pub description: String,
  pub image: String,
}

#[derive(Debug, Deserialize)]
pub struct Context {
  pub ancestors: Vec<Status>,
  pub decendants: Vec<Status>,
}

#[derive(Debug, Deserialize)]
pub struct Error {
  pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct Instance {
  pub uri: String,
  pub title: String,
  pub description: String,
  pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct Mention {
  pub url: String,
  pub username: String,
  pub acct: String,
  pub id: AccountID,
}

#[derive(Debug, Deserialize)]
pub struct Notification {
  pub id: NotificationID,
  #[serde(rename = "type")]
  pub type_: NotificationType,
  pub created_at: String,
  pub account: Account,
  pub status: Option<Status>,
}

#[derive(Debug, Deserialize)]
pub struct Relationship {
  pub id: AccountID,
  pub following: bool,
  pub followed_by: bool,
  pub blocking: bool,
  pub muting: bool,
  pub requested: bool,
}

#[derive(Debug, Deserialize)]
pub struct Report {
  pub id: AccountID,
  pub action_taken: String,
}

#[derive(Debug, Deserialize)]
pub struct Results {
  pub accounts: Vec<Account>,
  pub statuses: Vec<Status>,
  pub hashtags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Status {
  pub id: StatusID,
  pub uri: String,
  pub url: String,
  pub account: Account,
  pub in_reply_to_id: Option<StatusID>,
  pub in_reply_to_account_id: Option<AccountID>,
  pub reblog: Option<Box<Status>>,
  pub content: String,
  pub created_at: String,
  pub reblogs_count: usize,
  pub favourites_count: usize,
  pub reblogged: Option<bool>,
  pub favourited: Option<bool>,
  pub sensitive: bool,
  pub spoiler_text: String,
  pub visibility: StatusVisibility,
  pub media_attachments: Vec<Attachment>,
  pub mentions: Vec<Mention>,
  pub tags: Vec<Tag>,
  pub application: Option<Application>,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
  pub name: String,
  pub url: String,
}
