use tui::{Frame, backend::Backend, widgets::{Block, Borders, Clear, Paragraph}, style::{Style, Color}, text::Text};

use crate::utils::centered_rect;

pub fn render_error(rect: &mut Frame<impl Backend>, message: &str) {
  let popup = Block::default()
      .borders(Borders::ALL)
      .title("Message")
      .style(Style::default().fg(Color::White).bg(Color::DarkGray))
      .borders(Borders::ALL);

  let paragraph = Paragraph::new(Text::from(message))
      .style(Style::default().fg(Color::White));

  let popup_chunk = centered_rect(35, 10, rect.size());

  rect.render_widget(popup, popup_chunk);
  rect.render_widget(Clear, popup_chunk);
  rect.render_widget(paragraph, popup_chunk);
}
