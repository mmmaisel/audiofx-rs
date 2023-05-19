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

use super::{widgets::ImageButton, WavehackerApplication};
use gtk4::glib::{self, Object, Sender};
use gtk4::{gio, ApplicationWindow, FileDialog, HeaderBar, Widget, Window};

use std::cell::RefCell;

#[derive(Default)]
pub struct WavehackerWindowImpl {
    open_button: RefCell<ImageButton>,
    save_button: RefCell<ImageButton>,
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

    pub fn setup_events(&self, tx: Sender<super::GuiEvent>) {
        let window = WavehackerWindowImpl::from_obj(self);

        let open_tx = tx.clone();
        window.open_button.borrow_mut().clone().connect_clicked(
            move |button| {
                let dialog = FileDialog::builder()
                    .modal(true)
                    .title("Open File")
                    .build();

                let open_tx2 = open_tx.clone();
                dialog.open(
                    Some(&button.root().unwrap().downcast::<Window>().unwrap()),
                    None::<&gio::Cancellable>,
                    move |result| {
                        if let Ok(x) = result {
                            open_tx2
                                .send(super::GuiEvent::OpenFile(x))
                                .unwrap();
                        }
                    },
                );
            },
        );

        let save_tx = tx.clone();
        window.save_button.borrow_mut().clone().connect_clicked(
            move |button| {
                let dialog = FileDialog::builder()
                    .modal(true)
                    .title("Save File")
                    .build();

                let save_tx2 = save_tx.clone();
                dialog.save(
                    Some(&button.root().unwrap().downcast::<Window>().unwrap()),
                    None::<&gio::Cancellable>,
                    move |result| {
                        if let Ok(x) = result {
                            save_tx2
                                .send(super::GuiEvent::SaveFile(x))
                                .unwrap();
                        }
                    },
                );
            },
        );
    }
}
