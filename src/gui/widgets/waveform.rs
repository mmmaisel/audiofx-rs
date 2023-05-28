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
    Accessible, Actionable, Buildable, ConstraintTarget, DrawingArea, Widget,
};

// TODO: implement scrollable

#[derive(Default)]
pub struct WaveformImpl {}

#[glib::object_subclass]
impl ObjectSubclass for WaveformImpl {
    const NAME: &'static str = "Waveform";
    type Type = Waveform;
    type ParentType = DrawingArea;
}

impl ObjectImpl for WaveformImpl {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.set_content_height(128);

        // TODO: size this plot
        // TODO: add hscroll
        // TODO: plot actual data

        obj.set_draw_func(|_area, context, width, height| {
            let widthf = width as f64;
            let heightf = height as f64;

            context.set_source_rgb(0.0, 0.0, 0.0);
            context.set_line_width(2.0);

            context.move_to(0.0, heightf * 0.5);
            for x in 1..width {
                let y = heightf * (0.5 + 0.5 * (x as f64 / 10.0).sin());
                context.line_to(x.into(), y);
            }
            context.stroke().unwrap();
        });
    }
}

impl DrawingAreaImpl for WaveformImpl {}
impl WidgetImpl for WaveformImpl {}

glib::wrapper! {
    pub struct Waveform(ObjectSubclass<WaveformImpl>)
        @extends DrawingArea, Widget,
        @implements Accessible, Actionable, Buildable, ConstraintTarget;
}

impl Default for Waveform {
    fn default() -> Self {
        Object::builder()
            .property("hexpand", true)
            .property("vexpand", true)
            .build()
    }
}
