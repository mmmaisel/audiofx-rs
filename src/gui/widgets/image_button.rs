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
    Accessible, Actionable, Box, Buildable, Button, ConstraintTarget, Image,
    Label, Orientation, Widget,
};

use std::cell::RefCell;

#[derive(Properties, Default)]
#[properties(wrapper_type = ImageButton)]
pub struct ImageButtonImpl {
    #[property(get, set)]
    icon: RefCell<String>,
    #[property(get, set)]
    label: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for ImageButtonImpl {
    const NAME: &'static str = "ImageButton";
    type Type = ImageButton;
    type ParentType = Button;
}

impl ObjectImpl for ImageButtonImpl {
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
        let icon = Image::new();
        let label = Label::new(None);

        let bx = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(4)
            .build();
        bx.append(&icon);
        bx.append(&label);

        let obj = self.obj();
        obj.set_child(Some(&bx));
        obj.bind_property("label", &label, "label")
            .sync_create()
            .build();
        obj.bind_property("icon", &icon, "icon-name")
            .sync_create()
            .build();
    }
}
impl WidgetImpl for ImageButtonImpl {}
impl ButtonImpl for ImageButtonImpl {}

glib::wrapper! {
    pub struct ImageButton(ObjectSubclass<ImageButtonImpl>)
        @extends Button, Widget,
        @implements Accessible, Actionable, Buildable, ConstraintTarget;
}

impl ImageButton {
    pub fn new(label: &str, icon: &str) -> Self {
        Object::builder()
            .property("label", label)
            .property("icon", icon)
            .build()
    }
}

impl Default for ImageButton {
    fn default() -> Self {
        Object::builder().build()
    }
}
