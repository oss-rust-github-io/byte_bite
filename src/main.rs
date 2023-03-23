extern crate chrono;
extern crate thiserror;

use chrono::prelude::{DateTime, Utc};
use crossterm::{
    event::{self, Event as CEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Tabs},
    Terminal,
};

const RSS_DB_PATH: &str = "data/rss_db.json";
const ARTICLE_DB_PATH: &str = "data/article_db.json";

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),

    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Serialize, Deserialize, Clone)]
struct RSSFeed {
    id: usize,
    rss_feed: String,
    category: String,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Articles {
    id: usize,
    rss_id: usize,
    title: String,
    summary: String,
    created_at: DateTime<Utc>,
}

fn read_rss_db() -> Result<Vec<RSSFeed>, Error> {
    let db_content = fs::read_to_string(RSS_DB_PATH)?;
    let parsed: Vec<RSSFeed> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

fn read_articles_db() -> Result<Vec<Articles>, Error> {
    let db_content = fs::read_to_string(ARTICLE_DB_PATH)?;
    let parsed: Vec<Articles> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

fn render_rss_feed_list<'a>() -> List<'a> {
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
                feed.rss_feed.clone(),
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

fn render_rss_articles_list<'a>(list_state: &ListState) -> (List<'a>, Paragraph<'a>) {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let (tx, _rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;

    let app_heading = "BYTE-BITE: Take a bite out of the news and updates with ByteBite";
    let menu_titles = vec!["Add", "Update", "Delete", "Quit"];
    let mut rss_list_state = ListState::default();
    rss_list_state.select(Some(0));

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let heading = Paragraph::new(app_heading)
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(BorderType::Plain),
                );

            rect.render_widget(heading, chunks[0]);

            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let menu_titles = Tabs::new(menu)
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw(" | "));

            rect.render_widget(menu_titles, chunks[1]);

            let rss_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(30),
                        Constraint::Percentage(50),
                    ]
                    .as_ref(),
                )
                .split(chunks[2]);

            let left = render_rss_feed_list();
            let (middle, right) = render_rss_articles_list(&rss_list_state);
            rect.render_stateful_widget(left, rss_chunks[0], &mut rss_list_state);
            rect.render_stateful_widget(middle, rss_chunks[1], &mut rss_list_state);
            rect.render_widget(right, rss_chunks[2]);

            let license = Paragraph::new("Released and maintained under GPL-3.0 license")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(BorderType::Plain),
                );

            rect.render_widget(license, chunks[3]);
        })?;

        disable_raw_mode()?;
    }
}
