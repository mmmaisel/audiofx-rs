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
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;

use gtk4::glib::{self, Object};
use gtk4::{
    Accessible, Actionable, Box, Buildable, Button, ConstraintTarget, Label,
    Orientable, Orientation, Widget,
};

use super::Waveform;

#[derive(Default)]
pub struct AudioTrackImpl {}

#[glib::object_subclass]
impl ObjectSubclass for AudioTrackImpl {
    const NAME: &'static str = "AudioTrack";
    type Type = AudioTrack;
    type ParentType = Box;
}

impl ObjectImpl for AudioTrackImpl {
    fn constructed(&self) {
        self.parent_constructed();

        let bx = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(4)
            .build();

        let label = Label::builder().label("Track 1").build();
        let button = Button::builder().label("Mute").build();
        bx.append(&label);
        bx.append(&button);
        let wave = Waveform::default();

        let obj = self.obj();
        obj.set_spacing(4);
        obj.set_margin_top(4);
        obj.set_margin_bottom(4);
        obj.set_margin_start(4);
        obj.set_margin_end(4);
        obj.append(&bx);
        obj.append(&wave);
    }
}

impl BoxImpl for AudioTrackImpl {}
impl WidgetImpl for AudioTrackImpl {}

glib::wrapper! {
    pub struct AudioTrack(ObjectSubclass<AudioTrackImpl>)
        @extends Box, Widget,
        @implements Accessible, Actionable, Buildable, ConstraintTarget,
            Orientable;
}

impl Default for AudioTrack {
    fn default() -> Self {
        Object::builder().build()
    }
}
