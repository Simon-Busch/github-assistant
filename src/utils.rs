use tui::layout::Rect;

pub fn centered_rect(width: u16, height: u16, parent: Rect) -> Rect {
    let parent_width = parent.width;
    let parent_height = parent.height;

    let x = (parent_width - width) / 2;
    let y = (parent_height - height) / 2;

    Rect::new(x, y, width, height)
}
