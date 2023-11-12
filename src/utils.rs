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

pub fn filter_issues(issues: &Vec<ApiResponseItem>, filter: String) -> Vec<ApiResponseItem> {
  let mut filtered_issues: Vec<ApiResponseItem> = vec![];
  for issue in issues {
      if let Some(organization) = &issue.organization {
          if organization.contains(&filter) {
              filtered_issues.push(issue.clone());
          }
      }
  }
  filtered_issues
}
// filter issue list by pasing is_pr
pub fn filter_issues_by_state(issues: &Vec<ApiResponseItem>, filter_pr: bool) -> Vec<ApiResponseItem> {
  let mut filtered_issues: Vec<ApiResponseItem> = vec![];
  for issue in issues {
      if issue.is_pr == filter_pr {
          filtered_issues.push(issue.clone());
      }
  }
  filtered_issues
}

pub fn get_org_list(issues: &Vec<ApiResponseItem>) -> Vec<String> {
  let mut org_list: Vec<String> = vec![];
  for issue in issues {
      if let Some(organization) = &issue.organization {
          if !org_list.contains(organization) {
              org_list.push(organization.clone());
          }
      }
  }
  org_list
}

pub fn get_repo_list(issues: &Vec<ApiResponseItem>) -> Vec<String> {
  let mut repo_list: Vec<String> = vec![];
  for issue in issues {
      if let Some(repository) = &issue.repository {
          if !repo_list.contains(repository) {
              repo_list.push(repository.clone());
          }
      }
  }
  repo_list
}
