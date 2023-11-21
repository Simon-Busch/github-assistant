use tui::{
    layout::Alignment,
    style::{ Modifier, Style, Color },
    text::Span,
    widgets::{ Block, BorderType, Borders, Paragraph },
};

pub fn render_waiting_screen<B: tui::backend::Backend>(
    terminal: &mut tui::Terminal<B>
) -> Result<(), Box<dyn std::error::Error>> {
    let loading_text =
        r#"

  ██████╗ ██╗████████╗██╗  ██╗██╗   ██╗██████╗
  ██╔════╝ ██║╚══██╔══╝██║  ██║██║   ██║██╔══██╗
  ██║  ███╗██║   ██║   ███████║██║   ██║██████╔╝
  ██║   ██║██║   ██║   ██╔══██║██║   ██║██╔══██╗
  ╚██████╔╝██║   ██║   ██║  ██║╚██████╔╝██████╔╝
   ╚═════╝ ╚═╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚═════╝

   █████╗ ███████╗███████╗██╗███████╗████████╗ █████╗ ███╗   ██╗████████╗
  ██╔══██╗██╔════╝██╔════╝██║██╔════╝╚══██╔══╝██╔══██╗████╗  ██║╚══██╔══╝
  ███████║███████╗███████╗██║███████╗   ██║   ███████║██╔██╗ ██║   ██║
  ██╔══██║╚════██║╚════██║██║╚════██║   ██║   ██╔══██║██║╚██╗██║   ██║
  ██║  ██║███████║███████║██║███████║   ██║   ██║  ██║██║ ╚████║   ██║
  ╚═╝  ╚═╝╚══════╝╚══════╝╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═══╝   ╚═╝

      "#;

    let loading_screen_paragraph = Paragraph::new(loading_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(
                    Span::styled("GitHub Assistant", Style::default().add_modifier(Modifier::BOLD))
                )
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White).bg(Color::DarkGray))
                .border_type(BorderType::Rounded)
        );

    terminal.draw(|f| {
        let size = f.size();
        f.render_widget(loading_screen_paragraph, size);
    })?;

    Ok(())
}
