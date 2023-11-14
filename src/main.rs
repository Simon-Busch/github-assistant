mod structs;
use structs::ApiResponseItem;

mod api;
use api::{ init_gh_data, update_issue_status };

mod render_items;
use render_items::{
    render_home,
    render_issues,
    render_waiting_screen,
    render_popup,
    render_error,
    render_loading_popup,
};

mod utils;
use utils::{
    get_current_state_and_list,
    get_current_state_repo_org_list,
    move_selection,
    move_selection_org_repo,
    get_org_list,
    get_repo_list,
    filter_issues_by_state,
    filter_issues_by_org,
    filter_issues_by_repo,
};

use dotenv::dotenv;
use tokio;
use std::{ error::Error, sync::mpsc };
use tui::{
    backend::CrosstermBackend,
    layout::{ Alignment, Constraint, Direction, Layout },
    style::{ Color, Modifier, Style },
    text::{ Span, Spans },
    widgets::{ Block, BorderType, Borders, ListState, Paragraph, Tabs },
    Terminal,
};
use crossterm::{
    event::{ self, Event as CEvent, KeyCode, KeyModifiers },
    terminal::{ disable_raw_mode, enable_raw_mode },
};
use std::time::{ Duration, Instant };
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
    Refresh,
    ToReview,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Assignments => 1,
            MenuItem::Closed => 2,
            MenuItem::Refresh => 3,
            MenuItem::ToReview => 4,
        }
    }
}

