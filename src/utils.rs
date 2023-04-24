use tui::{layout::Rect, widgets::ListState};

use crate::structs::ApiResponseItem;

pub fn centered_rect(width: u16, height: u16, parent: Rect) -> Rect {
    let parent_width = parent.width;
    let parent_height = parent.height;

    let x = (parent_width - width) / 2;
    let y = (parent_height - height) / 2;

    Rect::new(x, y, width, height)
}

pub fn get_current_state_and_list<'a>(
    active_open: bool,
    is_pr_review: bool,
    issue_list_state_open: &'a mut ListState,
    issue_list_state_closed: &'a mut ListState,
    issue_list_state_to_review: &'a mut ListState,
    issues_list_open: &'a Vec<ApiResponseItem>,
    issues_list_closed: &'a Vec<ApiResponseItem>,
    assigned_pr_list: &'a Vec<ApiResponseItem>,
) -> (&'a mut ListState, &'a Vec<ApiResponseItem>) {
    if is_pr_review {
        (issue_list_state_to_review, assigned_pr_list)
    } else if active_open {
        (issue_list_state_open, issues_list_open)
    } else {
        (issue_list_state_closed, issues_list_closed)
    }
}

pub fn move_selection(state: &mut ListState, items: &Vec<ApiResponseItem>, delta: isize) {
    if let Some(selected) = state.selected() {
        let next = (selected as isize + delta).max(0).min((items.len() - 1) as isize);
        state.select(Some(next as usize));
    }
}
