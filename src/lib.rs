use std::error::Error;
use reqwest;
use rss::Channel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RSSFeed {
  name: String,
  url: String,
}

impl RSSFeed {
    pub fn new(name: String, url: String) -> RSSFeed {
        RSSFeed { name, url }
    }

    pub async fn get_articles(self) -> Result<Vec<Article>, Box<dyn Error>> {
        let article: Vec<Article> = Article::get_info(&self.url).await.or(Err("Address not found"))?;
        Ok(article)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
  pub title: String,
  pub summary: String,
}

impl Article {
    pub async fn get_info(url: &String) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut result: Vec<Article> = vec![];
        let response = reqwest::get(url).await?;
        let content = response.bytes().await?;
        let rss = Channel::read_from(&content[..])?;

        for item in rss.items() {
            result.push(Article {
              title: item.title()
                .ok_or(format!("No title available"))?
                .to_string(),
              summary: item.description()
                .ok_or(format!("No summary available"))?
                .to_string(),
            });
        }

        Ok(result)
    }
}
