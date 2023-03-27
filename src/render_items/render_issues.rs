use tui::{
  layout::{Constraint},
  style::{Color, Modifier, Style},
  text::{Span, Spans},
  widgets::{Block, BorderType, Borders, Cell, List, ListItem, Row, Table},
};
use crate::{structs::ApiResponseItem, AppState};
use chrono::{Duration as ChronoDuration, Utc, DateTime};
use textwrap::wrap;
use crossterm::terminal::size;


pub fn render_issues<'a>(issues: &Vec<ApiResponseItem>, selected_issue_index: Option<usize>, show_comment: bool, scroll_offset: usize) -> (List<'a>, List<'a>, usize) {
    let mut count = 0;
    // Determine the terminal width, with a default value if it cannot be determined
    let terminal_size = size().unwrap_or_default();
    let terminal_width = terminal_size.0 as usize;
    let percentage = 0.65;
    let body_width = (terminal_width as f32 * percentage) as usize;

    let items: Vec<ListItem> = issues
        .iter()
        .map(|i| {
            count += 1;
            let updated_at = i.updated_at.parse::<DateTime<Utc>>().unwrap();
            let now = Utc::now();
            let diff = now.signed_duration_since(updated_at);

            let color = if diff > ChronoDuration::days(90) {
                Color::Red
            } else if diff > ChronoDuration::days(60) {
                Color::Yellow
            } else {
                Color::White
            };
            let indicator;
            if i.is_pr {
                indicator = "🔗";
            } else {
                indicator = "📄";
            }
            ListItem::new(Spans::from(vec![
                Span::styled(format!("{: <4} | {: <1} |{: <20}", i.number, indicator, i.title), Style::default().fg(color)),
            ]))
        })
        // .skip(scroll_offset)
        // .take(terminal_size.1 as usize - 1)
        .collect();

    let issue_list = List::new(items)
        .block(Block::default().title(format!("Assignments ({} total)", count)).borders(Borders::ALL).border_type(BorderType::Rounded))
        .style(Style::default().fg(Color::White))
        .highlight_symbol("> ");

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
        comments_list: vec![],
        comments_url: "".to_owned(),
        is_pr: false,
    };

    let selected_issue = selected_issue_index
        .map(|i| &issues[i])
        .unwrap_or(&binding);

    let comments_text: Vec<String> = selected_issue
        .comments_list
        .iter()
        .filter(|comment| comment.user.login != "netlify[bot]" && comment.user.login != "gatsby-cloud[bot]" )
        .map(|comment| {
          let formatted_body = wrap(&comment.body, body_width).join("\n");
          format!("{}: {}", comment.user.login, formatted_body)
        })
        .collect();

    let items = vec![
        ListItem::new("Number"),
        ListItem::new(selected_issue.number.to_string()),
        ListItem::new("Repository"),
        ListItem::new(match &selected_issue.repository {
            Some(repository) => repository.to_string(),
            None => "N/A".to_owned(),
        }),
        ListItem::new("Organization"),
        ListItem::new(match &selected_issue.organization {
            Some(organization) => organization.to_string(),
            None => "N/A".to_owned(),
        }),
        ListItem::new("Title"),
        ListItem::new(selected_issue.title.clone()),
        ListItem::new("State"),
        ListItem::new(selected_issue.state.clone()),
        ListItem::new("Labels"),
        ListItem::new(selected_issue.labels.iter().map(|l| l.name.as_str()).collect::<Vec<_>>().join(", ")),
        ListItem::new("Body"),
        ListItem::new(match &selected_issue.body {
            Some(body) => wrap(body, body_width).join("\n"),
            None => "N/A".to_owned(),
        }),
        ListItem::new("Comments"),
        ListItem::new(if comments_text.is_empty() { "No comments".to_owned() } else { comments_text.join("\n\n") }),
      ];

    let list_items = items.into_iter().map(|i| i.style(Style::default().fg(Color::White))).collect::<Vec<_>>();
    let scrollable_list = create_scrollable_list(&list_items, scroll_offset);
  (issue_list, scrollable_list, items.clone().len() as usize)
}

fn create_scrollable_list<'a>(items: &[ListItem<'a>], scroll_offset: usize) -> List<'a> {
  let visible_items = items
      .iter()
      .skip(scroll_offset)
      .cloned()
      .collect::<Vec<ListItem>>();

  List::new(visible_items)
      .block(
          Block::default()
              .title("Details")
              .border_type(BorderType::Rounded)
              .borders(Borders::ALL),
      )
      .highlight_style(
          Style::default()
              .add_modifier(Modifier::BOLD)
              .fg(Color::LightMagenta),
      )
}
