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
#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};
use hound::{WavReader, WavWriter};

mod analyzer;
mod conversion;
mod effects;
mod error;
mod filters;
mod frame;
mod gui;
mod operations;
mod progress;

#[derive(Debug, Parser)]
#[command(name = "audio-effects")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Input wav filename
    #[arg(short)]
    input_filename: Option<String>,
    /// Output wav filename
    #[arg(short)]
    output_filename: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Amplifier
    Amplify(effects::amplify::Settings),
    /// Dynamic compression
    Compressor(effects::compressor::Settings),
    /// Normalize audio loudness
    Normalize(operations::normalize::Settings),
    /// Analyze audio true peak
    TruePeak(analyzer::true_peak::Settings),
    /// Analyze audio loudness
    Loudness(analyzer::loudness::Settings),
    /// Analyze audio RMS
    Rms(analyzer::rms::Settings),
}

fn open_input(
    input_filename: Option<String>,
) -> WavReader<std::io::BufReader<std::fs::File>> {
    let input = WavReader::open(input_filename.unwrap()).unwrap();
    let spec = input.spec();
    let duration = input.duration();
    eprintln!(
        "channels: {}, sample_rate: {}, length: {}",
        spec.channels, spec.sample_rate, duration,
    );

    input
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        None => gui::run(),
        Some(x) => match x {
            Commands::Amplify(x) => {
                let mut input = open_input(cli.input_filename);
                let mut output = match &cli.output_filename {
                    Some(filename) => {
                        WavWriter::create(filename, input.spec()).unwrap()
                    }
                    None => {
                        println!("No output filename was given!");
                        return;
                    }
                };
                if let Err(e) = x.amplify(&mut input, &mut output) {
                    println!("\nAmplifying failed: {}", e.to_string());
                }
                if let Err(e) = output.finalize() {
                    println!("Finalizing wav file failed: {}", e.to_string());
                }
            }
            Commands::Compressor(x) => {
                let mut input = open_input(cli.input_filename);
                let mut output = match &cli.output_filename {
                    Some(filename) => {
                        WavWriter::create(filename, input.spec()).unwrap()
                    }
                    None => {
                        println!("No output filename was given!");
                        return;
                    }
                };
                if let Err(e) = x.compress(&mut input, &mut output) {
                    println!("\nCompressing failed: {}", e.to_string());
                }
                if let Err(e) = output.finalize() {
                    println!("Finalizing wav file failed: {}", e.to_string());
                }
            }
            Commands::Normalize(x) => {
                let mut input = open_input(cli.input_filename);
                let mut output = match &cli.output_filename {
                    Some(filename) => {
                        WavWriter::create(filename, input.spec()).unwrap()
                    }
                    None => {
                        println!("No output filename was given!");
                        return;
                    }
                };
                if let Err(e) = x.normalize(&mut input, &mut output) {
                    println!("\nNormalizing failed: {}", e.to_string());
                }
                if let Err(e) = output.finalize() {
                    println!("Finalizing wav file failed: {}", e.to_string());
                }
            }
            Commands::TruePeak(x) => {
                match x.analyze(&mut open_input(cli.input_filename)) {
                    Ok(true_peak) => {
                        println!("Input has true peak at {:?}", true_peak)
                    }
                    Err(e) => {
                        println!("True peak analyses failed: {}", e.to_string())
                    }
                }
            }
            Commands::Loudness(x) => {
                match x.analyze(&mut open_input(cli.input_filename)) {
                    Ok(loudness) => println!(
                        "Input has integrative loudness of {:?} LUFS",
                        loudness
                            .iter()
                            .map(|x| 10.0 * x.log10())
                            .collect::<Vec<f64>>(),
                    ),
                    Err(e) => {
                        println!("Loudness analysis failed: {}", e.to_string())
                    }
                }
            }
            Commands::Rms(x) => {
                match x.analyze(&mut open_input(cli.input_filename)) {
                    Ok(rms) => println!(
                        "Input has RMS of {:?} dB",
                        rms.iter()
                            .map(|x| 20.0 * x.log10())
                            .collect::<Vec<f64>>()
                    ),
                    Err(e) => {
                        println!("RMS analysis failed: {}", e.to_string())
                    }
                }
            }
        },
    };
}
