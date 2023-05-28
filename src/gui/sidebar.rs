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

use gtk4::gio::{Menu, SimpleAction};
use gtk4::glib::{self, Object, PropertySet};
use gtk4::{
    Accessible, Actionable, Align, ApplicationWindow, Box, Buildable,
    ConstraintTarget, Image, Label, ListBox, MenuButton, Orientable,
    Orientation, PolicyType, ScrolledWindow, SelectionMode, Widget,
};

use super::widgets::OpRow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct WavehackerSidebarImpl {
    op_list: Rc<RefCell<ListBox>>,
}

#[glib::object_subclass]
impl ObjectSubclass for WavehackerSidebarImpl {
    const NAME: &'static str = "WavehackerSidebar";
    type Type = WavehackerSidebar;
    type ParentType = Box;
}

impl ObjectImpl for WavehackerSidebarImpl {
    fn constructed(&self) {
        self.parent_constructed();
        let icon = Image::builder().icon_name("list-add").build();
        let label = Label::builder().label("Add Operation").build();

        let inner_bx = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(4)
            .build();
        inner_bx.append(&icon);
        inner_bx.append(&label);

        let menu_items = Menu::new();
        menu_items.append(Some("Amplify"), Some("win.add_amplify"));
        menu_items.append(Some("Compressor"), Some("win.add_compressor"));

        let menu_button = MenuButton::builder()
            .child(&inner_bx)
            .menu_model(&menu_items)
            .margin_top(4)
            .margin_bottom(4)
            .margin_start(4)
            .margin_end(4)
            .halign(Align::Center)
            .build();

        let op_list = ListBox::builder()
            .selection_mode(SelectionMode::None)
            .show_separators(true)
            .hexpand(true)
            .vexpand(true)
            .build();
        let op_list_scroll = ScrolledWindow::builder()
            .child(&op_list)
            .hscrollbar_policy(PolicyType::Never)
            .build();

        self.obj().append(&op_list_scroll);
        self.obj().append(&menu_button);

        self.op_list.set(op_list);
    }
}
impl WidgetImpl for WavehackerSidebarImpl {}
impl BoxImpl for WavehackerSidebarImpl {}

glib::wrapper! {
    pub struct WavehackerSidebar(ObjectSubclass<WavehackerSidebarImpl>)
        @extends Box, Widget,
        @implements Accessible, Actionable, Buildable, ConstraintTarget,
            Orientable;
}

impl WavehackerSidebar {
    pub fn setup_actions(&self) {
        let root = self.root().unwrap();
        let window = root.downcast::<ApplicationWindow>().unwrap();
        let op_list = WavehackerSidebarImpl::from_obj(self).op_list.clone();

        let action_amplify = SimpleAction::new("add_amplify", None);
        action_amplify.connect_activate(
            glib::clone!(@strong op_list => move |_, _| {
                // TODO: update model
                let op_row = OpRow::new("Amplify");
                let dummy = Label::builder().label("dummy").build();
                op_row.set_child(Some(dummy));
                op_list.borrow_mut().append(&op_row);
            }),
        );
        window.add_action(&action_amplify);

        let action_compressor = SimpleAction::new("add_compressor", None);
        action_compressor.connect_activate(
            glib::clone!(@strong op_list => move |_, _| {
                // TODO: update model
                let op_row = OpRow::new("Compressor");
                let dummy = Label::builder().label("dummy").build();
                op_row.set_child(Some(dummy));
                op_list.borrow_mut().append(&op_row);
            }),
        );
        window.add_action(&action_compressor);
    }
}

impl Default for WavehackerSidebar {
    fn default() -> Self {
        Object::builder()
            .property("spacing", 4)
            .property("orientation", Orientation::Vertical)
            .build()
    }
}
