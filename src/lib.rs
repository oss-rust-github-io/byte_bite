extern crate chrono;
pub mod error_db;

use chrono::prelude::{DateTime, Utc};
use error_db::Error;
use reqwest;
use rss::Channel;
use serde::{Deserialize, Serialize};
use std::fs;
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
};

pub const RSS_DB_PATH: &str = "data/rss_db.json";
pub const ARTICLE_DB_PATH: &str = "data/article_db.json";

#[derive(Clone, Copy)]
pub struct GaugeApp {
    pub current_value: f64,
}

impl GaugeApp {
    pub fn new() -> GaugeApp {
        GaugeApp { current_value: 0.0 }
    }

    pub fn trigger(&mut self, max_value: f64) {
        self.current_value += 0.1 * ((1 as f64) / max_value);

        if self.current_value > 1.0 {
            self.current_value = 0.0;
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RSSFeed {
    pub rss_id: usize,
    pub category: String,
    pub name: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Articles {
    pub article_id: usize,
    pub rss_id: usize,
    pub title: String,
    pub summary: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Copy)]
pub struct PopupApp {
    pub show_popup: bool,
}

impl PopupApp {
    pub fn new() -> PopupApp {
        PopupApp { show_popup: false }
    }
}

pub fn read_rss_db() -> Result<Vec<RSSFeed>, Error> {
    let db_content = fs::read_to_string(RSS_DB_PATH)?;
    let parsed: Vec<RSSFeed> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn write_rss_db(input_text: String) -> Result<Vec<RSSFeed>, Error> {
    let split_parts = input_text.split(":").collect::<Vec<&str>>();
    let mut parsed: Vec<RSSFeed> = read_rss_db().expect("can fetch RSS feed list");
    let max_id = parsed
        .iter()
        .max_by_key(|p| p.rss_id)
        .map(|p| p.rss_id)
        .expect("can fetch rss feed id");

    let new_entry = RSSFeed {
        rss_id: max_id + 1,
        category: split_parts[0].trim().to_string(),
        name: split_parts[1].trim().to_string(),
        url: split_parts[2].trim().to_string(),
        created_at: Utc::now(),
    };

    parsed.push(new_entry);
    fs::write(RSS_DB_PATH, &serde_json::to_vec(&parsed)?)?;
    Ok(parsed)
}

pub fn read_articles_db() -> Result<Vec<Articles>, Error> {
    let db_content = fs::read_to_string(ARTICLE_DB_PATH)?;
    let parsed: Vec<Articles> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn write_articles_db(
    rss_selected: usize,
    mut gauge_app: GaugeApp,
    mut popup_app: PopupApp,
) -> Result<(), Box<dyn std::error::Error>> {
    let rss_feed_list: Vec<RSSFeed> = read_rss_db().expect("can fetch RSS feed list");
    let selected_rss_feed = rss_feed_list.get(rss_selected).expect("exists").clone();
    let content = match reqwest::blocking::get(selected_rss_feed.url) {
        Ok(content) => content.bytes().unwrap(),
        Err(_) => panic!("Invalid RSS response"),
    };
    let rss = Channel::read_from(&content[..])?;
    let max_value = rss.items().len() as u16;
    let mut articles_list: Vec<Articles> = vec![];

    for (article_id, item) in rss.items().iter().enumerate() {
        gauge_app.trigger(max_value as f64);

        let title = match item.title() {
            Some(t) => t,
            None => "",
        };

        let description = match item.description() {
            Some(t) => t,
            None => "",
        };

        let new_article = Articles {
            article_id,
            rss_id: selected_rss_feed.rss_id,
            title: title.to_string(),
            summary: description.to_string(),
            created_at: Utc::now(),
        };

        articles_list.push(new_article);
        fs::write(ARTICLE_DB_PATH, &serde_json::to_vec(&articles_list)?)?;
    }
    popup_app.show_popup = false;
    Ok(())
}

pub fn render_rss_feed_list<'a>(
    rss_list_state: &ListState,
    article_list_state: &ListState,
) -> (List<'a>, List<'a>, Paragraph<'a>) {
    let rss_feed_list = read_rss_db().expect("can fetch RSS feed list");

    let rss_feeds = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("RSS Feeds")
        .border_type(BorderType::Plain);

    let items: Vec<_> = rss_feed_list
        .iter()
        .map(|feed| {
            ListItem::new(Spans::from(vec![Span::styled(
                feed.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let rss_list = List::new(items).block(rss_feeds).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let selected_rss_feed = rss_feed_list
        .get(
            rss_list_state
                .selected()
                .expect("there is always a selected RSS feed"),
        )
        .expect("exists")
        .clone();

    let rss_articles_list: Vec<Articles> = read_articles_db()
        .expect("can fetch RSS articles list")
        .into_iter()
        .filter(|r| r.rss_id == selected_rss_feed.rss_id)
        .collect();

    let articles = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Articles")
        .border_type(BorderType::Plain);

    let items: Vec<_> = rss_articles_list
        .iter()
        .map(|feed| {
            ListItem::new(Spans::from(vec![Span::styled(
                feed.title.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let article_list = List::new(items).block(articles).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let selected_article = rss_articles_list
        .get(
            article_list_state
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

    (rss_list, article_list, article_summary)
}
