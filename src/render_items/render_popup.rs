use tui::{Frame, backend::Backend, widgets::{ListItem, List, Block, Borders, Clear}, style::{Modifier, Style, Color}};

use crate::utils::centered_rect;

pub fn render_popup(rect: &mut Frame<impl Backend>, items: Vec<ListItem>) {
  let list = List::new(items)
      .block(
          Block::default()
              .borders(Borders::ALL)
              .title("Actions"),
      )
      .highlight_style(Style::default().add_modifier(Modifier::BOLD))
      .highlight_symbol(">> ");

  let popup = Block::default()
      .borders(Borders::ALL)
      .title("Select an action")
      .style(Style::default().fg(Color::White).bg(Color::DarkGray));

  let popup_chunk = centered_rect(25, 10, rect.size()); // Adjust the width and height values as needed

  // Render the list on top of the existing widgets
  rect.render_widget(popup, popup_chunk);
  rect.render_widget(Clear, popup_chunk);
  rect.render_widget(list, popup_chunk);
}
