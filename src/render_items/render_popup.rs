use tui::{
    Frame,
    backend::Backend,
    widgets::{ ListItem, List, Block, Borders, Clear },
    style::{ Modifier, Style, Color },
};

use crate::utils::centered_rect;

fn convert_to_list_items(items: &[String], selected_issue_index: Option<usize>) -> Vec<ListItem> {
  items.iter().enumerate().map(|(index, item)| {
      let mut list_item = ListItem::new(item.clone());
      if Some(index) == selected_issue_index {
          list_item = list_item.style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow));
      }
      list_item
  }).collect()
}

pub fn render_popup(rect: &mut Frame<impl Backend>, items: Vec<String>, title: String, selected_issue_index: Option<usize>) {
    let default_items = vec![
        "  1 - Close issue".to_string(),
        "  2 - Choose organisation".to_string(),
        "  3 - Choose repository".to_string()
    ];
    let items = if items.is_empty() { &default_items } else { &items };
    let list_items = convert_to_list_items(&items, selected_issue_index);
    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
        .highlight_symbol(">> ");

    let popup = Block::default()
        .borders(Borders::ALL)
        .title("Select an action")
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));

    let popup_chunk = centered_rect(50, 20, rect.size()); // Adjust the width and height values as needed

    // Render the list on top of the existing widgets
    rect.render_widget(popup, popup_chunk);
    rect.render_widget(Clear, popup_chunk);
    rect.render_widget(list, popup_chunk);
}
