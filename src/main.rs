mod structs;
use structs::ApiResponseItem;

mod api;
use api::init_gh_data;

mod render_items;
use render_items::{render_home, render_issues, render_waiting_screen};

mod utils;
use utils::centered_rect;

use dotenv::dotenv;
use tokio;
use std::{error::Error, sync::mpsc};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders,  List, ListItem, ListState, Paragraph, Tabs, Clear},
    Terminal
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
    // Render the loading screen
    render_waiting_screen(&mut terminal)?;

    let (username, access_token) = init_variables();

    let (issues_list_open, issues_list_closed, issues_list_open_len, issues_list_closed_len) = init_gh_data(&username, &access_token).await?;

    let menu_titles = vec!["Home","Assignments", "Closed", "Quit"]; // Add "Refresh",
    let mut active_menu_item = MenuItem::Home;

    let mut issue_list_state_open = ListState::default();
    issue_list_state_open.select(Some(0));

    let mut issue_list_state_closed = ListState::default();
    issue_list_state_closed.select(Some(0));

    let mut action_list_state = ListState::default();
    action_list_state.select(Some(0));

    let mut active_open = true;
    let mut show_comment = false;

    // Create a flag to keep track of whether the prompt window is open
    let mut prompt_open = false;

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
                        .border_type(BorderType::Rounded)
                );

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
                .block(Block::default().title("Menu").borders(Borders::ALL).border_type(BorderType::Rounded))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::LightCyan))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);
            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(&issues_list_open_len, &issues_list_closed_len), chunks[1]),
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
                            if prompt_open == true {
                              let items = vec![
                                ListItem::new("  1 - Close issue"),
                                ListItem::new("  2 - Comment on issue"),
                                ListItem::new("  3 - Reopen issue"),
                            ];

                            let list = List::new(items)
                                .block(
                                    Block::default()
                                        .borders(Borders::ALL)
                                        .title("Actions")
                                )
                                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                                .highlight_symbol(">> ");

                            let popup = Block::default()
                                .borders(Borders::ALL)
                                .title("Select an action")
                                .style(Style::default().fg(Color::White).bg(Color::Black));

                            let popup_chunk = centered_rect(50, 30, rect.size()); // Adjust the width and height values as needed

                            // Render the list on top of the existing widgets
                            rect.render_widget(popup, popup_chunk);
                            rect.render_widget(Clear, popup_chunk);
                            rect.render_widget(list, popup_chunk);

                          }
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
              },
              KeyCode::Right => {
                  if active_open == true {
                    show_comment = true;
                  }
              },
              KeyCode::Left => {
                  if active_open == true {
                      show_comment = false;
                  }
              },
              KeyCode::Char('1')=> {
                // close issue
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
                      let number = &list[selected].number;
                      if prompt_open {
                          println!("Enter a comment");
                          println!("{}", number);
                          //todo
                          // -> implement github actions ...
                      }
                  }
              },
              KeyCode::Char('2')=> {
                // comment on the issue
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
                      let number = &list[selected].number;
                      if prompt_open {
                          println!("Enter a comment");
                          println!("{}", number);
                          //todo
                          // -> implement github actions ...
                      }
                  }
              },
              KeyCode::Char('3')=> {
                  // reopen issue
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
                      let number = &list[selected].number;
                      if prompt_open {
                          println!("Enter a comment");
                          println!("{}", number);
                          //todo
                          // -> implement github actions ...
                      }
                  }
              },
              KeyCode::Char('p') => {
                  prompt_open = !prompt_open;
              }

              _ => {}
          },
          Event::Tick => {}
      }
      }
    Ok(())
}
