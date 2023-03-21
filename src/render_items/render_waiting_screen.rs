use tui::{
  layout::Alignment,
  style::{Modifier, Style},
  text::{Span, Spans},
  widgets::{Block, BorderType, Borders, Paragraph},
  Terminal
};


pub fn render_waiting_screen<B: tui::backend::Backend>(
  terminal: &mut Terminal<B>,
) -> Result<(), Box<dyn std::error::Error>> {
  let loading_screen = vec![
      Spans::from(Span::styled("Loading GitHub data...", Style::default().add_modifier(Modifier::BOLD))),
      Spans::from(Span::raw("")),
      Spans::from(Span::styled("Please wait while we fetch the data from GitHub...", Style::default())),
  ];

  terminal.draw(|f| {
      let size = f.size();
      let loading_screen_paragraph = Paragraph::new(loading_screen.clone())
          .alignment(Alignment::Center)
          .block(
              Block::default()
                  .title(Span::styled("GitHub Assistant", Style::default().add_modifier(Modifier::BOLD)))
                  .borders(Borders::ALL).border_type(BorderType::Rounded),
          );
      f.render_widget(loading_screen_paragraph, size);
  })?;

  Ok(())
}
