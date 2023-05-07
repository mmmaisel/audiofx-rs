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
mod conversion;
mod error;
mod filters;
mod frame;
mod operations;
mod progress;

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
    /// Analyze audio true peak
    TruePeak(analyzer::true_peak::Settings),
    /// Analyze audio loudness
    Loudness(analyzer::loudness::Settings),
    /// Analyze audio RMS
    Rms(analyzer::rms::Settings),
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

    match cli.command {
        Commands::Normalize(x) => {
            let output = match &cli.output_filename {
                Some(filename) => WavWriter::create(filename, spec).unwrap(),
                None => {
                    println!("No output filename was given!");
                    return;
                }
            };
            if let Err(e) = x.normalize(input, output) {
                println!("\nNormalizing failed: {}", e.to_string());
            }
        }
        Commands::TruePeak(x) => match x.analyze(&mut input) {
            Ok(true_peak) => println!("Input has true peak at {:?}", true_peak),
            Err(e) => println!("True peak analyses failed: {}", e.to_string()),
        },
        Commands::Loudness(x) => match x.analyze(&mut input) {
            Ok(loudness) => println!(
                "Input has integrative loudness of {:?} LUFS",
                loudness
                    .iter()
                    .map(|x| 10.0 * x.log10())
                    .collect::<Vec<f64>>(),
            ),
            Err(e) => println!("Loudness analysis failed: {}", e.to_string()),
        },
        Commands::Rms(x) => match x.analyze(&mut input) {
            Ok(rms) => println!(
                "Input has RMS of {:?} dB",
                rms.iter().map(|x| 20.0 * x.log10()).collect::<Vec<f64>>()
            ),
            Err(e) => println!("RMS analysis failed: {}", e.to_string()),
        },
    };
}
