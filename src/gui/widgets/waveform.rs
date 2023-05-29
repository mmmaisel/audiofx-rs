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
#[properties(wrapper_type = Waveform)]
pub struct WaveformImpl {
    #[property(get, set, nullable, override_interface = Scrollable)]
    hadjustment: RefCell<Option<Adjustment>>,
    #[property(get, set, override_interface = Scrollable)]
    hscroll_policy: RefCell<ScrollablePolicy>,
    #[property(get, set, nullable, override_interface = Scrollable)]
    vadjustment: RefCell<Option<Adjustment>>,
    #[property(get, set, override_interface = Scrollable)]
    vscroll_policy: RefCell<ScrollablePolicy>,
}

impl Default for WaveformImpl {
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
impl ObjectSubclass for WaveformImpl {
    const NAME: &'static str = "Waveform";
    type Type = Waveform;
    type ParentType = DrawingArea;
    type Interfaces = (Scrollable,);
}

impl ObjectImpl for WaveformImpl {
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
        obj.set_content_height(128);

        // TODO: size this plot
        // TODO: plot actual data

        obj.set_draw_func(|obj, context, width, height| {
            let heightf = height as f64;

            let wave = obj.clone().downcast::<Waveform>().unwrap();
            let adj = wave.hadjustment().unwrap();
            let val = adj.value();

            context.set_source_rgb(0.0, 0.0, 0.0);
            context.set_line_width(2.0);

            context.move_to(0.0, heightf * 0.5);
            for x in 1..width {
                let y = heightf * (0.5 + 0.5 * ((x as f64 + val) / 10.0).sin());
                context.line_to(x.into(), y);
            }
            context.stroke().unwrap();
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

impl DrawingAreaImpl for WaveformImpl {}
impl ScrollableImpl for WaveformImpl {}
impl WidgetImpl for WaveformImpl {}

glib::wrapper! {
    pub struct Waveform(ObjectSubclass<WaveformImpl>)
        @extends DrawingArea, Widget,
        @implements Accessible, Actionable, Buildable, ConstraintTarget,
            Scrollable;
}

impl Default for Waveform {
    fn default() -> Self {
        Object::builder()
            .property("hexpand", true)
            .property("vexpand", true)
            .build()
    }
}