fn init_variables() -> (String, String) {
    dotenv().ok();
    let username = std::env
        ::var("GITHUB_USERNAME")
        .expect(
            "GITHUB_USERNAME must be set. Make sure you run export GITHUB_USERNAME='your username'"
        );
    let access_token = std::env
        ::var("GITHUB_TOKEN")
        .expect(
            "GITHUB_TOKEN must be set. Make sure you run export GITHUB_TOKEN='your github token'"
        );
    return (username, access_token);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode().expect("can run in raw mode");
    let (username, access_token) = init_variables();
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(1000);
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

    let (
        mut issues_list_open,
        mut issues_list_closed,
        mut assigned_pr_list,
        mut issues_list_open_len,
        mut issues_list_closed_len,
        mut assigned_pr_list_len,
    ) = init_gh_data(&username, &access_token).await?;

    let menu_titles = vec!["Home", "Assignments", "Closed", "Refresh", "To Review", "Quit"];
    let mut active_menu_item = MenuItem::Home;

    let mut issue_list_state_open = ListState::default();
    issue_list_state_open.select(Some(0));

    let mut issue_list_state_closed = ListState::default();
    issue_list_state_closed.select(Some(0));

    let mut issue_list_state_to_review = ListState::default();
    issue_list_state_to_review.select(Some(0));

    let mut action_list_state = ListState::default();
    action_list_state.select(Some(0));

    let mut org_or_repo_list = ListState::default();
    org_or_repo_list.select(Some(0));

    let mut active_open = true;
    let mut show_comment = false;
    let mut to_review_open = false;

    // Create a flag to keep track of whether the prompt window is open
    let mut prompt_open = false;
    let mut show_org_modal = false;
    let mut show_repo_modal = false;
    let mut org_list: Vec<String> = vec![];
    let mut repo_list: Vec<String> = vec![];
    let mut is_loading = false;

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default() // define the Menu
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [Constraint::Length(3), Constraint::Min(2), Constraint::Length(3)].as_ref()
                )
                .split(size);

            let copyright = Paragraph::new(
                "Github Assistant - All rights reserved - V0.1.5 - Simon-Busch ®"
            )
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
                    Spans::from(
                        vec![
                            Span::styled(
                                first,
                                Style::default()
                                    .fg(Color::LightCyan)
                                    .add_modifier(Modifier::UNDERLINED)
                            ),
                            Span::styled(rest, Style::default().fg(Color::White))
                        ]
                    )
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item.into())
                .block(
                    Block::default()
                        .title("Menu")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                )
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::LightCyan))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);
            match active_menu_item {
                MenuItem::Home => if is_loading == true {
                    let data_chunk = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(100)].as_ref())
                        .split(chunks[1]);
                    render_loading_popup(rect, data_chunk[0]);
                } else {
                    rect.render_widget(
                        render_home(
                            &issues_list_open_len,
                            &issues_list_closed_len,
                            &assigned_pr_list_len,
                            &username
                        ),
                        chunks[1]
                    );
                }
                MenuItem::Assignments => {
                    if is_loading == true {
                        let data_chunk = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([Constraint::Percentage(100)].as_ref())
                            .split(chunks[1]);
                        render_loading_popup(rect, data_chunk[0]);
                    } else if issues_list_open_len == 0 {
                        render_error(rect, "No assigned issues found");
                    } else {
                        let data_chunck = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints(
                                [Constraint::Percentage(30), Constraint::Percentage(70)].as_ref()
                            )
                            .split(chunks[1]);

                        if active_open == true && show_comment == false {
                            let selected_issue_index = issue_list_state_open.selected();
                            let (left, right) = render_issues(
                                &issues_list_open,
                                selected_issue_index,
                                show_comment
                            );
                            rect.render_stateful_widget(
                                left,
                                data_chunck[0],
                                &mut issue_list_state_open
                            );
                            rect.render_widget(right, data_chunck[1]);
                            if
                                prompt_open == true &&
                                show_org_modal == false &&
                                show_repo_modal == false
                            {
                                render_popup(rect, [].to_vec(), "Actions".to_string(), None);
                            } else if
                                prompt_open == true &&
                                show_org_modal == true &&
                                show_repo_modal == false
                            {
                                render_popup(
                                    rect,
                                    org_list.clone(),
                                    "Choose an organisation".to_string(),
                                    org_or_repo_list.selected()
                                );
                            } else if
                                prompt_open == true &&
                                show_repo_modal == true &&
                                show_org_modal == false
                            {
                                render_popup(
                                    rect,
                                    repo_list.clone(),
                                    "Choose a repository".to_string(),
                                    org_or_repo_list.selected()
                                );
                            }
                        } else if active_open == true && show_comment == true {
                            let selected_issue_index = issue_list_state_open.selected();
                            let (left, right) = render_issues(
                                &issues_list_open,
                                selected_issue_index,
                                show_comment
                            );
                            rect.render_stateful_widget(
                                left,
                                data_chunck[0],
                                &mut issue_list_state_open
                            );
                            rect.render_widget(right, data_chunck[1]);
                        }
                    }
                }
                MenuItem::Closed => {
                    if is_loading == true {
                        let data_chunk = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([Constraint::Percentage(100)].as_ref())
                            .split(chunks[1]);
                        render_loading_popup(rect, data_chunk[0]);
                    } else if issues_list_closed_len == 0 {
                        render_error(rect, "No closed issues found");
                    } else {
                        let data_chunck = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints(
                                [Constraint::Percentage(30), Constraint::Percentage(70)].as_ref()
                            )
                            .split(chunks[1]);
                        if active_open == false {
                            let selected_issue_index = issue_list_state_closed.selected();
                            let (left, right) = render_issues(
                                &issues_list_closed,
                                selected_issue_index,
                                show_comment
                            );
                            rect.render_stateful_widget(
                                left,
                                data_chunck[0],
                                &mut issue_list_state_closed
                            );
                            rect.render_widget(right, data_chunck[1]);
                        }
                    }
                }
                MenuItem::Refresh => {}
                MenuItem::ToReview => {
                    if assigned_pr_list_len == 0 {
                        render_error(rect, "No Assigned PR");
                        // Wait for 5 seconds
                        //TODO
                        // std::thread::sleep(std::time::Duration::from_secs(5));
                        // Set the active menu item to Home
                        active_menu_item = MenuItem::Home;
                    } else {
                        let data_chunck = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints(
                                [Constraint::Percentage(30), Constraint::Percentage(70)].as_ref()
                            )
                            .split(chunks[1]);
                        let selected_issue_index = issue_list_state_to_review.selected();
                        let (left, right) = render_issues(
                            &assigned_pr_list,
                            selected_issue_index,
                            show_comment
                        );
                        rect.render_stateful_widget(
                            left,
                            data_chunck[0],
                            &mut issue_list_state_to_review
                        );
                        rect.render_widget(right, data_chunck[1]);
                    }
                }
            }
            rect.render_widget(copyright, chunks[2]);
        })?;

        match rx.recv()? {
            Event::Input(event) =>
                match (event.code, event.modifiers) {
                    (KeyCode::Char('q'), _) => {
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        break;
                    }
                    (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                        active_menu_item = MenuItem::Home;
                    }
                    (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                        active_open = true;
                        to_review_open = false;
                        active_menu_item = MenuItem::Assignments;
                    }
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        active_open = false;
                        to_review_open = false;
                        active_menu_item = MenuItem::Closed;
                    }
                    (KeyCode::Down, _) => {
                        if show_org_modal == true {
                            let (state, items) = get_current_state_repo_org_list(
                                &mut org_or_repo_list,
                                &org_list
                            );
                            move_selection_org_repo(state, items, 1);
                        } else if show_repo_modal == true {
                            let (state, items) = get_current_state_repo_org_list(
                                &mut org_or_repo_list,
                                &repo_list
                            );
                            move_selection_org_repo(state, items, 1);
                        } else {
                            let (state, items) = get_current_state_and_list(
                                active_open,
                                to_review_open,
                                &mut issue_list_state_open,
                                &mut issue_list_state_closed,
                                &mut issue_list_state_to_review,
                                &issues_list_open,
                                &issues_list_closed,
                                &assigned_pr_list
                            );
                            move_selection(state, items, 1);
                        }
                    }
                    (KeyCode::Up, _) => {
                        if show_org_modal == true {
                            let (state, items) = get_current_state_repo_org_list(
                                &mut org_or_repo_list,
                                &org_list
                            );
                            move_selection_org_repo(state, items, -1);
                        } else if show_repo_modal == true {
                            let (state, items) = get_current_state_repo_org_list(
                                &mut org_or_repo_list,
                                &repo_list
                            );
                            move_selection_org_repo(state, items, -1);
                        } else {
                            let (state, _) = get_current_state_and_list(
                                active_open,
                                to_review_open,
                                &mut issue_list_state_open,
                                &mut issue_list_state_closed,
                                &mut issue_list_state_to_review,
                                &issues_list_open,
                                &issues_list_closed,
                                &assigned_pr_list
                            );
                            move_selection(state, &issues_list_open, -1);
                        }
                    }
                    (KeyCode::Enter, _) => {
                        if show_org_modal == true {
                            let (state, items) = get_current_state_repo_org_list(
                                &mut org_or_repo_list,
                                &org_list
                            );
                            issues_list_open = filter_issues_by_org(
                                &issues_list_open,
                                items[state.selected().unwrap()].clone()
                            );
                            show_org_modal = false;
                            show_repo_modal = false;
                            prompt_open = false;
                            org_or_repo_list.select(Some(0));
                        } else if show_repo_modal == true {
                            let (state, items) = get_current_state_repo_org_list(
                                &mut org_or_repo_list,
                                &repo_list
                            );
                            issues_list_open = filter_issues_by_repo(
                                &issues_list_open,
                                items[state.selected().unwrap()].clone()
                            );
                            show_org_modal = false;
                            show_repo_modal = false;
                            prompt_open = false;
                            org_or_repo_list.select(Some(0));
                        } else {
                            let (state, list) = get_current_state_and_list(
                                active_open,
                                to_review_open,
                                &mut issue_list_state_open,
                                &mut issue_list_state_closed,
                                &mut issue_list_state_to_review,
                                &issues_list_open,
                                &issues_list_closed,
                                &assigned_pr_list
                            );
                            if let Some(selected) = state.selected() {
                                let url = &list[selected].url;
                                if let Err(e) = open::that(url) {
                                    eprintln!("Failed to open URL '{}': {}", url, e);
                                }
                            }
                        }
                    }
                    (KeyCode::Right, _) => {
                        if active_open == true {
                            show_comment = true;
                        }
                    }
                    (KeyCode::Left, _) => {
                        if active_open == true {
                            show_comment = false;
                        }
                    }
                    (KeyCode::Char('1'), _) => {
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
                            let number = list[selected].number;
                            let repo_owner = list[selected].organization
                                .as_ref()
                                .unwrap()
                                .to_owned();
                            let repo_name = list[selected].repository.as_ref().unwrap().to_owned();
                            if prompt_open == true {
                                update_issue_status(
                                    repo_owner,
                                    repo_name,
                                    number,
                                    &access_token,
                                    "closed"
                                ).await?;
                                issues_list_open = issues_list_open
                                    .into_iter()
                                    .filter(|item| item.number != number)
                                    .collect::<Vec<ApiResponseItem>>();
                                issue_list_state_open = ListState::default();
                                issue_list_state_open.select(Some(0));
                                issues_list_open_len = issues_list_open_len - 1;
                                prompt_open = false;
                            }
                        }
                    }
                    (KeyCode::Char('2'), _) => {
                        show_org_modal = true;
                        show_repo_modal = false;
                        org_or_repo_list.select(Some(0));
                        match active_menu_item {
                            MenuItem::Home => {}
                            MenuItem::Assignments => {
                                org_list = get_org_list(&issues_list_open);
                            }
                            MenuItem::Closed => {
                                org_list = get_org_list(&issues_list_closed);
                            }
                            MenuItem::Refresh => {}
                            MenuItem::ToReview => {}
                        }
                    }
                    (KeyCode::Char('3'), _) => {
                        show_repo_modal = true;
                        show_org_modal = false;
                        org_or_repo_list.select(Some(0));
                        match active_menu_item {
                            MenuItem::Home => {}
                            MenuItem::Assignments => {
                                repo_list = get_repo_list(&issues_list_open);
                            }
                            MenuItem::Closed => {
                                repo_list = get_repo_list(&issues_list_closed);
                            }
                            MenuItem::Refresh => {}
                            MenuItem::ToReview => {}
                        }
                    }
                    (KeyCode::Char('n'), _) => {
                        if active_open == true {
                            prompt_open = !prompt_open;
                        }
                    }
                    (KeyCode::Char('P'), KeyModifiers::SHIFT) => {
                        issues_list_open = filter_issues_by_state(&issues_list_open, true);
                    }
                    (KeyCode::Char('I'), KeyModifiers::SHIFT) => {
                        issues_list_open = filter_issues_by_state(&issues_list_open, false);
                    }
                    (KeyCode::Char('r'), KeyModifiers::CONTROL) => {
                      // TODO
                      // Not working, pausing the code for execution so loading is never used
                        is_loading = true;
                        (
                            issues_list_open,
                            issues_list_closed,
                            assigned_pr_list,
                            issues_list_open_len,
                            issues_list_closed_len,
                            assigned_pr_list_len,
                        ) = init_gh_data(&username, &access_token).await.unwrap();
                        is_loading = false;
                    }
                    (KeyCode::Char('t'), KeyModifiers::CONTROL) => {
                        if to_review_open == false {
                            to_review_open = true;
                            active_menu_item = MenuItem::ToReview;
                        }
                    }
                    _ => {}
                }
            Event::Tick => {}
        }
    }
    Ok(())
}
