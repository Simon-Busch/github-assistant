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
use chrono::{Duration as ChronoDuration, Utc, DateTime};

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Assignments,
    Closed,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Assignments => 1,
            MenuItem::Closed => 2,
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
    let issues_list_response_open = get_github_response(&username, &access_token, "open").await?;
    let issues_list_response_closed = get_github_response(&username, &access_token, "closed").await?;

    let mut issues_list_open = issues_list_response_open.items.to_owned();
    //sort alphabetically by repository
    issues_list_open.sort_by_key(|i| i.repository.clone().unwrap_or_default());

    let mut issues_list_closed = issues_list_response_closed.items.to_owned();
    //sort alphabetically by repository
    issues_list_closed.sort_by_key(|i| i.repository.clone().unwrap_or_default());

    let menu_titles = vec!["Home","Assignments", "Closed", "Quit"]; // Add "Refresh",
    let mut active_menu_item = MenuItem::Home;

    let mut issue_list_state_open = ListState::default();
    issue_list_state_open.select(Some(0));

    let mut issue_list_state_closed = ListState::default();
    issue_list_state_closed.select(Some(0));

    let mut active_open = true;
    let mut show_comment = false;

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default() // define the Menu
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
                                .fg(Color::LightCyan)
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
                .highlight_style(Style::default().fg(Color::LightCyan))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);
            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(&issues_list_response_open.total_count, &issues_list_response_closed.total_count), chunks[1]),
                MenuItem::Assignments => {
                    let data_chunck = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(30), Constraint::Percentage(70)].as_ref(),
                        )
                        .split(chunks[1]);
                      if active_open == true && show_comment == false {
                        let selected_issue_index =  issue_list_state_open.selected();
                        let (left, right) = render_issues(&issues_list_open, selected_issue_index, show_comment);
                        rect.render_stateful_widget(left, data_chunck[0], &mut issue_list_state_open);
                        rect.render_widget(right, data_chunck[1]);
                      } else if active_open == true && show_comment == true {
                          let selected_issue_index =  issue_list_state_open.selected();
                          let (left, right) = render_issues(&issues_list_open, selected_issue_index, show_comment);
                          rect.render_stateful_widget(left, data_chunck[0], &mut issue_list_state_open);
                          rect.render_widget(right, data_chunck[1]);
                      }
                },
                MenuItem::Closed => {
                  let data_chunck = Layout::default()
                      .direction(Direction::Horizontal)
                      .constraints(
                          [Constraint::Percentage(30), Constraint::Percentage(70)].as_ref(),
                      )
                      .split(chunks[1]);
                  if active_open == false {
                      let selected_issue_index =  issue_list_state_closed.selected();
                      let (left, right) = render_issues(&issues_list_closed, selected_issue_index, show_comment);
                      rect.render_stateful_widget(left, data_chunck[0], &mut issue_list_state_closed);
                      rect.render_widget(right, data_chunck[1]);
                  }
                },
            }
            rect.render_widget(copyright, chunks[2]);
        })?;

      match rx.recv()? {
          Event::Input(event) => match event.code {
              KeyCode::Char('q') => {
                  disable_raw_mode()?;
                  terminal.show_cursor()?;
                  break;
              },
              KeyCode::Char('h') => active_menu_item = MenuItem::Home,
              KeyCode::Char('a') => {
                  active_open = true;
                  active_menu_item = MenuItem::Assignments
              },
              KeyCode::Char('c') => {
                  active_open = false;
                  active_menu_item = MenuItem::Closed
              },
              KeyCode::Down => {
                  let state;
                  let items;
                  if active_open {
                      state = &mut issue_list_state_open;
                      items = &issues_list_open;
                  } else {
                      state = &mut issue_list_state_closed;
                      items = &issues_list_closed;
                  }
                  if let Some(selected) = state.selected() {
                      let next = selected.checked_add(1);
                      if let Some(new_selection) = next {
                          if new_selection < items.len() {
                              state.select(Some(new_selection));
                          } else {
                              state.select(Some(0));
                          }
                      }
                  }
              },
              KeyCode::Up => {
                  let state;
                  if active_open {
                      state = &mut issue_list_state_open;
                  } else {
                      state = &mut issue_list_state_closed;
                  }
                  if let Some(selected) = state.selected() {
                      let next = selected.checked_sub(1);
                      if let Some(new_selection) = next {
                          state.select(Some(new_selection));
                      } else if active_open {
                          state.select(Some(issues_list_open.len() - 1));
                      } else {
                          state.select(Some(issues_list_closed.len() - 1));
                      }
                  }
              },
              KeyCode::Enter => {
                  let state;
                  let list: &Vec<ApiResponseItem>;
                  if active_open == true {
                      state = &mut issue_list_state_open;
                      list = &issues_list_open;
                  } else {
                      state = &mut issue_list_state_closed;
                      list = &issues_list_closed;
                  }
                  if let Some(selected) = state.selected() {
                      let url = &list[selected].url;
                      if let Err(e) = open::that(url) {
                          eprintln!("Failed to open URL '{}': {}", url, e);
                      }
                  }
              }
              KeyCode::Right => {
                  show_comment = true;
              }
              KeyCode::Left => {
                show_comment = false;
              }
              _ => {}
          },
          Event::Tick => {}
      }
  }
    Ok(())
}


