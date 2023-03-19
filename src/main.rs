use byte_bite::RSSFeed;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rss_feed_name = "Hindustan Times".to_string();
    let rss_feed_url = "https://www.hindustantimes.com/feeds/rss/latest/rssfeed.xml".to_string();

    let rss_feed = RSSFeed::new(rss_feed_name, rss_feed_url);
    let rss_article = rss_feed.get_articles().await?;
    println!("{:#?}", rss_article);
    Ok(())
}
