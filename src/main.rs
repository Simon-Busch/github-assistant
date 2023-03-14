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
    let issues_list = &items.items;
    println!("{:?}", issues_list);
    println!("{:?}", issues_list[0].url);
    println!("{:?}", issues_list[1].body);
    println!("{:?}", items.total_count);

    let menu_titles = vec!["Home","Issues", "PullRequests", "Quit"]; // vec!["Home", "Issues", "PR", "Quit"]
    let mut active_menu_item = MenuItem::Home;
    let mut issue_list_state = ListState::default();
    issue_list_state.select(Some(0));
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
                          [Constraint::Percentage(15), Constraint::Percentage(80)].as_ref(),
                      )
                      .split(chunks[1]);
                    let selected_issue_index = issue_list_state.selected();
                    let (left, right) = render_issues(&issues_list, selected_issue_index);
                  rect.render_stateful_widget(left, pets_chunks[0], &mut issue_list_state);
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
              KeyCode::Down => {
                if let Some(selected) = issue_list_state.selected() {
                    let next = selected + 1;
                    if next < issues_list.len() {
                        issue_list_state.select(Some(next));
                    }
                }
              }
              KeyCode::Up => {
                  if let Some(selected) = issue_list_state.selected() {
                      let next = selected.checked_sub(1);
                      if let Some(new_selection) = next {
                          issue_list_state.select(Some(new_selection));
                      }
                  }
              }
              KeyCode::Enter => {
                if let Some(selected) = issue_list_state.selected() {
                    let url = &items.items[selected].url;
                    if let Err(e) = open::that(url) {
                        eprintln!("Failed to open URL '{}': {}", url, e);
                    }
                }
              }

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


fn render_issues<'a>(issues: &Vec<ApiResponseItem>, selected_issue_index: Option<usize>) -> (List<'a>, Table<'a>) {
  let items: Vec<ListItem> = issues
      .iter()
      .map(|i| {
          let mut labels = i.labels.iter().map(|l| l.name.as_str()).collect::<Vec<_>>();
          labels.sort();
          let mut labels_string = labels.join(", ");
          if labels_string.len() > 20 {
              labels_string.truncate(20);
              labels_string.push_str("...");
          }
          ListItem::new(Spans::from(vec![
              Span::raw(format!(
                  "{: <4} | {: <20} | {: <30} | {: <20}",
                  i.number, i.state, labels_string, i.title
              )),
          ]))
      })
      .collect();

  let issue_list = List::new(items)
      .block(Block::default().title("Issues").borders(Borders::ALL))
      .style(Style::default().fg(Color::White))
      .highlight_style(Style::default().fg(Color::Yellow));

  let binding = ApiResponseItem {
          url: "".to_owned(),
          title: "".to_owned(),
          number: 0,
          body: None,
          state: "".to_owned(),
          created_at: "".to_owned(),
          labels: vec![],
      };
  let selected_issue = selected_issue_index
      .map(|i| &issues[i])
      .unwrap_or(&binding);

  let issue_details = Table::new(vec![
      Row::new(vec![
          Cell::from("Number"),
          Cell::from("Title"),
          Cell::from("Labels"),
          Cell::from("State"),
      ])
      .style(Style::default().fg(Color::Yellow))
      .height(1),
      Row::new(vec![
          Cell::from(selected_issue.number.to_string()),
          Cell::from(selected_issue.title.clone()),
          Cell::from(selected_issue.labels.iter().map(|l| l.name.as_str()).collect::<Vec<_>>().join(", ")),
          Cell::from(selected_issue.state.clone()),
      ])
      .style(Style::default().fg(Color::White))
      .height(1),
    ])
    .block(
        Block::default()
            .title("Details")
            .borders(Borders::ALL),
    )
    .widths(&[
        Constraint::Length(6),
        Constraint::Length(50),
        Constraint::Length(20),
        Constraint::Min(0),
    ])
    .highlight_style(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::LightMagenta),
    )
    .highlight_symbol("> ");

  (issue_list, issue_details)
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
      "{}/search/issues?q=assignee:{}+state:open&per_page=100",
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
  // #[serde(skip)]
  number: i32,
  state: String,
  created_at: String,
  labels: Vec<Label>,
  body: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
struct Label {
  name: String,
}
