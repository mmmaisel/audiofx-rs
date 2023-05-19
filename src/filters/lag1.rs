/******************************************************************************\
    wavehacker
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

/// 1st order lag filter
#[derive(Clone, Debug)]
pub struct Lag1 {
    /// Filter gain
    gain: f64,
    /// Time constant derived factor for rising slopes
    rise_factor: f64,
    /// Time constant derived factor for falling slopes
    fall_factor: f64,
    /// Internal filter level
    level: f64,
}

impl Lag1 {
    pub fn new(gain: f64, tau_rise: f64, tau_fall: f64, fs: f64) -> Self {
        // Limit tau to sample interval
        let tau_rise = tau_rise.max(1.0 / fs);
        let tau_fall = tau_fall.max(1.0 / fs);

        Self {
            gain,
            rise_factor: 1.0 / (tau_rise * fs),
            fall_factor: 1.0 / (tau_fall * fs),
            level: 0.0,
        }
    }

    /// Calculate compensation value for filter initial condition.
    pub fn process_initial(&mut self, input: f64, gain: f64) -> f64 {
        if input >= self.level {
            self.level += gain * self.rise_factor * (input - self.level);
        } else {
            self.level += self.fall_factor * (input - self.level);
        }
        self.level
    }

    /// Reset filter level to given value.
    pub fn reset(&mut self, level: f64) {
        self.level = level;
    }

    /// Number of samples the filter approximately needs to settle.
    pub fn settling_len(fs: f64, tau_rise: f64) -> usize {
        (fs * tau_rise.sqrt()) as usize
    }
}

impl Filter for Lag1 {
    fn process(&mut self, input: f64) -> f64 {
        if input >= self.level {
            self.level += self.rise_factor * (input - self.level);
        } else {
            self.level += self.fall_factor * (input - self.level);
        }
        self.level * self.gain
    }
}
