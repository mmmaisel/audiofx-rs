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
use crate::filters::{biquad::Biquad, Filter};
use crate::frame::FrameIterator;
use crate::progress::Progress;
use hound::WavReader;
use std::collections::VecDeque;

#[derive(Debug, Clone, clap::Args)]
pub struct Settings {
    /// Analyze multiple channels independently
    #[arg(short)]
    channel_independent: bool,
    /// Do not normalize result to stereo.
    /// You should only use this flag if you have to be strictly
    /// EBU R128 compliant.
    #[arg(short)]
    strict_ebur128: bool,
}

impl Settings {
    pub fn new(channel_independent: bool, strict_ebur128: bool) -> Self {
        Self {
            channel_independent,
            strict_ebur128,
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
            vec![
                Loudness::new(spec.sample_rate as f64, 1);
                spec.channels as usize
            ]
        } else {
            vec![Loudness::new(
                spec.sample_rate as f64,
                spec.channels as usize,
            )]
        };

        let mut progress = Progress::new(duration as usize, "Analyzing sample");
        let mut frames = FrameIterator::new(input.samples_f32(), spec.channels);
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

        let mut loudness = analyzer
            .iter_mut()
            .map(|x| {
                x.finalize()?;
                Ok(x.integrative_loudness())
            })
            .collect::<Result<Vec<f64>, Error>>()?;
        if !self.strict_ebur128 && !self.channel_independent {
            loudness = loudness
                .iter()
                .map(|x| 2.0 * x / spec.channels as f64)
                .collect();
        }

        Ok(loudness)
    }
}

/// EBUR128 loudness analyzer
#[derive(Debug, Clone)]
pub struct Loudness {
    /// Number of channels
    channels: usize,
    /// Weighting channels, one HSF/HPF pair for every channel.
    filter: Vec<[Biquad; 2]>,
    /// Working ringbuffer
    buffer: VecDeque<f64>,
    /// Sample counter for overlap detection
    counter: usize,
    /// Size of one block in samples
    block_size: usize,
    /// Overlap of the blocks in samples
    block_overlap: usize,
    /// Loudness histogram
    histogram: Vec<usize>,
}

impl Loudness {
    /// Histogram bin count, [-70; +8] dB in 0.01 dB steps
    const BIN_COUNT: usize = 7801;
    /// Maximum mean-square value of a block.
    const Z_MAX: f64 = 8.0;
    /// EBU R128 absolute threshold
    const GAMMA_A: f64 = (-70.0 + 0.691) / 10.0;

    /// Channel weights according to EBU R128
    const CHANNEL_WEIGHT: [f64; 8] = [1.0, 1.0, 1.0, 0.0, 1.4, 1.4, 1.4, 1.4];

    /// Calculates with k-weighting filters for one channel.
    ///
    /// EBU R128 parameter sampling rate adaption after
    /// Mansbridge, Stuart, Saoirse Finn, and Joshua D. Reiss.
    /// "Implementation and Evaluation of Autonomous Multi-track Fader Control."
    /// Paper presented at the 132nd Audio Engineering Society Convention,
    /// Budapest, Hungary, 2012."
    fn k_filter(fs: f64) -> [Biquad; 2] {
        // High shelf filter
        let db: f64 = 3.999843853973347;
        let f0: f64 = 1681.974450955533;
        let q: f64 = 0.7071752369554196;
        let k = (std::f64::consts::PI * f0 / fs).tan();
        let vh = 10.0_f64.powf(db / 20.0);
        let vb = vh.powf(0.4996667741545416);
        let a0 = 1.0 + k / q + k * k;
        let hsf = Biquad::new(
            [
                (vh + vb * k / q + k * k) / a0,
                2.0 * (k * k - vh) / a0,
                (vh - vb * k / q + k * k) / a0,
            ],
            [2.0 * (k * k - 1.0) / a0, (1.0 - k / q + k * k) / a0],
        );

        // High pass filter
        let f0: f64 = 38.13547087602444;
        let q: f64 = 0.5003270373238773;
        let k = (std::f64::consts::PI * f0 / fs).tan();
        let hpf = Biquad::new(
            [1.0, -2.0, 1.0],
            [
                2.0 * (k * k - 1.0) / (1.0 + k / q + k * k),
                (1.0 - k / q + k * k) / (1.0 + k / q + k * k),
            ],
        );

        [hsf, hpf]
    }

    pub fn new(fs: f64, channels: usize) -> Self {
        // 400 ms blocks
        let block_size = (0.4 * fs).ceil() as usize;
        // 100 ms overlap
        let block_overlap = (0.1 * fs).ceil() as usize;

        Self {
            channels,
            filter: vec![Self::k_filter(fs); channels],
            buffer: VecDeque::with_capacity(block_size),
            counter: 0,
            block_size,
            block_overlap,
            histogram: vec![0; Self::BIN_COUNT],
        }
    }

