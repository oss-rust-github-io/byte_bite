use reqwest;

#[tokio::main]
async fn main() {
    let result = reqwest::get("https://www.hindustantimes.com/feeds/rss/cricket/ipl/rssfeed.xml").await;
    println!("{:?}", result);
}
