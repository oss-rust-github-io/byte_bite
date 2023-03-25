extern crate chrono;
extern crate unicode_width;
pub mod error_db;

use byte_bite::{read_rss_db, render_rss_feed_list, write_rss_db};
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
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, ListState, Paragraph, Tabs},
    Terminal,
};
use unicode_width::UnicodeWidthStr;

const APP_HEADING: &str = "BYTE-BITE: Take a bite out of the news and updates with ByteBite";
const MENU_TITLES: [&'static str; 4] = ["Add", "Update", "Delete", "Quit"];

enum Event<I> {
    Input(I),
    Tick,
}

#[allow(dead_code)]
enum InputMode {
    Normal,
    Editing,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
            rect.render_stateful_widget(left, rss_chunks[0], &mut articles_list_state);
            rect.render_stateful_widget(middle, rss_chunks[1], &mut articles_list_state);
            rect.render_widget(right, rss_chunks[2]);

            let rss_url = Paragraph::new(inputbox_app.text_input.as_ref())
                .style(match inputbox_app.input_mode {
                    InputMode::Normal => Style::default(),
                    InputMode::Editing => Style::default().fg(Color::Yellow),
                })
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Add new RSS url"),
                );
            rect.render_widget(rss_url, chunks[3]);

            match inputbox_app.input_mode {
                InputMode::Normal => {}
                InputMode::Editing => rect.set_cursor(
                    chunks[3].x + inputbox_app.text_input.width() as u16 + 1,
                    chunks[3].y + 1,
                ),
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
        })?;

        match rx.recv()? {
            Event::Input(_event) => {
                if let CEvent::Key(key) = event::read()? {
                    match inputbox_app.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('a') => {
                                inputbox_app.input_mode = InputMode::Editing;
                            }
                            KeyCode::Down => {
                                if let Some(selected) = rss_list_state.selected() {
                                    let num_rss_feeds =
                                        read_rss_db().expect("can fetch rss list").len();
                                    if selected >= num_rss_feeds - 1 {
                                        rss_list_state.select(Some(0));
                                    } else {
                                        rss_list_state.select(Some(selected + 1));
                                    }
                                }
                            }
                            KeyCode::Up => {
                                if let Some(selected) = rss_list_state.selected() {
                                    let num_rss_feeds =
                                        read_rss_db().expect("can fetch rss list").len();
                                    if selected > 0 {
                                        rss_list_state.select(Some(selected - 1));
                                    } else {
                                        rss_list_state.select(Some(num_rss_feeds - 1));
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
                                write_rss_db(input_text).expect("can add new rss feed");
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
                    }
                }
            }
            Event::Tick => {}
        }
    }
}
