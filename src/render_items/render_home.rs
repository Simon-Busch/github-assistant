use tui::{
  layout::Alignment,
  style::{Color, Style, Modifier},
  text::{Span, Spans},
  widgets::{Block, BorderType, Borders, Paragraph}
};

pub fn render_home<'a>(opened: &i32, closed: &i32, review: &i32, username: &String) -> Paragraph<'a> {
  let home = Paragraph::new(vec![
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::raw(format!("Welcome to your GitHub assistant, {} ⭐️", username))]),
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
      Spans::from(vec![Span::raw(format!(
          "{} To review",
          review,
      ))]),
      Spans::from(vec![Span::raw("")]),

      Spans::from(vec![Span::raw("")]),

      Spans::from(vec![Span::styled("Navigate:", Style::default().add_modifier(Modifier::BOLD))]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::styled("Up: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw("Move up in the list")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::styled("Down: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw("Move down in the list")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::styled("Right: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw("Display comments for the selected issue/PR")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::styled("Left: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw("Hide comments for the selected issue/PR")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::styled("Commands",Style::default().add_modifier(Modifier::BOLD))]),
      Spans::from(vec![Span::raw("")]),

      Spans::from(vec![Span::styled("CTRL + a : ", Style::default().add_modifier(Modifier::BOLD)), Span::raw("show assignment")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::styled("CTRL + c: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw("show closed")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::styled("CTRL + h: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw("home")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::styled("q: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw("close app")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::styled("CTRL + r: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw("reload content")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::styled("ENTER: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw("open the issue in the browser")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::styled("CTRL + p: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw("show actions")]),
      Spans::from(vec![Span::raw("")]),
      Spans::from(vec![Span::styled("1: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw("close issue")]),
      Spans::from(vec![Span::raw("")]),

      Spans::from(vec![Span::raw("")]),

      Spans::from(vec![Span::raw("")]),
    ])
    .alignment(Alignment::Center)
    .block(
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Home")
        .border_type(BorderType::Plain)
        .border_type(BorderType::Rounded),
  );
  home
}
