extern crate chrono;
extern crate unicode_width;
pub mod error_db;

use byte_bite::{
    read_articles_db, read_rss_db, render_rss_feed_list, update_rss_db, write_articles_db,
    write_rss_db, Articles,
};
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Clear, ListState, Paragraph, Tabs},
    Terminal,
};
use unicode_width::UnicodeWidthStr;

const APP_HEADING: &str = "BYTE-BITE: Take a bite out of the news and updates with ByteBite";
const MENU_TITLES: [&'static str; 5] = ["Add", "Delete", "Refresh", "Help", "Quit"];

enum Event<I> {
    Input(I),
    Tick,
}

enum InputMode {
    Normal,
    Editing,
    Popup,
}

struct InputBoxApp {
    text_input: String,
    input_mode: InputMode,
}

impl InputBoxApp {
    fn new() -> InputBoxApp {
        InputBoxApp {
            text_input: String::new(),
            input_mode: InputMode::Normal,
        }
    }
}

pub struct PopupApp {
    pub show_refresh_popup: bool,
    pub show_help_popup: bool,
}

impl PopupApp {
    pub fn new() -> PopupApp {
        PopupApp {
            show_refresh_popup: false,
            show_help_popup: false,
        }
    }
}

fn show_popup(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
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

    let mut popup_app = PopupApp::new();
    let mut inputbox_app = InputBoxApp::new();
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;

    let mut rss_list_state = ListState::default();
    rss_list_state.select(Some(0));

    let mut articles_list_state = ListState::default();
    articles_list_state.select(Some(0));

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
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let heading = Paragraph::new(APP_HEADING)
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(BorderType::Plain),
                );

            rect.render_widget(heading, chunks[0]);

            let menu = MENU_TITLES
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

            let (left, middle, right) = render_rss_feed_list(&rss_list_state, &articles_list_state);
            rect.render_stateful_widget(left, rss_chunks[0], &mut rss_list_state);
            rect.render_stateful_widget(middle, rss_chunks[1], &mut articles_list_state);
            rect.render_widget(right, rss_chunks[2]);

            let rss_url = Paragraph::new(inputbox_app.text_input.as_ref())
                .style(match inputbox_app.input_mode {
                    InputMode::Normal => Style::default(),
                    InputMode::Editing => Style::default().fg(Color::Yellow),
                    InputMode::Popup => Style::default(),
                })
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Add new RSS feed (<RSS category> | <RSS Name> | <RSS Url>). Press <Enter> to submit."),
                );
            rect.render_widget(rss_url, chunks[3]);

            match inputbox_app.input_mode {
                InputMode::Normal => {}
                InputMode::Editing => rect.set_cursor(
                    chunks[3].x + inputbox_app.text_input.width() as u16 + 1,
                    chunks[3].y + 1,
                ),
                InputMode::Popup => {}
            }

            let license = Paragraph::new("Released and maintained under GPL-3.0 license")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(BorderType::Plain),
                );

            rect.render_widget(license, chunks[4]);

            if popup_app.show_refresh_popup {
                let area = show_popup(50, 15, size);

                let popup_text = Paragraph::new(
                    "RSS feed refresh has started in background. (Press Esc to go back)",
                )
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(BorderType::Plain),
                );

                rect.render_widget(Clear, area);
                rect.render_widget(popup_text, area);
            }

            if popup_app.show_help_popup {
                let area = show_popup(60, 40, size);

                let rss_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(35),
                        Constraint::Percentage(65),
                    ]
                    .as_ref(),
                )
                .split(area);

                let popup_title_text = Paragraph::new(vec![
                    Spans::from(vec![Span::raw("")]),
                    Spans::from(vec![Span::styled(
                        "Welcome to Byte-Bite",
                        Style::default()
                            .fg(Color::LightBlue)
                            .add_modifier(Modifier::BOLD),
                    )]),
                    Spans::from(vec![Span::raw("")]),
                    Spans::from(vec![Span::styled(
                        "Take a bite out of the news and updates with ByteBite, the bite-sized RSS feed reader that delivers all the essential  news in a pocket-size format.",
                        Style::default().fg(Color::LightBlue),
                    )]),
                    Spans::from(vec![Span::raw("")]),
                ])
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(BorderType::Plain),
                );

                let popup_help_text = Paragraph::new(vec![
                    Spans::from(vec![Span::raw("")]),
                    Spans::from(vec![Span::styled(
                        "       Keyboard Navigation Help",
                        Style::default().fg(Color::Yellow),
                    )]),
                    Spans::from(vec![Span::raw("")]),
                    Spans::from(vec![Span::styled(
                        "       a                     ",
                        Style::default().fg(Color::LightGreen),
                    ), Span::styled(
                        " --> Add new RSS feed url",
                        Style::default().fg(Color::White),
                    )]),
                    Spans::from(vec![Span::styled(
                        "       d                     ",
                        Style::default().fg(Color::LightGreen),
                    ), Span::styled(
                        " --> Delete existing RSS feed",
                        Style::default().fg(Color::White),
                    )]),
                    Spans::from(vec![Span::styled(
                        "       r                     ",
                        Style::default().fg(Color::LightGreen),
                    ), Span::styled(
                        " --> Refresh articles for RSS feed",
                        Style::default().fg(Color::White),
                    )]),
                    Spans::from(vec![Span::styled(
                        "       page-up / page-down   ",
                        Style::default().fg(Color::LightGreen),
                    ), Span::styled(
                        " --> Navigate through list of RSS feeds",
                        Style::default().fg(Color::White),
                    )]),
                    Spans::from(vec![Span::styled(
                        "       arrow-up / arrow-down ",
                        Style::default().fg(Color::LightGreen),
                    ), Span::styled(
                        " --> Navigate through list of articles in each RSS feed",
                        Style::default().fg(Color::White),
                    )]),
                    Spans::from(vec![Span::styled(
                        "       esc                   ",
                        Style::default().fg(Color::LightGreen),
                    ), Span::styled(
                        " --> Exit RSS add option / Exit popup windows",
                        Style::default().fg(Color::White),
                    )]),
                    Spans::from(vec![Span::styled(
                        "       h                     ",
                        Style::default().fg(Color::LightGreen),
                    ), Span::styled(
                        " --> Open help menu",
                        Style::default().fg(Color::White),
                    )]),
                    Spans::from(vec![Span::styled(
                        "       q                     ",
                        Style::default().fg(Color::LightGreen),
                    ), Span::styled(
                        " --> Exit the application",
                        Style::default().fg(Color::White),
                    )]),
                ])
                .alignment(Alignment::Left)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(BorderType::Plain),
                );

                rect.render_widget(Clear, area);
                rect.render_widget(popup_title_text, rss_chunks[0]);
                rect.render_widget(popup_help_text, rss_chunks[1]);
            }
        })?;

        match rx.recv()? {
            Event::Input(_event) => {
                if let CEvent::Key(key) = event::read()? {
                    match inputbox_app.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('a') => {
                                inputbox_app.input_mode = InputMode::Editing;
                            }
                            KeyCode::Char('d') => {
                                let selected = rss_list_state.selected().unwrap();
                                if selected > 0 {
                                    update_rss_db(&mut rss_list_state)
                                        .expect("can remove RSS feed");
                                }
                            }
                            KeyCode::Char('r') => {
                                let selected = rss_list_state.selected().unwrap();
                                if selected > 0 {
                                    thread::spawn(move || {
                                        let rt = tokio::runtime::Builder::new_multi_thread()
                                            .enable_all()
                                            .build()
                                            .unwrap();
                                        rt.block_on(async {
                                            let _ = write_articles_db(selected).await.unwrap();
                                        });
                                    });
                                    popup_app.show_refresh_popup = true;
                                    inputbox_app.input_mode = InputMode::Popup;
                                }
                            }
                            KeyCode::Char('h') => {
                                popup_app.show_help_popup = true;
                                inputbox_app.input_mode = InputMode::Popup;
                            }
                            KeyCode::PageDown => {
                                if let Some(selected) = rss_list_state.selected() {
                                    let num_rss_feeds =
                                        read_rss_db().expect("can fetch rss list").len();
                                    if selected >= num_rss_feeds - 1 {
                                        rss_list_state.select(Some(0));
                                    } else {
                                        rss_list_state.select(Some(selected + 1));
                                    }
                                }
                                articles_list_state.select(Some(0));
                            }
                            KeyCode::PageUp => {
                                if let Some(selected) = rss_list_state.selected() {
                                    let num_rss_feeds =
                                        read_rss_db().expect("can fetch rss list").len();
                                    if selected > 0 {
                                        rss_list_state.select(Some(selected - 1));
                                    } else {
                                        rss_list_state.select(Some(num_rss_feeds - 1));
                                    }
                                }
                                articles_list_state.select(Some(0));
                            }
                            KeyCode::Down => {
                                let rss_feed_list = read_rss_db().expect("can fetch RSS feed list");
                                let selected_rss_feed = rss_feed_list
                                    .get(rss_list_state.selected().unwrap())
                                    .expect("exists")
                                    .clone();
                                let rss_articles_list: Vec<Articles> = read_articles_db()
                                    .expect("can fetch RSS articles list")
                                    .into_iter()
                                    .filter(|r| r.rss_id == selected_rss_feed.rss_id)
                                    .collect();

                                if let Some(selected) = articles_list_state.selected() {
                                    let num_articles = rss_articles_list.len();
                                    if selected >= num_articles - 1 {
                                        articles_list_state.select(Some(0));
                                    } else {
                                        articles_list_state.select(Some(selected + 1));
                                    }
                                }
                            }
                            KeyCode::Up => {
                                let rss_feed_list = read_rss_db().expect("can fetch RSS feed list");
                                let selected_rss_feed = rss_feed_list
                                    .get(rss_list_state.selected().unwrap())
                                    .expect("exists")
                                    .clone();
                                let rss_articles_list: Vec<Articles> = read_articles_db()
                                    .expect("can fetch RSS articles list")
                                    .into_iter()
                                    .filter(|r| r.rss_id == selected_rss_feed.rss_id)
                                    .collect();

                                if let Some(selected) = articles_list_state.selected() {
                                    let num_articles = rss_articles_list.len();
                                    if selected > 0 {
                                        articles_list_state.select(Some(selected - 1));
                                    } else {
                                        articles_list_state.select(Some(num_articles - 1));
                                    }
                                }
                            }
                            KeyCode::Char('q') => {
                                disable_raw_mode()?;
                                terminal.clear()?;
                                terminal.show_cursor()?;
                                return Ok(());
                            }
                            _ => {}
                        },
                        InputMode::Editing => match key.code {
                            KeyCode::Enter => {
                                let input_text: String =
                                    inputbox_app.text_input.drain(..).collect::<String>();
                                write_rss_db(input_text).await?;
                            }
                            KeyCode::Char(c) => {
                                inputbox_app.text_input.push(c);
                            }
                            KeyCode::Backspace => {
                                inputbox_app.text_input.pop();
                            }
                            KeyCode::Esc => {
                                inputbox_app.input_mode = InputMode::Normal;
                            }
                            _ => {}
                        },
                        InputMode::Popup => match key.code {
                            KeyCode::Esc => {
                                popup_app.show_refresh_popup = false;
                                popup_app.show_help_popup = false;
                                inputbox_app.input_mode = InputMode::Normal;
                            }
                            _ => {}
                        },
                    }
                }
            }
            Event::Tick => {}
        }
    }
}
