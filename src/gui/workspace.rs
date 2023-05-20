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
    Accessible, Actionable, Box, Buildable, ConstraintTarget, Label,
    Orientation, Widget,
};

#[derive(Default)]
pub struct WavehackerWorkspaceImpl {}

#[glib::object_subclass]
impl ObjectSubclass for WavehackerWorkspaceImpl {
    const NAME: &'static str = "WavehackerWorkspace";
    type Type = WavehackerWorkspace;
    type ParentType = Box;
}

impl ObjectImpl for WavehackerWorkspaceImpl {
    fn constructed(&self) {
        self.parent_constructed();

        let dummy = Label::builder().label("Hello Workspace!").build();

        // TODO: add timeline, listbox for tracks, and player widgets
        // TODO: link scrollers for track headers
        // TODO: implement scrollable for workspace,
        //      redirect hscroll to plots and vscroll to listbox

        self.obj().append(&dummy);
    }
}
impl WidgetImpl for WavehackerWorkspaceImpl {}
impl BoxImpl for WavehackerWorkspaceImpl {}

glib::wrapper! {
    pub struct WavehackerWorkspace(ObjectSubclass<WavehackerWorkspaceImpl>)
        @extends Box, Widget,
        @implements Accessible, Actionable, Buildable, ConstraintTarget;
}

impl WavehackerWorkspace {}

impl Default for WavehackerWorkspace {
    fn default() -> Self {
        Object::builder()
            .property("spacing", 4)
            .property("orientation", Orientation::Vertical)
            .build()
    }
}
