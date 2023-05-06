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
use hound::{Error, SampleFormat, WavReader, WavSamples};

pub struct IntoF32Samples<'a, R> {
    samples: WavSamples<'a, R, i32>,
    amplitude: f32,
}

impl<'a, R> IntoF32Samples<'a, R> {
    pub fn new(samples: WavSamples<'a, R, i32>, bits: u16) -> Self {
        Self {
            samples,
            amplitude: 2.0_f32.powi(1 - (bits as i32)),
        }
    }
}

impl<'a, R> Iterator for IntoF32Samples<'a, R>
where
    R: std::io::Read,
{
    type Item = Result<f32, Error>;
    fn next(&mut self) -> Option<Result<f32, Error>> {
        self.samples
            .next()
            .map(|t| t.map(|x| (x as f32) * self.amplitude))
    }
}

pub trait Conversion<R> {
    fn samples_f32(
        &mut self,
    ) -> Box<dyn Iterator<Item = Result<f32, Error>> + '_>;
}

impl<R> Conversion<R> for WavReader<R>
where
    R: std::io::Read,
{
    fn samples_f32(
        &mut self,
    ) -> Box<dyn Iterator<Item = Result<f32, Error>> + '_> {
        let spec = self.spec();
        match spec.sample_format {
            SampleFormat::Float => Box::new(self.samples::<f32>()),
            SampleFormat::Int => Box::new(IntoF32Samples::new(
                self.samples::<i32>(),
                spec.bits_per_sample,
            )),
        }
    }
}