fn render_home<'a>(opened: &i32, closed: &i32) -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome to your GitHub assistant!")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(format!(
            "{} open issues 🚧",
            opened,
        ))]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(format!(
            "{} closed issues ✅",
            closed,
        ))]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
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


fn render_issues<'a>(issues: &Vec<ApiResponseItem>, selected_issue_index: Option<usize>, show_comment: bool) -> (List<'a>, Table<'a>) {
    let mut count = 0;
    let items: Vec<ListItem> = issues
        .iter()
        .map(|i| {
            count += 1;
            let created_at = i.created_at.parse::<DateTime<Utc>>().unwrap();
            let now = Utc::now();
            let diff = now.signed_duration_since(created_at);

            let color = if diff > ChronoDuration::days(90) {
                Color::Red
            } else if diff > ChronoDuration::days(60) {
                Color::Yellow
            } else {
                Color::White
            };

            ListItem::new(Spans::from(vec![
                Span::styled(format!("{: <4} | {: <20}", i.number, i.title), Style::default().fg(color)),
            ]))
        })
        .collect();

    let issue_list = List::new(items)
        .block(Block::default().title(format!("Issues ({} total)", count)).borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::UNDERLINED));

    let binding = ApiResponseItem {
        url: "".to_owned(),
        title: "".to_owned(),
        number: 0,
        body: None,
        state: "".to_owned(),
        repository: None,
        organization: None,
        created_at: "".to_owned(),
        updated_at: "".to_owned(),
        labels: vec![],
        // comments: None,
    };

    let selected_issue = selected_issue_index
        .map(|i| &issues[i])
        .unwrap_or(&binding);

    let body_height = match &selected_issue.body {
          Some(body) => body.lines().count() + 1,
          None => 1,
    };
    let issue_details;
    if show_comment == true {
        issue_details = Table::new(vec![
          Row::new(vec![Cell::from("Number")])
          .style(Style::default().fg(Color::LightCyan))
          .height(1),
          Row::new(vec![
              Cell::from(selected_issue.number.to_string()),
              Cell::from(selected_issue.title.clone()),
              Cell::from(selected_issue.labels.iter().map(|l| l.name.as_str()).collect::<Vec<_>>().join(", ")),
              Cell::from(selected_issue.state.clone()),
          ])
          .style(Style::default().fg(Color::White))
          .height(2),
      ])
      .block(
          Block::default()
              .title("Details")
              .borders(Borders::ALL),
      )
      .widths(&[Constraint::Min(0)])
      .highlight_style(
          Style::default()
              .add_modifier(Modifier::BOLD)
              .fg(Color::LightMagenta),
      )
      .highlight_symbol(">>>>> ");
    } else {
      issue_details = Table::new(vec![
          Row::new(vec![Cell::from("Number")])
          .style(Style::default().fg(Color::LightCyan))
          .height(1),
          Row::new(vec![
              Cell::from(selected_issue.number.to_string()),
              Cell::from(selected_issue.title.clone()),
              Cell::from(selected_issue.labels.iter().map(|l| l.name.as_str()).collect::<Vec<_>>().join(", ")),
              Cell::from(selected_issue.state.clone()),
          ])
          .style(Style::default().fg(Color::White))
          .height(2),

          Row::new(vec![Cell::from("Repository")])
          .style(Style::default().fg(Color::LightCyan))
          .height(1),
          Row::new(vec![
              match &selected_issue.repository {
                  Some(repository) => Cell::from(repository.to_string()),
                  None => Cell::from("N/A"),
              },
          ])
          .style(Style::default().fg(Color::White))
          .height(2),

          Row::new(vec![Cell::from("Organization")])
          .style(Style::default().fg(Color::LightCyan))
          .height(1),
          Row::new(vec![
              match &selected_issue.organization {
                  Some(organization) => Cell::from(organization.to_string()),
                  None => Cell::from("N/A"),
              },
          ])
          .style(Style::default().fg(Color::White))
          .height(2),

          Row::new(vec![Cell::from("Title")])
          .style(Style::default().fg(Color::LightCyan))
          .height(1),
          Row::new(vec![
              Cell::from(selected_issue.title.clone()),
          ])
          .style(Style::default().fg(Color::White))
          .height(2),

          Row::new(vec![Cell::from("Labels")])
          .style(Style::default().fg(Color::LightCyan))
          .height(1),
          Row::new(vec![
              Cell::from(selected_issue.labels.iter().map(|l| l.name.as_str()).collect::<Vec<_>>().join(", ")),
          ])
          .style(Style::default().fg(Color::White))
          .height(2),

          Row::new(vec![Cell::from("Details")])
          .style(Style::default().fg(Color::LightCyan))
          .height(1),
          Row::new(vec![
              match &selected_issue.body {
                  Some(body) => Cell::from(body.to_string()),
                  None => Cell::from("N/A"),
              },
          ])
          .style(Style::default().fg(Color::White))
          .height(body_height.try_into().unwrap()),

          Row::new(vec![Cell::from("Created at")])
          .style(Style::default().fg(Color::LightCyan))
          .height(1),
          Row::new(vec![
              Cell::from(selected_issue.created_at.clone()),
          ])
          .style(Style::default().fg(Color::White))
          .height(2),

          Row::new(vec![Cell::from("Updated at")])
          .style(Style::default().fg(Color::LightCyan))
          .height(1),
          Row::new(vec![
              Cell::from(selected_issue.updated_at.clone()),
          ])
          .style(Style::default().fg(Color::White))
          .height(2),
      ])
      .block(
          Block::default()
              .title("Details")
              .borders(Borders::ALL),
      )
      .widths(&[Constraint::Min(0)])
      .highlight_style(
          Style::default()
              .add_modifier(Modifier::BOLD)
              .fg(Color::LightMagenta),
      )
      .highlight_symbol(">>>>> ");
    }


  (issue_list, issue_details)
}

