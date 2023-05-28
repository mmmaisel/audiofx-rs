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

#[derive(Default)]
pub struct TimelineImpl {}

#[glib::object_subclass]
impl ObjectSubclass for TimelineImpl {
    const NAME: &'static str = "Timeline";
    type Type = Timeline;
    type ParentType = DrawingArea;
}

impl ObjectImpl for TimelineImpl {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();

        obj.set_content_height(16);

        // TODO: size this plot
        // TODO: add hscroll
        // TODO: plot actual data

        obj.set_draw_func(|_area, context, width, height| {
            let widthf = width as f64;
            let heightf = height as f64;

            context.set_source_rgb(0.0, 0.0, 0.0);
            context.set_line_width(2.0);
            context.move_to(0.0, heightf * 0.5);
            context.line_to(widthf, heightf * 0.5);
            context.stroke().unwrap();
        });
    }
}

impl DrawingAreaImpl for TimelineImpl {}
impl WidgetImpl for TimelineImpl {}

glib::wrapper! {
    pub struct Timeline(ObjectSubclass<TimelineImpl>)
        @extends DrawingArea, Widget,
        @implements Accessible, Actionable, Buildable, ConstraintTarget;
}

impl Default for Timeline {
    fn default() -> Self {
        Object::builder()
            .property("hexpand", true)
            .property("vexpand", true)
            .build()
    }
}
