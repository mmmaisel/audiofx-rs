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
#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Mode {
    /// Analyze peak amplitude
    Amplitude,
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
    target: f64,
    /// Analyze multiple channels independently
    #[arg(short)]
    channel_independent: bool,
    /// Normalize result to stereo (recommended),
    /// This is the recommended, but not EBU R128 compliant, setting.
    #[arg(short, action=clap::ArgAction::SetFalse)]
    normalize: bool,
}
