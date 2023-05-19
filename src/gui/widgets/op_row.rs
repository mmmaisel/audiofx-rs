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

use gtk4::glib::{self, BindingFlags, Object, ParamSpec, Properties, Value};
use gtk4::{
    Accessible, Actionable, Box, Buildable, Button, ConstraintTarget, Label,
    ListBox, Orientation, Revealer, Widget,
};

use std::cell::RefCell;

#[derive(Properties, Default)]
#[properties(wrapper_type = OpRow)]
pub struct OpRowImpl {
    #[property(get, set, nullable)]
    child: RefCell<Option<Widget>>,
    #[property(get, set)]
    label: RefCell<String>,
    #[property(get, set)]
    reveal_child: RefCell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for OpRowImpl {
    const NAME: &'static str = "OpRow";
    type Type = OpRow;
    type ParentType = Box;
}

impl ObjectImpl for OpRowImpl {
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

        let del_button = Button::builder().icon_name("list-remove").build();
        let expander = Button::builder()
            .icon_name("go-next")
            .has_frame(false)
            .build();
        let label = Label::builder()
            .halign(gtk4::Align::Start)
            .hexpand(true)
            .build();

        // TODO: implement reordering

        let bx = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(4)
            .build();
        bx.append(&expander);
        bx.append(&label);
        bx.append(&del_button);

        obj.append(&bx);
        obj.bind_property("label", &label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();

        del_button.connect_clicked(glib::clone!(@weak obj => move |_button| {
            // XXX: this code sucks
            // TODO: update model
            let list_row = obj.parent().unwrap();
            let list_box = list_row.parent().unwrap().downcast::<ListBox>().unwrap();
            list_box.remove(&list_row.upcast::<Widget>());
        }));

        let revealer = Revealer::builder().build();
        obj.bind_property("child", &revealer, "child")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        obj.bind_property("reveal-child", &revealer, "reveal-child")
            .flags(BindingFlags::SYNC_CREATE)
            .build();

        expander.connect_clicked(glib::clone!(@weak obj => move |button| {
            if obj.reveal_child() {
                obj.set_reveal_child(false);
                button.set_icon_name("go-next");
            } else {
                obj.set_reveal_child(true);
                button.set_icon_name("go-down");
            }
        }));

        obj.append(&revealer);
    }
}
impl WidgetImpl for OpRowImpl {}
impl BoxImpl for OpRowImpl {}

glib::wrapper! {
    pub struct OpRow(ObjectSubclass<OpRowImpl>)
        @extends Box, Widget,
        @implements Accessible, Actionable, Buildable, ConstraintTarget;
}

impl OpRow {
    pub fn new(label: &str) -> Self {
        Object::builder()
            .property("label", label)
            .property("orientation", Orientation::Vertical)
            .build()
    }
}

impl Default for OpRow {
    fn default() -> Self {
        Object::builder()
            .property("orientation", Orientation::Vertical)
            .build()
    }
}
