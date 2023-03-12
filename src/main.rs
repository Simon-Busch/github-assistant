use reqwest::header::{HeaderValue, ACCEPT};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use tokio;
use reqwest::header;
use std::{error::Error, sync::mpsc};
use tui::{
  backend::CrosstermBackend,
  layout::{Alignment, Constraint, Direction, Layout},
  style::{Color, Modifier, Style},
  text::{Span, Spans},
  widgets::{
      Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
  },
  Terminal,
};
use crossterm::{
  event::{self, Event as CEvent, KeyCode},
  terminal::{disable_raw_mode, enable_raw_mode},
};
use std::time::{Duration, Instant};
use std::io;
use std::thread;

enum Event<I> {
  Input(I),
  Tick,
}

fn init_variables() -> (String, String) {
  dotenv().ok();
  let username = std::env::var("GITHUB_USERNAME").expect("GITHUB_USERNAME must be set.");
  let access_token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set.");
  return (username, access_token);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let (username, access_token) = init_variables();
    let github_response = get_github_response(&username, &access_token).await?;
    let items: ApiResponse = serde_json::from_str(&github_response)?;
    println!("{:?}", items.items);
    println!("{:?}", items.items[0].url);
    println!("{:?}", items.items[1].labels);
    Ok(())
}


fn render_home<'a>() -> Paragraph<'a> {
  let home = Paragraph::new(vec![
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::raw("Welcome")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::raw("to")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::raw("your")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::raw("github")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::raw("assistant")]),
      Spans::from(vec![Span::styled(
          "Simon-Busch ®",
          Style::default().fg(Color::LightBlue),
      )]),
      Spans::from(vec![Span::raw("")]),
      // Spans::from(vec![Span::raw("Press 'p' to access pets, 'a' to add random new pets and 'd' to delete the currently selected pet.")]),
  ])
  .alignment(Alignment::Center)
  .block(
      Block::default()
          .borders(Borders::ALL)
          .style(Style::default().fg(Color::White))
          .title("Home")
          .border_type(BorderType::Plain),
  );
  home
}



async fn get_github_response(username: &str, access_token: &str) -> Result<String, Box<dyn Error>> {
  let mut headers = header::HeaderMap::new();
  headers.insert(
      ACCEPT,
      HeaderValue::from_static("application/vnd.github.v3+json"),
  );
  headers.insert(
      "Authorization",
      HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
  );
  headers.insert("User-Agent", HeaderValue::from_static("my app"));
  let client = reqwest::Client::builder()
      .default_headers(headers)
      .build()?;
  let base_url = "https://api.github.com";
  let url = format!(
      "{}/search/issues?q=assignee:{}",
      base_url, username
  );
  let github_response = client
      .get(url)
      .send()
      .await?
      .text()
      .await?;
  Ok(github_response)
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
  total_count: i32,
  items: Vec<ApiResponseItem>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiResponseItem {
  #[serde(rename = "html_url")]
  url: String,
  title: String,
  #[serde(skip)]
  number: i32,
  state: String,
  created_at: String,
  labels: Vec<Label>,
}
#[derive(Debug, Deserialize, Serialize)]
struct Label {
  name: String,
}
