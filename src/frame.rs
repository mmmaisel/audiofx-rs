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
use hound::{Error, Sample};

pub enum ChannelMap {
    Left,
    Right,
    Center,
    Lfe,
    RearLeft,
    RearRight,
    SideLeft,
    SideRight,
}

pub struct FrameIterator<S, T> {
    samples: T,
    channels: u16,
    buffer: Vec<S>,
}

impl<S, T> FrameIterator<S, T>
where
    S: Sample,
    T: Iterator<Item = Result<S, Error>>,
{
    pub fn new(samples: T, channels: u16) -> Self {
        Self {
            samples,
            channels,
            buffer: Vec::with_capacity(channels as usize),
        }
    }

    pub fn next(&mut self) -> Option<Result<&Vec<S>, Error>> {
        self.buffer.clear();

        for _ in 0..self.channels {
            match self.samples.next() {
                None => return None,
                Some(x) => match x {
                    Ok(x) => self.buffer.push(x),
                    Err(e) => return Some(Err(e)),
                },
            }
        }
        Some(Ok(&self.buffer))
    }
}