async fn get_github_response(username: &str, access_token: &str, status: &str) -> Result<ApiResponse, Box<dyn Error>> {
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
      "{}/search/issues?q=assignee:{}+state:{}&per_page=100",
      base_url, username, status
  );
  let github_response = client
      .get(url)
      .send()
      .await?
      .text()
      .await?;

  let mut items: ApiResponse = serde_json::from_str(&github_response)?;

  for item in items.items.iter_mut() {
      let url_parts: Vec<&str> = item.url.split("/").collect();
      item.repository = Some(url_parts[url_parts.len() - 3].to_string());
      item.organization = Some(url_parts[url_parts.len() - 4].to_string());
    //TODO: get comments
    //   if item.state == "open" {
    //     let organization = item.organization.as_ref().map_or("", String::as_str);
    //     let repository = item.repository.as_ref().map_or("", String::as_str);
    //     let issue_number = item.number;
    //     let comments_url = format!(
    //         "{}/repos/{}/{}/issues/{}/comments",
    //         base_url,
    //         organization,
    //         repository,
    //         issue_number
    //     );
    //     let comments_response = client.get(comments_url).send().await?.text().await?;
    //     println!("comments_response: {}", comments_response);
    //     if comments_response.is_empty() {
    //         continue;
    //     } else {
    //         let comments: Vec<IssueComments> = serde_json::from_str(&comments_response)?;
    //         item.comments = comments;
    //     }
    //     // let comments: Vec<IssueComments> = serde_json::from_str(&comments_response)?;
    //     println!("comments: {:?}", comments);
    //     // item.comments = comments;
    // }
  }

  Ok(items)
}

async fn get_issue_comments(access_token: &str, owner: &str, repository: &str, issue_number: &i32) -> Result<Vec<IssueComments>, Box<dyn Error>> {
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
      "{}/repos/{}/{}/issues/{}/comments",
      base_url, owner, repository, issue_number
  );
  let github_response = client
      .get(url)
      .send()
      .await?
      .text()
      .await?;
    let items: Vec<IssueComments> = serde_json::from_str(&github_response)?;
    // println!("{:?}", items);
  Ok(items)
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    total_count: i32,
    items: Vec<ApiResponseItem>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ApiResponseItem {
    #[serde(rename = "html_url")]
    url: String,
    title: String,
    number: i32,
    state: String,
    created_at: String,
    updated_at: String,
    labels: Vec<Label>,
    body: Option<String>,
    repository: Option<String>,
    organization: Option<String>,
    // comments: Option<Vec<IssueComments>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct IssueComments {
    body: String,
    user: User,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct User {
    login: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Label {
    name: String,
}
