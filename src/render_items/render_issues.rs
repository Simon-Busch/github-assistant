use tui::{
    layout::Constraint,
    style::{ Color, Modifier, Style },
    text::{ Span, Spans },
    widgets::{ Block, BorderType, Borders, Cell, List, ListItem, Row, Table },
};
use crate::structs::ApiResponseItem;
use chrono::{ Duration as ChronoDuration, Utc, DateTime };
use textwrap::wrap;
use crossterm::terminal::size;

fn format_date(date_string: &str) -> String {
    match DateTime::parse_from_rfc3339(date_string) {
        // Ok(parsed_date) => parsed_date.format("%Y-%m-%d %H:%M:%S").to_string(),
        Ok(parsed_date) => parsed_date.format("%Y-%m-%d").to_string(),
        Err(_) => "Invalid Date".to_string(), // Handle invalid date strings
    }
}

pub fn render_issues<'a>(
    issues: &Vec<ApiResponseItem>,
    selected_issue_index: Option<usize>,
    show_comment: bool
) -> (List<'a>, Table<'a>) {
    let mut count = 0;
    // Determine the terminal width and hight, with a default value if it cannot be determined
    let terminal_size = size().unwrap_or_default();
    let terminal_width = terminal_size.0 as usize;
    let terminal_height = terminal_size.1 as usize;
    let percentage_width = 0.65;
    let percentage_body = 0.6;
    let body_width = ((terminal_width as f32) * percentage_width) as usize;
    let body_height = ((terminal_height as f32) * percentage_body) as usize;

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
            ListItem::new(
                Spans::from(
                    vec![
                        Span::styled(
                            format!("{: <4} | {: <1} |{: <20}", i.number, indicator, i.title),
                            Style::default().fg(color)
                        )
                    ]
                )
            )
        })
        .collect();

    let issue_list = List::new(items)
        .block(
            Block::default()
                .title(format!("Assignments ({} total)", count))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
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

    let selected_issue = selected_issue_index.map(|i| &issues[i]).unwrap_or(&binding);

    // test
    let org_repo_text = if
        let (Some(org), Some(repo)) = (&selected_issue.organization, &selected_issue.repository)
    {
        format!("Org: {}\nRepo: {}", org, repo)
    } else {
        String::from("N/A")
    };

    let max_lines: usize = body_height.try_into().unwrap_or_default(); // Maximum number of lines based on available height
    let mut display_body = String::new();
    if let Some(body) = &selected_issue.body {
        let lines: Vec<&str> = body.split('\n').collect(); // Split body into lines
        for line in lines.iter().take(max_lines) {
            display_body.push_str(line);
            display_body.push('\n');
        }
        // If the number of lines exceeds available height, add an ellipsis
        if lines.len() > max_lines {
            display_body.push_str("..."); // Indicate there's more text
        }
    } else {
        display_body = String::from("N/A");
    }

    let issue_details;
    if show_comment == true {
        let comments_text: Vec<String> = selected_issue.comments_list
            .iter()
            .filter(
                |comment|
                    comment.user.login != "netlify[bot]" &&
                    comment.user.login != "gatsby-cloud[bot]"
            )
            .map(|comment| {
                let formatted_body = wrap(&comment.body, body_width); // Use textwrap's wrap function
                let formatted_comment = formatted_body.join("\n");
                format!("{}: {}", comment.user.login, formatted_comment)
            })
            .collect();
        let comments_cell;
        if comments_text.is_empty() {
            comments_cell = Cell::from("No comments");
        } else {
            let comments_text_joined = comments_text.join("\n\n");
            let wrapped_comments = wrap(&comments_text_joined, body_width); // Wrap the joined comments
            comments_cell = Cell::from(wrapped_comments.join("\n\n"));
        }
        issue_details = Table::new(
            vec![
                Row::new(vec![Cell::from("Created at")])
                    .style(Style::default().fg(Color::LightCyan))
                    .height(1),
                Row::new(vec![Cell::from(format_date(&selected_issue.created_at.clone()))])
                    .style(Style::default().fg(Color::White))
                    .height(2),
                Row::new(vec![Cell::from("Updated at")])
                    .style(Style::default().fg(Color::LightCyan))
                    .height(1),
                Row::new(vec![Cell::from(format_date(&selected_issue.updated_at.clone()))])
                    .style(Style::default().fg(Color::White))
                    .height(2),
                Row::new(vec![Cell::from("Comments").style(Style::default().fg(Color::LightCyan))]),
                Row::new(vec![comments_cell]).style(Style::default().fg(Color::White)).height(25)
            ]
        )
            .block(Block::default().title("Details").borders(Borders::ALL))
            .widths(&[Constraint::Min(0)])
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::LightMagenta));
    } else {
        issue_details = Table::new(
            vec![
                Row::new(vec![Cell::from("Organization - Repository")])
                    .style(Style::default().fg(Color::LightCyan))
                    .height(1),
                Row::new(vec![Cell::from(org_repo_text)])
                    .style(Style::default().fg(Color::White))
                    .height(3), // Adjust the height as needed

                Row::new(vec![Cell::from("Labels")])
                    .style(Style::default().fg(Color::LightCyan))
                    .height(1),
                Row::new(
                    vec![
                        Cell::from(
                            selected_issue.labels
                                .iter()
                                .map(|l| l.name.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    ]
                )
                    .style(Style::default().fg(Color::White))
                    .height(2),

                Row::new(vec![Cell::from("Description")])
                    .style(Style::default().fg(Color::LightCyan))
                    .height(1),
                Row::new(vec![Cell::from(display_body)])
                    .style(Style::default().fg(Color::White))
                    .height(body_height.try_into().unwrap_or(1))
            ]
        )
            .block(
                Block::default()
                    .title(
                        string_to_spans(
                            format!("{} - {}", selected_issue.number, selected_issue.title)
                        )
                    )
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
            )
            .widths(&[Constraint::Min(0)])
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::LightMagenta));
    }
    (issue_list, issue_details)
}

fn string_to_spans(text: String) -> Spans<'static> {
    let span = Span::raw(text);
    Spans::from(span)
}
