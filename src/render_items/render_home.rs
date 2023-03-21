use tui::{
  layout::Alignment,
  style::{Color, Style},
  text::{Span, Spans},
  widgets::{Block, BorderType, Borders, Paragraph}
};

pub fn render_home<'a>(opened: &i32, closed: &i32) -> Paragraph<'a> {
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
        .border_type(BorderType::Plain)
        .border_type(BorderType::Rounded),
  );
  home
}