    /// Analyze frame of samples and add it to the cumulative loudness
    /// statistics.
    pub fn process(&mut self, frame: &Vec<f32>) -> Result<(), Error> {
        if frame.len() != self.channels {
            return Err(Error::InvalidFrame);
        }

        let mut sq_sum: f64 = 0.0;
        for (i, x) in frame.iter().enumerate() {
            // Skip unnecessary calculations for LFE channel.
            if Self::CHANNEL_WEIGHT[i] == 0.0 {
                continue;
            }
            // Apply k-weighting filter.
            // True-peak analysis is unnecessary as it does not change the RMS.
            let val = self.filter[i][0].process(*x as f64);
            let val = self.filter[i][1].process(val);
            sq_sum += Self::CHANNEL_WEIGHT[i] * val * val;
        }

        self.buffer.push_back(sq_sum);
        self.counter += 1;

        // Commit every time a new overlapping section starts
        // and if we have a complete block.
        if self.counter % self.block_overlap == 0
            && self.buffer.len() >= self.block_size
        {
            self.commit_block()?;
        }
        Ok(())
    }

    /// Handle incomplete block if no non-zero block was found.
    /// This is only necessary for short signals.
    pub fn finalize(&mut self) -> Result<(), Error> {
        if self.histogram.iter().sum::<usize>() == 0 {
            self.commit_block()?;
        }
        Ok(())
    }

    /// Commits a new block to the histogram.
    /// Incomplete blocks shall be discarded according to the EBU R128
    /// specification so this usually should not be called if the buffer
    /// contains incomplete blocks. An exception to this is if the processed
    /// audio is shorter than one block.
    fn commit_block(&mut self) -> Result<(), Error> {
        // Reset counter to avoid overflow, just in case as the actual value
        // does not matter, only the modulo to block size.
        self.counter = self.block_size;
        // Buffer contains mean square values without root
        // (called z_i in EBU R128).
        let block_sum: f64 = self.buffer.iter().sum();

        // Histogram values are simplified log10() immediate values
        // without -0.691 + 10*(...) to safe computing power. This is
        // possible because these constant cancel out anyway during the
        // following processing steps.
        let block_log = (block_sum / self.buffer.len() as f64).log10();

        // log(block_sum) is within ]-inf, Z_MAX]
        // Get histogram index
        let idx = (Self::BIN_COUNT as f64 / (Self::Z_MAX - Self::GAMMA_A)
            * (block_log - Self::GAMMA_A)
            - 1.0)
            .round() as isize;

        // If index is out of range, the input is denormalized.
        if idx >= Self::BIN_COUNT as isize {
            return Err(Error::Denormalized);
        // If index is less than zero, the value is below threshold.
        } else if idx >= 0 {
            self.histogram[idx as usize] += 1;
        }
        self.buffer.drain(..self.block_size);
        Ok(())
    }

    /// Calculate loudness statistics from histogram.
    /// Returns the accumulated loudness and block count.
    fn accumulated_loudness(&self, start_idx: usize) -> (f64, usize) {
        let mut acc_loudness: f64 = 0.0;
        let mut block_count: usize = 0;

        for (i, x) in self.histogram.iter().enumerate().skip(start_idx) {
            let bin_log = (Self::Z_MAX - Self::GAMMA_A)
                / (Self::BIN_COUNT as f64)
                * ((i + 1) as f64)
                + Self::GAMMA_A;
            acc_loudness += 10.0_f64.powf(bin_log) * (*x as f64);
            block_count += x;
        }

        (acc_loudness, block_count)
    }

    /// Returns the integrative loudness of the processed frames in
    /// linear units, i.e. 10^(LUFS/10).
    pub fn integrative_loudness(&self) -> f64 {
        let (acc_loudness, block_count) = self.accumulated_loudness(0);

        // Calculate gamma_r from histogram.
        // Histogram values are simplified log(x^2) immediate values
        // without -0.691 + 10*(...) to safe computing power. This is
        // possible because they will cancel out anyway.
        // The -1 in the line below is the -10 LUFS from the EBU R128
        // specification without the scaling factor of 10.
        let gamma_r = (acc_loudness / block_count as f64).log10() - 1.0;
        let idx_r = ((gamma_r - Self::GAMMA_A) * Self::BIN_COUNT as f64
            / (Self::Z_MAX - Self::GAMMA_A)
            - 1.0)
            .round() as usize;

        // Apply Gamma_R threshold and calculate gated loudness (extent).
        let (acc_loudness, block_count) = self.accumulated_loudness(idx_r);
        if block_count == 0 {
            // Silence was processed
            0.0
        } else {
            // LUFS is defined as -0.691 dB + 10*log10(sum(channels))
            0.8529037031 * acc_loudness / block_count as f64
        }
    }
}
