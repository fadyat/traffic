use crate::api::github::RepoView;
use std::cmp::{max, min};
use chrono::DateTime;
use crate::ui::r#type::BuildType;

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
    pub fn new(
        traffic: Vec<RepoView>,
        window_size: usize,
    ) -> Self {
        let length = traffic.len();

        App {
            window_size: min(window_size, length),
            traffic: traffic.clone(),
            right_bound: length,
            left_bound: max(length as isize - window_size as isize, 0) as usize,
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

    pub fn move_window(&mut self, offset: isize) {
        let new_left = max(self.left_bound as isize + offset, 0);
        let new_right = min(self.right_bound as isize + offset, self.traffic.len() as isize);

        if new_right - new_left != self.window_size as isize {
            return;
        }

        self.left_bound = new_left as usize;
        self.right_bound = new_right as usize;
    }

    pub fn get_dataset(
        window: &[IndexedView],
        gtype: &BuildType,
    ) -> Vec<(f64, f64)> {
        window
            .iter()
            .map(|view| {
                let x = view.index as f64;
                let y = match gtype {
                    BuildType::Uniques => view.view.uniques as f64,
                    BuildType::Views => view.view.count as f64,
                };

                (x, y)
            })
            .collect::<Vec<(f64, f64)>>()
    }

    pub fn get_xbounds(window: &[IndexedView]) -> [f64; 2] {
        [
            window[0].index as f64,
            window[window.len() - 1].index as f64,
        ]
    }

    pub fn get_ybounds(
        window: &[IndexedView],
        gtype: &BuildType,
    ) -> [f64; 2] {
        window.iter().fold([0.0, 0.0], |acc, view| {
            let [mn, mx] = acc;
            let y = match gtype {
                BuildType::Uniques => view.view.uniques as f64,
                BuildType::Views => view.view.count as f64,
            };

            [mn.min(y), mx.max(y)]
        })
    }

    pub fn get_xlabels(window: &[IndexedView]) -> Vec<String> {
        let fun = |view: &IndexedView| {
            DateTime::parse_from_rfc3339(&view.view.timestamp)
                .unwrap_or_default()
                .format("%d-%m")
                .to_string()
        };

        let labels_count = 10;
        let step = window.len() / labels_count;
        (0..labels_count)
            .map(|i| {
                match i {
                    0 => fun(&window[0]),
                    _ if i == labels_count - 1 => fun(&window[window.len() - 1]),
                    _ => fun(&window[i * step]),
                }
            })
            .collect()
    }


    pub fn get_ylabels(
        window: &[IndexedView],
        gtype: &BuildType,
    ) -> Vec<String> {
        let [mn, mx] = App::get_ybounds(window, gtype);
        let step = (mx - mn) / 10.0;

        (0..10)
            .map(|i| format!("{:.0}", mn + step * i as f64))
            .collect()
    }

    pub fn expand_window(&mut self) {
        self.change_window_size(1);
    }

    pub fn shrink_window(&mut self) {
        self.change_window_size(-1);
    }

    fn change_window_size(&mut self, offset: isize) {
        let new_size = self.window_size as isize + offset;
        let stop_point = 5;

        if new_size < stop_point || new_size > self.traffic.len() as isize {
            return;
        }

        self.window_size = new_size as usize;
        self.left_bound = (self.left_bound as isize - offset) as usize;
    }
}
