use crate::api::github;

pub fn merge_views(
    old: Vec<github::RepoView>,
    new: Vec<github::RepoView>,
) -> Vec<github::RepoView> {
    if old.is_empty() {
        return new;
    }

    let mut merged = old.clone();
    let top_idx = merged
        .iter()
        .rposition(|view| view.timestamp == new[0].timestamp);

    if let Some(idx) = top_idx {
        merged.truncate(idx);
    }

    merged.extend_from_slice(&new);
    merged
}
