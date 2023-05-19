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
/// Terminal based progress indicator.
pub struct Progress {
    count: usize,
    total_count: usize,
    update_every: usize,
    message: String,
}

impl Progress {
    pub fn new<S>(total_count: usize, message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            count: 0,
            total_count,
            update_every: total_count / 100,
            message: message.into(),
        }
    }

    pub fn next(&mut self) {
        self.count += 1;
        if self.count % self.update_every == 0 {
            eprint!("\r{}: {}/{}", self.message, self.count, self.total_count);
        }
    }
}

impl Drop for Progress {
    fn drop(&mut self) {
        eprintln!();
    }
}
