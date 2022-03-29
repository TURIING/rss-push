use rocket::serde::{Deserialize, Serialize};
use rss::Channel;
use std::time::Duration;
use uuid::Uuid;

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
        let res = reqwest::Client::new()
                        .get(url.clone())
                        .timeout(Duration::new(10, 0))
                        .send().await?
                        .bytes().await?;

        let channel = Channel::read_from(&res[..])?;

        Ok(RssInfo { url, title: channel.title, description: channel.description })
    }

    pub fn is_empty(&self) -> bool {
        return self.url.is_empty() && self.title.is_empty() && self.description.is_empty()
    }
}

pub struct Rss;

impl Rss {
    pub async fn get_update(
        url: String, 
        last_msg_uuid: Option<Uuid>
    ) -> Result<Option<Vec<RssItem>>, RssError> {

        let res = reqwest::Client::new()
                        .get(url.clone())
                        .timeout(Duration::new(10, 0))
                        .send().await?
                        .bytes().await?;
        let channel = Channel::read_from(&res[..])?;

        let mut res :Vec<RssItem> = Vec::new();
        let mut index = 1;
        for item in channel.items {
            let title = item.title.unwrap_or_default().to_string();
            let description = item.description.unwrap_or_default().to_string();
            let link = item.link.unwrap_or_default();
            let content = item.content.unwrap_or_default().to_string();
            let pub_date = item.pub_date.unwrap_or_default().to_string();

            let uuid_ctx = title.clone() + &link;
            let item_uuid = Uuid::new_v3(&Uuid::NAMESPACE_URL, uuid_ctx.as_bytes());
            
            if let Some(last_msg_uuid) = last_msg_uuid {
                if last_msg_uuid == item_uuid  {
                    if index == 1 {
                        return Ok(None);
                    } else {
                        return Ok(Some(res));
                    }
                }
            }

            let tmp = RssItem {uuid: item_uuid.to_string(), title, description, content, link: link.to_string(), pub_date};
            res.push(tmp);
            index += 1;
        }
        Ok(Some(res))
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct RssItem {
    pub uuid: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub content: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub link: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub pub_date: String
}


