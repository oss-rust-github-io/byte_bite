extern crate chrono;
pub mod error_db;

use chrono::prelude::{DateTime, Utc};
use error_db::Error;
use serde::{Deserialize, Serialize};
use std::fs;
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
};

#[allow(dead_code)]
const RSS_DB_PATH: &str = "data/rss_db.json";

#[allow(dead_code)]
const ARTICLE_DB_PATH: &str = "data/article_db.json";

#[derive(Serialize, Deserialize, Clone)]
struct RSSFeed {
    rss_id: usize,
    category: String,
    name: String,
    url: String,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Articles {
    article_id: usize,
    rss_id: usize,
    title: String,
    summary: String,
    created_at: DateTime<Utc>,
}

#[allow(dead_code)]
fn read_rss_db() -> Result<Vec<RSSFeed>, Error> {
    let db_content = fs::read_to_string(RSS_DB_PATH)?;
    let parsed: Vec<RSSFeed> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

#[allow(dead_code)]
fn read_articles_db() -> Result<Vec<Articles>, Error> {
    let db_content = fs::read_to_string(ARTICLE_DB_PATH)?;
    let parsed: Vec<Articles> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn render_rss_feed_list<'a>() -> List<'a> {
    let rss_feeds = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("RSS Feeds")
        .border_type(BorderType::Plain);

    let rss_feed_list = read_rss_db().expect("can fetch RSS feed list");
    let items: Vec<_> = rss_feed_list
        .iter()
        .map(|feed| {
            ListItem::new(Spans::from(vec![Span::styled(
                feed.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let rss_feed_list = List::new(items).block(rss_feeds).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    rss_feed_list
}

pub fn render_rss_articles_list<'a>(list_state: &ListState) -> (List<'a>, Paragraph<'a>) {
    let articles = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Articles")
        .border_type(BorderType::Plain);

    let articles_list = read_articles_db().expect("can fetch RSS articles list");

    let items: Vec<_> = articles_list
        .iter()
        .map(|feed| {
            ListItem::new(Spans::from(vec![Span::styled(
                feed.title.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let list = List::new(items).block(articles).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let selected_article = articles_list
        .get(
            list_state
                .selected()
                .expect("there is always a selected article"),
        )
        .expect("exists")
        .clone();

    let article_summary = Paragraph::new(vec![Spans::from(vec![Span::styled(
        selected_article.summary,
        Style::default().fg(Color::LightBlue),
    )])])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain),
    );

    (list, article_summary)
}
