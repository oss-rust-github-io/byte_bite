//! ## Byte-Bite
//! Terminal centric RSS feed reader (powered by Rust) that delivers all the essential news in a pocket-size format
//!
//! ## Key Features:
//! - Enables users to add/remove RSS feeds
//! - Incremental refresh for RSS articles
//! - Help menu provided to help users with keyboard navigation
//!
//! ## Getting Started:
//! Visit the [Byte-Bite official repository](https://github.com/oss-rust-github-io/byte_bite) to download and install the application on the host machine.
//!
//! ## License
//! GPL-3.0 license. See [LICENSE](LICENSE) file.

extern crate chrono;
pub mod error_db;

use chrono::prelude::{DateTime, Utc};
use error_db::{ErrorCodes, ErrorMessages};
use log::{debug, error, info};
use reqwest;
use rss::Channel;
use serde::{Deserialize, Serialize};
use std::fs;
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

/// JSON file path for RSS feed data
pub const RSS_DB_PATH: &str = "data/rss_db.json";

/// JSON file path for RSS articles data
pub const ARTICLE_DB_PATH: &str = "data/article_db.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Defines the metadata for storing RSS feed information
pub struct RSSFeed {
    /// Unique identifier for each RSS feed
    pub rss_id: usize,
    /// RSS feed category (news, sports, technology, etc.)
    pub category: String,
    /// RSS feed name
    pub name: String,
    /// RSS feed URL
    pub url: String,
    created_at: DateTime<Utc>,
}

