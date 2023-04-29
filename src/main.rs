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

use clap::Parser;
use hound::{WavReader, WavWriter};

#[derive(Debug, Parser)]
#[command(name = "audio-effects")]
struct Cli {
    /// Input wav filename
    #[arg(short)]
    input_filename: String,
    /// Output wav filename
    #[arg(short)]
    output_filename: String,
}

fn main() {
    let cli = Cli::parse();

    let mut input = WavReader::open(&cli.input_filename).unwrap();
    let spec = input.spec();
    let duration = input.duration();
    eprintln!(
        "channels: {}, sample_rate: {}, length: {}",
        spec.channels,
        spec.sample_rate,
        (duration as usize) / (spec.channels as usize),
    );

    let mut output = WavWriter::create(cli.output_filename, spec).unwrap();
    let mut channel = 0;
    let mut values = vec![0.0; spec.channels.into()];

    for i in input.samples() {
        values[channel] = i.unwrap();
        channel += 1;
        if channel == spec.channels.into() {
            channel = 0;
            // XXX: process data here
            for i in &values {
                output.write_sample(*i).unwrap();
            }
        }
    }

    output.finalize().unwrap();
}
