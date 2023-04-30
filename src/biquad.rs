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
pub struct Biquad {
    /// Numerator coefficients B0, B1, B2
    b: [f64; 3],
    /// Denominator coefficients A1, A2, A0 == 1.0
    a: [f64; 2],
    /// Previous input values
    input: [f64; 2],
    /// Previous output values
    output: [f64; 2],
}

impl Biquad {
    pub fn new(b: [f64; 3], a: [f64; 2]) -> Self {
        Self {
            b,
            a,
            input: [0.0; 2],
            output: [0.0; 2],
        }
    }

    pub fn process(&mut self, input: f64) -> f64 {
        // Biquad must use double for all calculations.
        // Otherwise, some filters may have catastrophic rounding errors.
        let output = self.b[0] * input
            + self.b[1] * self.input[0]
            + self.b[2] * self.input[1]
            - self.a[0] * self.output[0]
            - self.a[1] * self.output[1];

        self.input[1] = self.input[0];
        self.input[0] = input;
        self.output[1] = self.output[0];
        self.output[0] = output;
        output
    }
}
