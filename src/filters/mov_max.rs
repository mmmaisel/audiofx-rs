/******************************************************************************\
    audiofx-rs
    Copyright (C) 2023 Max Maisel

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
\******************************************************************************/
use super::Filter;
use std::collections::VecDeque;

/// Moving maximum filter
#[derive(Clone, Debug)]
pub struct MovMax {
    /// Sliding window length
    window_length: usize,
    /// Sliding window buffer
    window: VecDeque<f64>,
    /// Intermediate maxima values
    maxes: VecDeque<f64>,
}

impl MovMax {
    pub fn new(window_length: usize) -> Self {
        Self {
            window_length,
            window: VecDeque::with_capacity(window_length),
            maxes: VecDeque::with_capacity(window_length),
        }
    }
}

impl Filter for MovMax {
    fn process(&mut self, input: f64) -> f64 {
        while self.maxes.back().map_or(false, |x| x < &input) {
            self.maxes.pop_back();
        }
        self.window.push_back(input);
        self.maxes.push_back(input);

        if self.window.len() > self.window_length {
            if self.maxes.front() == self.window.front() {
                self.maxes.pop_front();
            }
            self.window.pop_front();
        }
        *self.maxes.front().unwrap_or(&0.0)
    }
}

#[test]
fn test_mov_max() {
    let mut mov_max = MovMax::new(4);
    let data = vec![
        5.6, 7.3, 9.9, 5.7, 3.3, 5.1, 2.8, 6.4, 9.9, 4.8, 0.9, 4.7, 9.0, 7.4,
        8.2, 9.1, 7.4, 0.8, 2.6, 3.7, 0.5, 4.3, 7.6, 7.6, 4.3, 4.1, 1.2, 1.7,
    ];
    let expected = vec![
        5.6, 7.3, 9.9, 9.9, 9.9, 9.9, 5.7, 6.4, 9.9, 9.9, 9.9, 9.9, 9.0, 9.0,
        9.0, 9.1, 9.1, 9.1, 9.1, 7.4, 3.7, 4.3, 7.6, 7.6, 7.6, 7.6, 7.6, 4.3,
    ];

    let maxes: Vec<f64> = data.iter().map(|x| mov_max.process(*x)).collect();
    assert_eq!(maxes, expected);
}
