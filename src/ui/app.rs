use std::cmp::max;
use crate::gh_client::RepoView;

pub struct App {
    traffic: Vec<RepoView>,
    window_size: usize,
    left_bound: usize,
    right_bound: usize,
}

pub struct IndexedView {
    pub index: usize,
    pub view: RepoView,
}

impl App {
    pub fn new(traffic: Vec<RepoView>) -> Self {
        let length = traffic.len().clone();
        let window_size = 14;

        App {
            window_size,
            traffic: traffic.clone(),
            right_bound: length,
            left_bound: max(length - window_size, 0),
        }
    }

    pub fn get_window(&self) -> Vec<IndexedView> {
        let slice = &self.traffic[self.left_bound..self.right_bound];

        slice.iter()
            .enumerate()
            .map(|(i, view)| IndexedView {
                index: i + self.left_bound,
                view: view.clone(),
            })
            .collect::<Vec<IndexedView>>()
    }
}
