use tui::{Frame, backend::Backend, widgets::{Block, Borders, Clear, Paragraph}, style::{Style, Color}, layout::Rect};

use crate::utils::centered_rect;

pub fn render_loading_popup(rect: &mut Frame<impl Backend>, area: Rect) {
  let loading_text = r#"
  _______ __________________                   ______     _______  _______  _______ _________ _______ _________ _______  _       _________
  (  ____ \\__   __/\__   __/|\     /||\     /|(  ___ \   (  ___  )(  ____ \(  ____ \\__   __/(  ____ \\__   __/(  ___  )( (    /|\__   __/
  | (    \/   ) (      ) (   | )   ( || )   ( || (   ) )  | (   ) || (    \/| (    \/   ) (   | (    \/   ) (   | (   ) ||  \  ( |   ) (
  | |         | |      | |   | (___) || |   | || (__/ /   | (___) || (_____ | (_____    | |   | (_____    | |   | (___) ||   \ | |   | |
  | | ____    | |      | |   |  ___  || |   | ||  __ (    |  ___  |(_____  )(_____  )   | |   (_____  )   | |   |  ___  || (\ \) |   | |
  | | \_  )   | |      | |   | (   ) || |   | || (  \ \   | (   ) |      ) |      ) |   | |         ) |   | |   | (   ) || | \   |   | |
  | (___) |___) (___   | |   | )   ( || (___) || )___) )  | )   ( |/\____) |/\____) |___) (___/\____) |   | |   | )   ( || )  \  |   | |
  (_______)\_______/   )_(   |/     \|(_______)|/ \___/   |/     \|\_______)\_______)\_______/\_______)   )_(   |/     \||/    )_)   )_(

.______       _______  __        ______        ___       _______   __  .__   __.   _______
|   _  \     |   ____||  |      /  __  \      /   \     |       \ |  | |  \ |  |  /  _____|
|  |_)  |    |  |__   |  |     |  |  |  |    /  ^  \    |  .--.  ||  | |   \|  | |  |  __
|      /     |   __|  |  |     |  |  |  |   /  /_\  \   |  |  |  ||  | |  . `  | |  | |_ |
|  |\  \----.|  |____ |  `----.|  `--'  |  /  _____  \  |  '--'  ||  | |  |\   | |  |__| |
| _| `._____||_______||_______| \______/  /__/     \__\ |_______/ |__| |__| \__|  \______|

  "#;

  let popup = Block::default()
      .borders(Borders::ALL)
      .title("Reloading all issues...")
      .style(Style::default().fg(Color::White).bg(Color::DarkGray));

  let popup_paragraph = Paragraph::new(loading_text)
      .style(Style::default().fg(Color::White).bg(Color::DarkGray));

  // Render the popup directly within the specified area
  rect.render_widget(popup, area);
  rect.render_widget(popup_paragraph, area);
}
