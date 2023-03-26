use tui::{
  layout::{Constraint},
  style::{Color, Modifier, Style},
  text::{Span, Spans},
  widgets::{Block, BorderType, Borders, Cell, List, ListItem, Row, Table},
};
use crate::structs::ApiResponseItem;
use chrono::{Duration as ChronoDuration, Utc, DateTime};
use textwrap::wrap;
use crossterm::terminal::size;


pub fn render_issues<'a>(issues: &Vec<ApiResponseItem>, selected_issue_index: Option<usize>, show_comment: bool) -> (List<'a>, Table<'a>) {
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

    let body_height = match &selected_issue.body {
          Some(body) => body.lines().count() + 1,
          None => 1,
    };
    let issue_details;

    if show_comment == true {
        let comments_text: Vec<String> = selected_issue
        .comments_list
        .iter()
        .filter(|comment| comment.user.login != "netlify[bot]" && comment.user.login != "gatsby-cloud[bot]" )
        .map(|comment| {
          let formatted_body = wrap(&comment.body, body_width).join("\n");
          format!("{}: {}", comment.user.login, formatted_body)
        })
        .collect();
        let comments_cell;
        if comments_text.len() == 0 {
            comments_cell = Cell::from("No comments");
        } else {
            comments_cell = Cell::from(comments_text.join("\n\n"));
        }
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
            Row::new(vec![
                Cell::from("Comments")
                    .style(Style::default().fg(Color::LightCyan))
            ]),
            Row::new(vec![comments_cell])
                .style(Style::default().fg(Color::White))
                .height(50),
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

          Row::new(vec![Cell::from("Description")])
          .style(Style::default().fg(Color::LightCyan))
          .height(1),
          Row::new(vec![
              match &selected_issue.body {
                  Some(body) => {
                      let wrapped_body = wrap(body, body_width).join("\n");
                      Cell::from(wrapped_body)
                  }
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
              .border_type(BorderType::Rounded)
              .borders(Borders::ALL),
      )
      .widths(&[Constraint::Min(0)])
      .highlight_style(
          Style::default()
              .add_modifier(Modifier::BOLD)
              .fg(Color::LightMagenta),
      )
    }
  (issue_list, issue_details)
}
