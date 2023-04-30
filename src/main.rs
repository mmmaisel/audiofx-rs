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
#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};
use hound::{WavReader, WavWriter};

mod analyzer;
mod biquad;
mod frame;
mod operations;

use frame::FrameIterator;

#[derive(Debug, Parser)]
#[command(name = "audio-effects")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Input wav filename
    #[arg(short)]
    input_filename: String,
    /// Output wav filename
    #[arg(short)]
    output_filename: String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Normalize audio loudness
    Normalize(operations::normalize::Settings),
}

fn main() {
    let cli = Cli::parse();

    let mut input = WavReader::open(&cli.input_filename).unwrap();
    let spec = input.spec();
    let duration = input.duration();
    eprintln!(
        "channels: {}, sample_rate: {}, length: {}",
        spec.channels, spec.sample_rate, duration,
    );

    let mut output = WavWriter::create(cli.output_filename, spec).unwrap();
    let mut progress = 0;

    let mut frames = FrameIterator::new(input.samples::<f32>(), spec.channels);
    while let Some(x) = frames.next() {
        progress += 1;
        if progress % 100000 == 0 {
            eprint!("\rProcessing sample: {}/{}", progress, duration);
        }
        for i in x.unwrap() {
            output.write_sample(*i).unwrap();
        }
    }
    output.finalize().unwrap();
    eprintln!("\rDone!");
}
