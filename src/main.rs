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

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Issues,
    PullRequests,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Issues => 1,
            MenuItem::PullRequests => 2,
        }
    }
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

    let menu_titles = vec!["Home","Issues", "PullRequests", "Quit"]; // vec!["Home", "Issues", "PR", "Quit"]
    let mut active_menu_item = MenuItem::Home;
    let mut pet_list_state = ListState::default();
    pet_list_state.select(Some(0));
    loop {
      terminal.draw(|rect| {
          let size = rect.size();
          let chunks = Layout::default()
              .direction(Direction::Vertical)
              .margin(2)
              .constraints(
                  [
                      Constraint::Length(3),
                      Constraint::Min(2),
                      Constraint::Length(3),
                  ]
                  .as_ref(),
              )
              .split(size);

          let copyright = Paragraph::new("Github Assistant - All rights reserved")
              .style(Style::default().fg(Color::LightCyan))
              .alignment(Alignment::Center)
              .block(
                  Block::default()
                      .borders(Borders::ALL)
                      .style(Style::default().fg(Color::White))
                      .title("Copyright")
                      .border_type(BorderType::Plain)
              )
              ;

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

          let tabs = Tabs::new(menu)
              .select(active_menu_item.into())
              .block(Block::default().title("Menu").borders(Borders::ALL))
              .style(Style::default().fg(Color::White))
              .highlight_style(Style::default().fg(Color::Yellow))
              .divider(Span::raw("|"));

          rect.render_widget(tabs, chunks[0]);
          match active_menu_item {
              MenuItem::Home => rect.render_widget(render_home(), chunks[1]),
              MenuItem::PullRequests => rect.render_widget(render_home(), chunks[1]),
              MenuItem::Issues => {
                  let pets_chunks = Layout::default()
                      .direction(Direction::Horizontal)
                      .constraints(
                          [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                      )
                      .split(chunks[1]);
                  let (left, right) = render_issues(&items.items);
                  rect.render_stateful_widget(left, pets_chunks[0], &mut pet_list_state);
                  rect.render_widget(right, pets_chunks[1]);
              }
          }
          rect.render_widget(copyright, chunks[2]);
      })?;

      match rx.recv()? {
          Event::Input(event) => match event.code {
              KeyCode::Char('q') => {
                  disable_raw_mode()?;
                  terminal.show_cursor()?;
                  break;
              }
              KeyCode::Char('h') => active_menu_item = MenuItem::Home,
              KeyCode::Char('i') => active_menu_item = MenuItem::Issues,
              KeyCode::Char('p') => active_menu_item = MenuItem::PullRequests,
              // KeyCode::Char('p') => active_menu_item = MenuItem::Pets,
              // KeyCode::Char('a') => {
              //     add_random_pet_to_db().expect("can add new random pet");
              // }
              // KeyCode::Char('d') => {
              //     remove_pet_at_index(&mut pet_list_state).expect("can remove pet");
              // }
              // KeyCode::Down => {
              //     if let Some(selected) = pet_list_state.selected() {
              //         let amount_pets = read_db().expect("can fetch pet list").len();
              //         if selected >= amount_pets - 1 {
              //             pet_list_state.select(Some(0));
              //         } else {
              //             pet_list_state.select(Some(selected + 1));
              //         }
              //     }
              // }
              // KeyCode::Up => {
              //     if let Some(selected) = pet_list_state.selected() {
              //         let amount_pets = read_db().expect("can fetch pet list").len();
              //         if selected > 0 {
              //             pet_list_state.select(Some(selected - 1));
              //         } else {
              //             pet_list_state.select(Some(amount_pets - 1));
              //         }
              //     }
              // }
              _ => {}
          },
          Event::Tick => {}
      }
  }
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

fn render_issues<'a>(issues:  &Vec<ApiResponseItem>) -> (List<'a>, Table<'a>) {
  let pets = Block::default()
      .borders(Borders::ALL)
      .style(Style::default().fg(Color::White))
      .title("Pets")
      .border_type(BorderType::Plain);

  // let pet_list = read_db().expect("can fetch pet list");
  let items: Vec<_> = issues
      .iter()
      .map(|issue| {
          ListItem::new(Spans::from(vec![Span::styled(
              issue.title.clone(),
              Style::default(),
          )]))
      })
      .collect();

  let selected_issue = &issues[0];

  let list = List::new(items).block(pets).highlight_style(
      Style::default()
          .bg(Color::Yellow)
          .fg(Color::Black)
          .add_modifier(Modifier::BOLD),
  );

  let issue_details = Table::new(vec![Row::new(vec![
      Cell::from(Span::raw(selected_issue.number.to_string())),
      Cell::from(Span::raw(selected_issue.title.to_string())),
      Cell::from(Span::raw(selected_issue.state.to_string())),
      Cell::from(Span::raw(selected_issue.url.to_string())),
      Cell::from(Span::raw(selected_issue.created_at.to_string())),
  ])])
  .header(Row::new(vec![
      Cell::from(Span::styled(
          "number",
          Style::default().add_modifier(Modifier::BOLD),
      )),
      Cell::from(Span::styled(
          "title",
          Style::default().add_modifier(Modifier::BOLD),
      )),
      Cell::from(Span::styled(
          "state",
          Style::default().add_modifier(Modifier::BOLD),
      )),
      Cell::from(Span::styled(
          "url",
          Style::default().add_modifier(Modifier::BOLD),
      )),
      Cell::from(Span::styled(
          "Created At",
          Style::default().add_modifier(Modifier::BOLD),
      )),
  ]))
  .block(
      Block::default()
          .borders(Borders::ALL)
          .style(Style::default().fg(Color::White))
          .title("Detail")
          .border_type(BorderType::Plain),
  )
  .widths(&[
      Constraint::Percentage(5),
      Constraint::Percentage(20),
      Constraint::Percentage(20),
      Constraint::Percentage(5),
      Constraint::Percentage(20),
  ]);

  (list, issue_details)
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
