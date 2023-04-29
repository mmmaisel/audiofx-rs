/******************************************************************************\
    audiofx-rs
    Copyright (C) 2023 Max Maisel

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
\******************************************************************************/
use crate::conversion::Conversion;
use crate::error::Error;
use crate::filters::{lag1::Lag1, mov_max::MovMax, mov_rms::MovRms, Filter};
use crate::frame::FrameIterator;
use crate::progress::Progress;
use hound::{WavReader, WavWriter};
use std::collections::VecDeque;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum PeakDetector {
    /// Sliding maximum detector
    Peak,
    /// RMS detector,
    Rms,
}

#[derive(Debug, clap::Args)]
pub struct Settings {
    /// Peak detector to use
    detector: PeakDetector,
    /// Compress stereo channels independently
    #[arg(short)]
    stereo_indep: bool,
    /// Compressor threshold in dB
    threshold_db: f64,
    /// Compression ratio. Must be greater than one.
    ratio: f64,
    /// Compressor knee width in dB.
    knee_width_db: f64,
    /// Compressor attack time in seconds.
    attack_time: f64,
    /// Compressor release time in seconds.
    release_time: f64,
    /// Compressor lookahead time in seconds.
    lookahead_time: f64,
    /// Compressor hold time in seconds.
    hold_time: f64,
    /// Compressor output gain in dB.
    output_gain_db: f64,
}

impl Settings {
    pub fn compress<R, W>(
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

        // TODO: handle channel independent flag
        let mut compressor = Compressor::new(
            spec.sample_rate as f64,
            spec.channels as usize,
            &self,
        );

        Self::compensate_initial_condition(
            &mut compressor,
            input,
            spec.channels,
            spec.sample_rate as f64,
            self.attack_time,
        )?;
        input.seek(0)?;

        let mut counter = 0;
        let mut progress =
            Progress::new(duration as usize, "Compressing sample");
        let mut frames = FrameIterator::new(input.samples_f32(), spec.channels);
        while let Some(frame) = frames.next() {
            progress.next();
            match frame {
                Ok(frame) => {
                    let proc = compressor.process(frame)?;
                    // Compensate filter latency
                    if counter > compressor.latency() {
                        for sample in proc {
                            output.write_sample(sample)?;
                        }
                    } else {
                        counter += 1;
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }

        // Drain processing pipeline
        let padding = vec![0.0; spec.channels as usize];
        for _ in 1..compressor.latency() {
            let proc = compressor.process(&padding)?;
            for sample in proc {
                output.write_sample(sample)?;
            }
        }

        Ok(())
    }

    fn compensate_initial_condition<R>(
        compressor: &mut Compressor,
        input: &mut WavReader<R>,
        channels: u16,
        fs: f64,
        attack_time: f64,
    ) -> Result<(), Error>
    where
        R: std::io::Read + std::io::Seek,
    {
        let mut counter = 0;
        let settling_len = Lag1::settling_len(fs, attack_time);
        let mut frames = FrameIterator::new(input.samples_f32(), channels);
        while let Some(frame) = frames.next() {
            if counter > settling_len {
                break;
            }
            match frame {
                Ok(frame) => {
                    compressor.process_initial(frame)?;
                    counter += 1;
                }
                Err(e) => return Err(e.into()),
            }
        }

        Ok(())
    }
}

/// EBUR128 loudness analyzer
#[derive(Debug)]
pub struct Compressor {
    /// Number of channels
    channels: usize,
    /// Envelope detector preprocessing filter
    preprocessor: Box<dyn Filter>,
    /// Lookahead in samples, this is also the filter latency.
    lookahead: usize,
    /// Main envelope detection filter.
    envelope: Lag1,
    /// Filter input data buffer.
    buffer: VecDeque<Vec<f32>>,
    /// Compressor threshold in dB.
    threshold_db: f64,
    /// Compression ratio.
    ratio: f64,
    /// Compressor knee width in dB.
    knee_width_db: f64,
    /// Compressor output gin in dB.
    output_gain_db: f64,
}

impl Compressor {
    pub fn new(fs: f64, channels: usize, settings: &Settings) -> Self {
        let lookahead = (settings.lookahead_time * fs) as usize;
        let hold = (settings.hold_time * fs) as usize;
        let preprocessor = match settings.detector {
            PeakDetector::Peak => {
                Box::new(MovMax::new((lookahead + hold) * channels))
                    as Box<dyn Filter>
            }
            PeakDetector::Rms => {
                Box::new(MovRms::new(2.0, (lookahead + hold) * channels))
                    as Box<dyn Filter>
            }
        };

        Self {
            channels,
            preprocessor,
            lookahead,
            envelope: Lag1::new(
                match settings.detector {
                    PeakDetector::Peak => 1.0,
                    PeakDetector::Rms => {
                        1.0 + (settings.attack_time / 30.0).exp()
                    }
                },
                settings.attack_time,
                settings.release_time,
                fs,
            ),
            buffer: VecDeque::from(vec![
                vec![0.0; channels as usize];
                lookahead
            ]),
            threshold_db: settings.threshold_db,
            ratio: settings.ratio,
            knee_width_db: settings.knee_width_db,
            output_gain_db: settings.output_gain_db,
        }
    }

    pub fn latency(&self) -> usize {
        self.lookahead
    }

    // TODO: add test to validate effect
    pub fn process(&mut self, frame: &Vec<f32>) -> Result<Vec<f32>, Error> {
        if self.ratio <= 1.0 {
            return Err(Error::InvalidArgument(
                "Ratio must be greater than one.".into(),
            ));
        }
        if frame.len() != self.channels {
            return Err(Error::InvalidFrame);
        }

        let mut preproc = 0.0;
        for x in frame {
            preproc = self.preprocessor.process(*x as f64);
        }

        let envelope = self.envelope.process(preproc);
        let gain = self.gain(envelope) as f32;
        // TODO: avoid re-construction of inner vectors
        let current_frame = self.buffer.pop_front().unwrap();
        self.buffer.push_back(frame.to_owned());

        Ok(current_frame.iter().map(|x| x * gain).collect())
    }

    pub fn process_initial(&mut self, frame: &Vec<f32>) -> Result<(), Error> {
        if frame.len() != self.channels {
            return Err(Error::InvalidFrame);
        }
        let mut preproc = 0.0;
        for x in frame {
            preproc = self.preprocessor.process(*x as f64);
        }
        self.envelope.process(preproc);

        Ok(())
    }

    fn gain(&self, env: f64) -> f64 {
        let env_db = 20.0 * env.log10();
        // Prevent NaN propagation with a very low dB value if envelope is zero.
        let env_db = if env_db.is_nan() { -200.0 } else { env_db };

        let knee_cond = 2.0 * (env_db - self.threshold_db);

        if knee_cond < self.knee_width_db {
            // Below threshold: only apply make-up gain
            10.0_f64.powf(self.output_gain_db / 20.0)
        } else if knee_cond >= self.knee_width_db {
            // Above threshold: apply compression and make-up gain
            10.0_f64.powf(
                (self.threshold_db
                    + (env_db - self.threshold_db) / self.ratio
                    + self.output_gain_db
                    - env_db)
                    / 20.0,
            )
        } else {
            // Within knee: apply interpolated compression and make-up gain
            10.0_f64.powf(
                ((1.0 / self.ratio - 1.0)
                    * 2.0_f64.powf(
                        env_db - self.threshold_db + self.knee_width_db / 2.0,
                    ))
                    / 20.0,
            )
        }
    }
}
