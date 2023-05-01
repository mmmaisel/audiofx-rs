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
pub enum Error {
    Denormalized,
    InvalidFrame,
    Io(std::io::Error),
    Hound(hound::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Denormalized => write!(
                f,
                "Block value exceeds full-scale. Consider normalizing input."
            ),
            Self::InvalidFrame => {
                write!(f, "Frame is invalid amount of samples.")
            }
            Self::Io(e) => {
                write!(f, "IO Error: {}", e.to_string())
            }
            Self::Hound(e) => {
                write!(f, "{}", e.to_string())
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<hound::Error> for Error {
    fn from(e: hound::Error) -> Self {
        Self::Hound(e)
    }
}
