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
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Fir {
    /// Filter impulse response
    b: Vec<f64>,
    /// Internal buffer
    buf: VecDeque<f64>,
}

impl Fir {
    /// Construct and Lanczos upsampling FIR filter with
    /// upsampling factor "p" and Lanczos parameter "a".
    pub fn lanczos(p: usize, a: usize) -> Fir {
        let n = (2 * (p * a - 1)) as isize;
        let a = a as f64;
        let pi_p = std::f64::consts::PI / p as f64;

        let b = ((-n / 2)..(n / 2))
            .map(|k| {
                if k == 0 {
                    1.0
                } else {
                    let l = pi_p * k as f64;
                    l.sin() / l * (l / a).sin() / (l / a)
                }
            })
            .collect();

        Self {
            b,
            buf: VecDeque::from(vec![0.0; n as usize + 1]),
        }
    }

    pub fn process(&mut self, input: f64) -> f64 {
        self.buf.push_back(input);
        if self.buf.len() > self.b.len() {
            self.buf.pop_front();
        };

        // Apply filter through direct convolution.
        // For filters with impulse responses less than 64 samples,
        // this is more efficient than FFT based convolution, see
        // https://ccrma.stanford.edu/~jos/ReviewFourier/FFT_Convolution_vs_Direct.html
        self.buf
            .iter()
            .zip(self.b.iter())
            .fold(0.0, |acc, (x, b)| acc + x * b)
    }
}
