use tui::{
  layout::Alignment,
  style::{Modifier, Style, Color},
  text::{Span, Spans},
  widgets::{Block, BorderType, Borders, Paragraph},
  Terminal
};


pub fn render_waiting_screen<B: tui::backend::Backend>(
  terminal: &mut tui::Terminal<B>,
) -> Result<(), Box<dyn std::error::Error>> {
  let loading_text = r#"
  _______ __________________                   ______     _______  _______  _______ _________ _______ _________ _______  _       _________
  (  ____ \\__   __/\__   __/|\     /||\     /|(  ___ \   (  ___  )(  ____ \(  ____ \\__   __/(  ____ \\__   __/(  ___  )( (    /|\__   __/
  | (    \/   ) (      ) (   | )   ( || )   ( || (   ) )  | (   ) || (    \/| (    \/   ) (   | (    \/   ) (   | (   ) ||  \  ( |   ) (
  | |         | |      | |   | (___) || |   | || (__/ /   | (___) || (_____ | (_____    | |   | (_____    | |   | (___) ||   \ | |   | |
  | | ____    | |      | |   |  ___  || |   | ||  __ (    |  ___  |(_____  )(_____  )   | |   (_____  )   | |   |  ___  || (\ \) |   | |
  | | \_  )   | |      | |   | (   ) || |   | || (  \ \   | (   ) |      ) |      ) |   | |         ) |   | |   | (   ) || | \   |   | |
  | (___) |___) (___   | |   | )   ( || (___) || )___) )  | )   ( |/\____) |/\____) |___) (___/\____) |   | |   | )   ( || )  \  |   | |
  (_______)\_______/   )_(   |/     \|(_______)|/ \___/   |/     \|\_______)\_______)\_______/\_______)   )_(   |/     \||/    )_)   )_(
    "#;

  let loading_screen_paragraph = Paragraph::new(loading_text)
      .alignment(Alignment::Center)
      .block(
          Block::default()
              .title(Span::styled("GitHub Assistant", Style::default().add_modifier(Modifier::BOLD)))
              .borders(Borders::ALL)
              .border_style(Style::default().fg(Color::White).bg(Color::DarkGray))
              .border_type(BorderType::Rounded),
      );

  terminal.draw(|f| {
      let size = f.size();
      f.render_widget(loading_screen_paragraph, size);
  })?;

  Ok(())
}
