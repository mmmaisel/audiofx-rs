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
use crate::fir::Fir;
use crate::frame::FrameIterator;
use crate::progress::Progress;
use hound::WavReader;

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
            vec![TruePeak::new(1); spec.channels as usize]
        } else {
            vec![TruePeak::new(spec.channels as usize)]
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

        Ok(analyzer.iter().map(|x| x.true_peak()).collect())
    }
}

/// True peak analyzer
#[derive(Debug, Clone)]
pub struct TruePeak {
    /// Number of channels
    channels: usize,
    /// Maximum true peak value
    true_peak: f64,
    /// Upsampling filters
    filter: Vec<Fir>,
}

impl TruePeak {
    pub fn new(channels: usize) -> Self {
        Self {
            channels,
            true_peak: 0.0,
            filter: vec![Fir::lanczos(4, 3); channels],
        }
    }

    /// Analyze frame of samples and update true peak value.
    pub fn process(&mut self, frame: &Vec<f32>) -> Result<(), Error> {
        if frame.len() != self.channels {
            return Err(Error::InvalidFrame);
        }

        for (i, sample) in frame.iter().enumerate() {
            // Upsample by factor four.
            let val = self.filter[i].process(*sample as f64);
            self.true_peak = self.true_peak.max(val.abs());

            for _ in 1..4 {
                let val = self.filter[i].process(0.0);
                self.true_peak = self.true_peak.max(val.abs());
            }
        }

        Ok(())
    }

    /// Returns the detected true peak.
    pub fn true_peak(&self) -> f64 {
        self.true_peak
    }
}
