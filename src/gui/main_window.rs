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

use super::{
    widgets::ImageButton, WavehackerApplication, WavehackerSidebar,
    WavehackerWorkspace,
};
use gtk4::glib::{self, Object, Sender};
use gtk4::{
    gio, ApplicationWindow, FileDialog, HeaderBar, Paned, PolicyType,
    ScrolledWindow, Widget, Window,
};

use std::cell::RefCell;

#[derive(Default)]
pub struct WavehackerWindowImpl {
    open_button: RefCell<ImageButton>,
    save_button: RefCell<ImageButton>,
    sidebar: RefCell<WavehackerSidebar>,
    workspace: RefCell<WavehackerWorkspace>,
}

#[glib::object_subclass]
impl ObjectSubclass for WavehackerWindowImpl {
    const NAME: &'static str = "WavehackerWindow";
    type Type = WavehackerWindow;
    type ParentType = ApplicationWindow;
}

impl ObjectImpl for WavehackerWindowImpl {
    fn constructed(&self) {
        self.parent_constructed();

        // TODO: new, export, wave-view, spec-view
        // TODO: list view with "add-action" button
        let open_button = ImageButton::new("Open", "document-open");
        let save_button = ImageButton::new("Save", "document-save");
        let header_bar = HeaderBar::builder().build();
        header_bar.pack_start(&open_button);
        header_bar.pack_end(&save_button);

        self.open_button.replace(open_button);
        self.save_button.replace(save_button);
        self.obj().set_titlebar(Some(&header_bar));

        let workspace = self.workspace.borrow().clone();
        let sidebar = self.sidebar.borrow().clone();

        let workspace_scroll = ScrolledWindow::builder()
            .child(&workspace)
            .hscrollbar_policy(PolicyType::Never)
            .vscrollbar_policy(PolicyType::Always)
            .build();

        let paned = Paned::builder()
            .start_child(&workspace_scroll)
            .end_child(&sidebar)
            .build();

        self.obj().set_child(Some(&paned));
    }
}
impl WidgetImpl for WavehackerWindowImpl {}
impl WindowImpl for WavehackerWindowImpl {}
impl ApplicationWindowImpl for WavehackerWindowImpl {}

glib::wrapper! {
    pub struct WavehackerWindow(ObjectSubclass<WavehackerWindowImpl>)
        @extends Widget, Window, ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl WavehackerWindow {
    pub fn new(app: &WavehackerApplication) -> Self {
        Object::builder().property("application", app).build()
    }

    pub fn setup_actions(&self, tx: Sender<super::GuiEvent>) {
        let window = WavehackerWindowImpl::from_obj(self);

        window.open_button.borrow_mut().clone().connect_clicked(
            glib::clone!(@strong tx => move |button| {
                Self::open_file(button, tx.clone());
            }),
        );

        window.save_button.borrow_mut().clone().connect_clicked(
            glib::clone!(@strong tx => move |button| {
                Self::save_file(button, tx.clone());
            }),
        );
        window.sidebar.borrow().setup_actions();
    }

    fn open_file(button: &ImageButton, tx: Sender<super::GuiEvent>) {
        let dialog =
            FileDialog::builder().modal(true).title("Open File").build();

        dialog.open(
            Some(&button.root().unwrap().downcast::<Window>().unwrap()),
            None::<&gio::Cancellable>,
            glib::clone!(@strong tx => move |result| {
                if let Ok(x) = result {
                    tx
                        .send(super::GuiEvent::OpenFile(x))
                        .unwrap();
                }
            }),
        );
    }

    fn save_file(button: &ImageButton, tx: Sender<super::GuiEvent>) {
        let dialog =
            FileDialog::builder().modal(true).title("Save File").build();

        dialog.save(
            Some(&button.root().unwrap().downcast::<Window>().unwrap()),
            None::<&gio::Cancellable>,
            glib::clone!(@strong tx => move |result| {
                if let Ok(x) = result {
                    tx
                        .send(super::GuiEvent::SaveFile(x))
                        .unwrap();
                }
            }),
        );
    }
}
