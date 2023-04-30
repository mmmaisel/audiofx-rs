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
    output_filename: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Normalize audio loudness
    Normalize(operations::normalize::Settings),
    /// Analyze audio loudness
    Loudness(analyzer::loudness::Settings),
}

fn main() {
    let cli = Cli::parse();

    let input = WavReader::open(&cli.input_filename).unwrap();
    let spec = input.spec();
    let duration = input.duration();
    eprintln!(
        "channels: {}, sample_rate: {}, length: {}",
        spec.channels, spec.sample_rate, duration,
    );

    match cli.command {
        Commands::Loudness(x) => match x.analyze(input) {
            Ok(loudness) => println!(
                "\nInput has integrative loudness of {:?} LUFS",
                loudness
                    .iter()
                    .map(|x| 10.0 * x.log10())
                    .collect::<Vec<f64>>(),
            ),
            Err(e) => println!("\nLoudness analysis failed: {}", e.to_string()),
        },
        _ => eprintln!("Not implemented yet!"),
    };
}
