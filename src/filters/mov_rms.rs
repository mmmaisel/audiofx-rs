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

/// Moving RMS filter
#[derive(Clone, Debug)]
pub struct MovRms {
    /// Filter gain
    gain: f64,
    /// Internal accumulator
    acc: f64,
    /// Insertion counter
    counter: usize,
    /// Window buffer for refreshing
    buffer: VecDeque<f64>,
}

impl MovRms {
    /// Internal accumulator refresh interval
    const REFRESH_INTERVAL: usize = 1048576; // 1 MB

    pub fn new(gain: f64, window_length: usize) -> Self {
        Self {
            gain,
            acc: 0.0,
            counter: 0,
            buffer: VecDeque::from(vec![0.0; window_length]),
        }
    }

    fn refresh(&mut self) {
        self.acc = self.buffer.iter().sum();
        self.counter = 0;
    }
}

impl Filter for MovRms {
    fn process(&mut self, input: f64) -> f64 {
        let sq_input = input * input;
        let old_input = self.buffer.pop_front().unwrap_or(0.0);
        self.buffer.push_back(sq_input);

        if self.counter > Self::REFRESH_INTERVAL {
            // Recalculate sum from window buffer to avoid accumulation of
            // rounding errors.
            self.refresh();
        } else {
            self.counter += 1;
            self.acc -= old_input;
            self.acc += sq_input;
        }

        // Refresh if there are severe rounding errors so that acc is negative.
        if self.acc < 0.0 {
            self.refresh();
        }

        self.gain * (self.acc / (self.buffer.len() as f64)).sqrt()
    }
}

#[test]
fn test_mov_rms() {
    let mut mov_rms = MovRms::new(1.0, 4);
    let data = vec![1.0, 0.0, -1.0, 0.0, 0.5, 0.0, -0.5, 0.0];

    let rms: Vec<f64> = data.iter().map(|x| mov_rms.process(*x)).collect();

    assert_eq!(
        rms,
        vec![
            0.5,
            0.5,
            0.7071067811865476,
            0.7071067811865476,
            0.5590169943749475,
            0.5590169943749475,
            0.3535533905932738,
            0.3535533905932738
        ]
    );
    mov_rms.refresh();
    assert_eq!(mov_rms.acc, 0.5);
}
