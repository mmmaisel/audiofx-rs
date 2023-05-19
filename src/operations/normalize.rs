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
use crate::analyzer::{
    loudness::Settings as Lufs, rms::Settings as Rms,
    true_peak::Settings as TruePeak,
};
use crate::effects::amplify::Settings as Amplify;
use crate::error::Error;
use hound::{WavReader, WavWriter};

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Mode {
    /// Analyze true peak amplitude
    TruePeak,
    /// Analyze LUFS loudness
    Lufs,
    /// Analyze RMS loudness
    Rms,
}

#[derive(Debug, Clone, clap::Args)]
pub struct Settings {
    /// Algorithm to use
    mode: Mode,
    /// Target loudness in dB. Units depends on mode.
    target_db: f64,
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
    pub fn normalize<R, W>(
        &self,
        mut input: &mut WavReader<R>,
        mut output: &mut WavWriter<W>,
    ) -> Result<(), Error>
    where
        R: std::io::Read + std::io::Seek,
        W: std::io::Write + std::io::Seek,
    {
        let gain = match &self.mode {
            Mode::TruePeak => {
                let analyzer = TruePeak::new(self.channel_independent);
                let peak = analyzer.analyze(&mut input)?;
                peak.iter()
                    .map(|x| (10.0_f64.powf(self.target_db / 20.0) / x) as f32)
                    .collect::<Vec<f32>>()
            }
            Mode::Lufs => {
                let analyzer =
                    Lufs::new(self.channel_independent, self.strict_ebur128);
                let loudness = analyzer.analyze(&mut input)?;
                loudness
                    .iter()
                    .map(|x| {
                        (10.0_f64.powf(self.target_db / 10.0) / x).sqrt() as f32
                    })
                    .collect::<Vec<f32>>()
            }
            Mode::Rms => {
                let analyzer = Rms::new(self.channel_independent);
                let rms = analyzer.analyze(&mut input)?;
                rms.iter()
                    .map(|x| (10.0_f64.powf(self.target_db / 20.0) / x) as f32)
                    .collect::<Vec<f32>>()
            }
        };

        input.seek(0)?;
        let amplify =
            Amplify::new(gain.iter().map(|x| 20.0 * x.log10()).collect());
        amplify.amplify(&mut input, &mut output)?;

        Ok(())
    }
}
