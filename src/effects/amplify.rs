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
use crate::conversion::Conversion;
use crate::error::Error;
use crate::frame::FrameIterator;
use crate::progress::Progress;
use hound::{WavReader, WavWriter};

#[derive(Debug, Clone, clap::Args)]
pub struct Settings {
    /// Gain in dB to apply to each channel.
    gain_db: Vec<f32>,
}

impl Settings {
    pub fn new(gain_db: Vec<f32>) -> Self {
        Self { gain_db }
    }

    pub fn amplify<R, W>(
        &self,
        input: &mut WavReader<R>,
        output: &mut WavWriter<W>,
    ) -> Result<(), Error>
    where
        R: std::io::Read + std::io::Seek,
        W: std::io::Write + std::io::Seek,
    {
        let spec = input.spec();
        let duration = input.duration();
        let mut progress =
            Progress::new(duration as usize, "Processing sample");
        let mut frames = FrameIterator::new(input.samples_f32(), spec.channels);

        let gain: Vec<f32> = if usize::from(spec.channels) == self.gain_db.len()
        {
            self.gain_db
                .iter()
                .map(|x| 10.0_f32.powf(x / 20.0))
                .collect()
        } else if self.gain_db.len() == 1 {
            std::iter::repeat(10.0_f32.powf(self.gain_db[0] / 20.0))
                .take(spec.channels as usize)
                .collect()
        } else {
            return Err(Error::InvalidArgument(
                "gain.len() must be 1 or number of channels.".into(),
            ));
        };

        while let Some(frame) = frames.next() {
            progress.next();
            match frame {
                Ok(frame) => {
                    for (i, sample) in frame.iter().enumerate() {
                        output.write_sample(sample * gain[i])?;
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }

        Ok(())
    }
}
