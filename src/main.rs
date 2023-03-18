use std::error::Error;
use reqwest;
use rss::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let response = reqwest::get("https://www.hindustantimes.com/feeds/rss/latest/rssfeed.xml".to_string()).await?;
    //println!("{:?}", response);

    let content = response.bytes().await?;
    //println!("{:?}", content);

    let rss = Channel::read_from(&content[..])?;
    //println!("{:?}", rss);

    for item in rss.items() {
        println!("Title: {:?}", item.title());
        println!("Description: {:?}", item.description());
    }

    Ok(())
}
