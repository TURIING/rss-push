use rocket::serde::{Deserialize, Serialize};
use rss::Channel;

use crate::error::RssError;

#[derive(Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct RssInfo {
    pub url: String,
    pub title: String,
    pub description: String

}
impl RssInfo {
    pub async fn new(url: String) -> Result<RssInfo, RssError> {
        let res = reqwest::get(url.clone()).await?.bytes().await?;

        let channel = Channel::read_from(&res[..])?;

        Ok(RssInfo { url, title: channel.title, description: channel.description })
    }
    pub fn is_empty(&self) -> bool {
        return self.url.is_empty() && self.title.is_empty() && self.description.is_empty()
    }
}




