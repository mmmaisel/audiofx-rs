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
use crate::error::Error;
use crate::frame::FrameIterator;
use crate::progress::Progress;
use hound::WavReader;
use kahan::KahanSum;

#[derive(Debug, Clone, clap::Args)]
pub struct Settings {
    /// Analyze multiple channels independently
    #[arg(short)]
    channel_independent: bool,
}

impl Settings {
    pub fn new(channel_independent: bool) -> Self {
        Self {
            channel_independent,
        }
    }

    pub fn analyze<R>(
        &self,
        input: &mut WavReader<R>,
    ) -> Result<Vec<f64>, Error>
    where
        R: std::io::Read,
    {
        let spec = input.spec();
        let duration = input.duration();
        let mut analyzer = if self.channel_independent {
            vec![Rms::new(1); spec.channels as usize]
        } else {
            vec![Rms::new(spec.channels as usize)]
        };

        let mut progress = Progress::new(duration as usize, "Analyzing sample");
        let mut frames =
            FrameIterator::new(input.samples::<f32>(), spec.channels);
        while let Some(frame) = frames.next() {
            progress.next();
            match frame {
                Ok(frame) => {
                    if self.channel_independent {
                        for (i, x) in frame.iter().enumerate() {
                            match analyzer.get_mut(i) {
                                Some(a) => a.process(&vec![*x])?,
                                None => return Err(Error::InvalidFrame),
                            }
                        }
                    } else {
                        analyzer[0].process(frame)?
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }

        Ok(analyzer.iter().map(|x| x.rms()).collect())
    }
}

/// RMS analyzer
#[derive(Debug, Clone)]
pub struct Rms {
    /// Number of channels
    channels: usize,
    /// Sample counter for averaging operation
    counter: usize,
    /// Accumulated RMS
    sq_sum: KahanSum<f64>,
}

impl Rms {
    pub fn new(channels: usize) -> Self {
        Self {
            channels,
            counter: 0,
            sq_sum: KahanSum::new(),
        }
    }

    /// Analyze frame of samples and add it to the cumulative RMS.
    pub fn process(&mut self, frame: &Vec<f32>) -> Result<(), Error> {
        if frame.len() != self.channels {
            return Err(Error::InvalidFrame);
        }

        for sample in frame {
            self.sq_sum += (*sample as f64) * (*sample as f64);
        }
        self.counter += 1;

        Ok(())
    }

    /// Returns the root-mean-square value of the processed audio in
    /// linear units.
    pub fn rms(&self) -> f64 {
        (self.sq_sum.sum() / ((self.channels * self.counter) as f64)).sqrt()
    }
}