#[derive(PartialEq, Serialize, Deserialize, Clone)]
/// Defines the metadata for storing RSS articles information
pub struct Articles {
    /// Unique identifier for each Article
    pub article_id: usize,
    /// Unique identifier for each RSS feed
    pub rss_id: usize,
    /// Article title
    pub title: String,
    /// Article summary
    pub summary: String,
    /// URL to navigate to original article
    pub article_link: String,
    /// Article publishing date
    pub pub_date: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

/// Reads the RSS feed information from JSON files
pub fn read_rss_db() -> Vec<RSSFeed> {
    let db_content = fs::read_to_string(RSS_DB_PATH).unwrap_or_else(|_err| {
        let err_msg = ErrorMessages::new(ErrorCodes::E0007_FILE_READ_FAILURE);
        error!("{:?} - {}", err_msg.error_code, err_msg.error_message);
        panic!("{:?} - {}", err_msg.error_code, err_msg.error_message);
    });
    let parsed: Vec<RSSFeed> = serde_json::from_str(&db_content).unwrap_or_else(|_err| {
        let err_msg = ErrorMessages::new(ErrorCodes::E0006_SERDE_JSON_SERIALIZATION_FAILURE);
        error!("{:?} - {}", err_msg.error_code, err_msg.error_message);
        panic!("{:?} - {}", err_msg.error_code, err_msg.error_message);
    });
    debug!("Data read successfully from RSS database.");
    parsed
}

/// Stores the RSS feed information into JSON files
pub async fn write_rss_db(input_text: String) {
    let split_parts = input_text.split("|").collect::<Vec<&str>>();
    let mut parsed: Vec<RSSFeed> = read_rss_db();
    let max_id = parsed
        .iter()
        .max_by_key(|p| p.rss_id)
        .map(|p| p.rss_id)
        .unwrap_or_else(|| {
            let err_msg = ErrorMessages::new(ErrorCodes::E0017_RSS_MAX_ID_FETCH_FAILURE);
            error!("{:?} - {}", err_msg.error_code, err_msg.error_message);
            panic!("{:?} - {}", err_msg.error_code, err_msg.error_message);
        });

    let new_entry = RSSFeed {
        rss_id: max_id + 1,
        category: split_parts[0].trim().to_string(),
        name: split_parts[1].trim().to_string(),
        url: split_parts[2].trim().to_string(),
        created_at: Utc::now(),
    };

    parsed.push(new_entry);

    let parsed: &Vec<u8> = &serde_json::to_vec(&parsed).unwrap_or_else(|_err| {
        let err_msg = ErrorMessages::new(ErrorCodes::E0006_SERDE_JSON_SERIALIZATION_FAILURE);
        error!("{:?} - {}", err_msg.error_code, err_msg.error_message);
        panic!("{:?} - {}", err_msg.error_code, err_msg.error_message);
    });

    fs::write(RSS_DB_PATH, parsed).unwrap_or_else(|_err| {
        let err_msg = ErrorMessages::new(ErrorCodes::E0009_FILE_WRITE_FAILURE);
        error!("{:?} - {}", err_msg.error_code, err_msg.error_message);
        panic!("{:?} - {}", err_msg.error_code, err_msg.error_message);
    });

    let _ = write_articles_db(parsed.len() - 1).await;
}

/// Delete given RSS feed data from JSON files
pub fn update_rss_db(rss_list_state: &mut ListState) {
    if let Some(selected) = rss_list_state.selected() {
        let mut rss_feed_list: Vec<RSSFeed> = read_rss_db();
        rss_feed_list.remove(selected);

        let parsed: &Vec<u8> = &serde_json::to_vec(&rss_feed_list).unwrap_or_else(|_err| {
            let err_msg = ErrorMessages::new(ErrorCodes::E0006_SERDE_JSON_SERIALIZATION_FAILURE);
            error!("{:?} - {}", err_msg.error_code, err_msg.error_message);
            panic!("{:?} - {}", err_msg.error_code, err_msg.error_message);
        });

        fs::write(RSS_DB_PATH, parsed).unwrap_or_else(|_err| {
            let err_msg = ErrorMessages::new(ErrorCodes::E0009_FILE_WRITE_FAILURE);
            error!("{:?} - {}", err_msg.error_code, err_msg.error_message);
            panic!("{:?} - {}", err_msg.error_code, err_msg.error_message);
        });

        debug!("Data updated successfully in RSS database.");

        if selected > 0 {
            rss_list_state.select(Some(selected - 1));
        } else {
            rss_list_state.select(Some(0));
        }
    }
}

/// Reads the RSS articles information from JSON files
pub fn read_articles_db() -> Vec<Articles> {
    let db_content = fs::read_to_string(ARTICLE_DB_PATH).unwrap_or_else(|_err| {
        let err_msg = ErrorMessages::new(ErrorCodes::E0007_FILE_READ_FAILURE);
        error!("{:?} - {}", err_msg.error_code, err_msg.error_message);
        panic!("{:?} - {}", err_msg.error_code, err_msg.error_message);
    });

    let parsed: Vec<Articles> = serde_json::from_str(&db_content).unwrap_or_else(|_err| {
        let err_msg = ErrorMessages::new(ErrorCodes::E0006_SERDE_JSON_SERIALIZATION_FAILURE);
        error!("{:?} - {}", err_msg.error_code, err_msg.error_message);
        panic!("{:?} - {}", err_msg.error_code, err_msg.error_message);
    });
    debug!("Data read successfully from Articles database.");
    parsed
}

/// Stores the RSS articles information into JSON files
pub async fn write_articles_db(rss_selected: usize) {
    let mut articles_list: Vec<Articles> = read_articles_db();
    let rss_feed_list: Vec<RSSFeed> = read_rss_db();

    let selected_rss_feed = rss_feed_list
        .get(rss_selected)
        .unwrap_or_else(|| {
            let err_msg = ErrorMessages::new(ErrorCodes::E0008_LIST_STATE_SELECTION_FAILURE);
            error!("{:?} - {}", err_msg.error_code, err_msg.error_message);
            panic!("{:?} - {}", err_msg.error_code, err_msg.error_message);
        })
        .clone();

    info!("Selected RSS Feed: {:?}", selected_rss_feed);

    let max_timestamp = articles_list
        .iter()
        .max_by_key(|p| p.created_at)
        .map(|p| p.created_at)
        .expect("can fetch max timestamp");

    info!("Max timestamp: {}", max_timestamp);

    let client = reqwest::Client::new();
    let response = client
        .get(selected_rss_feed.url)
        .header(
            reqwest::header::IF_MODIFIED_SINCE,
            max_timestamp.to_rfc2822(),
        )
        .send()
        .await
        .unwrap_or_else(|_err| {
            let err_msg = ErrorMessages::new(ErrorCodes::E0010_HTTP_REQUEST_FAILURE);
            error!("{:?} - {}", err_msg.error_code, err_msg.error_message);
            panic!("{:?} - {}", err_msg.error_code, err_msg.error_message);
        });

    info!("Response status code: {}", response.status());

    if response.status() != 304 {
        let content = response.bytes().await.unwrap_or_else(|_err| {
            let err_msg = ErrorMessages::new(ErrorCodes::E0011_HTTP_RESPONSE_PARSE_FAILURE);
            error!("{:?} - {}", err_msg.error_code, err_msg.error_message);
            panic!("{:?} - {}", err_msg.error_code, err_msg.error_message);
        });

        let rss = Channel::read_from(&content[..]).unwrap_or_else(|_err| {
            let err_msg = ErrorMessages::new(ErrorCodes::E0012_RSS_CHANNEL_PARSE_FAILURE);
            error!("{:?} - {}", err_msg.error_code, err_msg.error_message);
            panic!("{:?} - {}", err_msg.error_code, err_msg.error_message);
        });

        let mut article_id = articles_list
            .iter()
            .max_by_key(|p| p.article_id)
            .map(|p| p.article_id)
            .unwrap_or_else(|| {
                let err_msg = ErrorMessages::new(ErrorCodes::E0013_ARTICLES_LIST_READ_FAILURE);
                error!("{:?} - {}", err_msg.error_code, err_msg.error_message);
                panic!("{:?} - {}", err_msg.error_code, err_msg.error_message);
            });

        for item in rss.items().iter() {
            article_id += 1;

            let title = match item.title() {
                Some(t) => t,
                None => "",
            };

            let summary = match item.description() {
                Some(t) => t,
                None => "",
            };

            let article_link = match item.link() {
                Some(t) => t,
                None => "",
            };

            let pub_date = match item.pub_date() {
                Some(t) => t,
                None => "",
            };

            let new_article = Articles {
                article_id,
                rss_id: selected_rss_feed.rss_id,
                title: title.to_string(),
                summary: summary.to_string(),
                article_link: article_link.to_string(),
                pub_date: DateTime::from(DateTime::parse_from_rfc2822(pub_date).unwrap_or_else(
                    |_err| {
                        let err_msg =
                            ErrorMessages::new(ErrorCodes::E0020_RFC2822_TIMESTAMP_PARSE_FAILURE);
                        error!("{:?} - {}", err_msg.error_code, err_msg.error_message);
                        panic!("{:?} - {}", err_msg.error_code, err_msg.error_message);
                    },
                )),
                created_at: Utc::now(),
            };

            if articles_list.contains(&new_article) {
                continue;
            } else {
                articles_list.push(new_article);
            }
        }

        let parsed: &Vec<u8> = &serde_json::to_vec(&articles_list).unwrap_or_else(|_err| {
            let err_msg = ErrorMessages::new(ErrorCodes::E0006_SERDE_JSON_SERIALIZATION_FAILURE);
            error!("{:?} - {}", err_msg.error_code, err_msg.error_message);
            panic!("{:?} - {}", err_msg.error_code, err_msg.error_message);
        });

        fs::write(ARTICLE_DB_PATH, parsed).unwrap_or_else(|_err| {
            let err_msg = ErrorMessages::new(ErrorCodes::E0009_FILE_WRITE_FAILURE);
            error!("{:?} - {}", err_msg.error_code, err_msg.error_message);
            panic!("{:?} - {}", err_msg.error_code, err_msg.error_message);
        });

        debug!("Data written successfully in Articles database.");
    } else {
        debug!("No new data to write to Articles database.");
    }
}

/// Renders the list of RSS feeds and articles, and articles summary in TUI
pub fn render_rss_feed_list<'a>(
    rss_list_state: &ListState,
    article_list_state: &ListState,
) -> (List<'a>, List<'a>, Paragraph<'a>) {
    let rss_feed_list = read_rss_db();

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

    let mut rss_articles_list: Vec<Articles> = read_articles_db()
        .into_iter()
        .filter(|r| r.rss_id == selected_rss_feed.rss_id)
        .collect();

    rss_articles_list.sort_by_key(|r| std::cmp::Reverse(r.pub_date));

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

    let article_summary = Paragraph::new(vec![
        Spans::from(vec![Span::styled(
            selected_article.title,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            selected_article.summary,
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            format!("Published On: {}", selected_article.pub_date),
            Style::default().fg(Color::White),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            format!("Link to the article: {}", selected_article.article_link),
            Style::default().fg(Color::LightGreen),
        )]),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain),
    )
    .wrap(Wrap { trim: true });

    (rss_list, article_list, article_summary)
}
