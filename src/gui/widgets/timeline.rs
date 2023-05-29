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

use gtk4::glib::{self, Object, ParamSpec, Properties, Value};
use gtk4::{
    Accessible, Actionable, Adjustment, Buildable, ConstraintTarget,
    DrawingArea, Scrollable, ScrollablePolicy, Widget,
};

use std::cell::RefCell;

#[derive(Properties)]
#[properties(wrapper_type = Timeline)]
pub struct TimelineImpl {
    #[property(get, set, nullable, override_interface = Scrollable)]
    hadjustment: RefCell<Option<Adjustment>>,
    #[property(get, set, override_interface = Scrollable)]
    hscroll_policy: RefCell<ScrollablePolicy>,
    #[property(get, set, nullable, override_interface = Scrollable)]
    vadjustment: RefCell<Option<Adjustment>>,
    #[property(get, set, override_interface = Scrollable)]
    vscroll_policy: RefCell<ScrollablePolicy>,
}

impl Default for TimelineImpl {
    fn default() -> Self {
        Self {
            hadjustment: None.into(),
            hscroll_policy: ScrollablePolicy::Natural.into(),
            vadjustment: None.into(),
            vscroll_policy: ScrollablePolicy::Natural.into(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for TimelineImpl {
    const NAME: &'static str = "Timeline";
    type Type = Timeline;
    type ParentType = DrawingArea;
    type Interfaces = (Scrollable,);
}

impl ObjectImpl for TimelineImpl {
    fn properties() -> &'static [ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
        self.derived_set_property(id, value, pspec)
    }

    fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
        self.derived_property(id, pspec)
    }

    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();

        obj.set_content_height(16);

        // TODO: size this plot
        // TODO: plot actual data
        // TODO: sync track header width

        obj.set_draw_func(|obj, context, width, height| {
            let timeline = obj.clone().downcast::<Timeline>().unwrap();
            let adj = timeline.hadjustment().unwrap();
            let widthf = width as f64;
            let heightf = height as f64;

            context.set_source_rgb(0.0, 0.0, 0.0);
            context.set_line_width(2.0);

            let mut x = 0.0;
            let val = adj.value();
            while x < widthf + 100.0 {
                context.move_to(x - val % 100.0, heightf * 0.5);
                context
                    .show_text(&format!("{}", ((x + val) / 100.0) as u32))
                    .unwrap();

                x += 100.0;
            }
            //context.line_to(widthf - 8.0, heightf * 0.5);
            //context.show_text("1").unwrap();
            //context.stroke().unwrap();
        });

        obj.connect_resize(|obj, x, _y| {
            let adj = obj.hadjustment().unwrap();

            // TODO: proper values
            adj.configure(
                adj.value(),
                0.0,
                x as f64 * 2.0,
                10.0,
                10.0,
                x as f64,
            );

            obj.set_hadjustment(Some(adj));
        });

        obj.connect_hadjustment_notify(|obj| {
            if let Some(adj) = obj.hadjustment() {
                adj.connect_value_notify(
                    glib::clone!(@strong obj => move |_adj| {
                        obj.queue_draw();
                    }),
                );
            }
        });
    }
}

impl DrawingAreaImpl for TimelineImpl {}
impl ScrollableImpl for TimelineImpl {}
impl WidgetImpl for TimelineImpl {}

glib::wrapper! {
    pub struct Timeline(ObjectSubclass<TimelineImpl>)
        @extends DrawingArea, Widget,
        @implements Accessible, Actionable, Buildable, ConstraintTarget,
            Scrollable;
}

impl Default for Timeline {
    fn default() -> Self {
        Object::builder()
            .property("hexpand", true)
            .property("vexpand", true)
            .build()
    }
}
