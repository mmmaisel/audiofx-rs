/******************************************************************************\
    audiofx-rs
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
mod main_window;
mod widgets;

use gtk4::prelude::*;
use gtk4::subclass::prelude::*;

use gtk4::glib::{self, MainContext, Object, WeakRef, PRIORITY_DEFAULT};
use gtk4::{gio, Application};

use main_window::AudiofxWindow;
use std::{cell::RefCell, rc::Rc};

pub enum GuiEvent {
    OpenFile(gio::File),
    SaveFile(gio::File),
}

#[derive(Default)]
pub struct ApplicationContext {
    audio: Vec<f32>,
}

#[derive(Default)]
pub struct AudiofxApplicationImpl {
    window: RefCell<Option<WeakRef<AudiofxWindow>>>,
    context: Rc<RefCell<ApplicationContext>>,
}

#[glib::object_subclass]
impl ObjectSubclass for AudiofxApplicationImpl {
    const NAME: &'static str = "AudiofxApplication";
    type Type = AudiofxApplication;
    type ParentType = Application;
}

impl ObjectImpl for AudiofxApplicationImpl {}

impl ApplicationImpl for AudiofxApplicationImpl {
    fn activate(&self) {
        self.parent_activate();

        let (tx, rx) = MainContext::channel(PRIORITY_DEFAULT);
        let window = AudiofxWindow::new(&self.obj());

        window.setup_events(tx);
        self.window.replace(Some(window.downgrade()));
        window.present();

        let context = self.context.clone();
        rx.attach(None, move |event| {
            match event {
                GuiEvent::OpenFile(file) => {
                    println!("Opened {:?}", file.path().unwrap());
                    // TODO: load file here
                }
                GuiEvent::SaveFile(file) => {
                    println!("Saved {:?}", file.path().unwrap());
                    // TODO: save result here
                }
            }
            Continue(true)
        });
    }
}

impl GtkApplicationImpl for AudiofxApplicationImpl {}

glib::wrapper! {
    pub struct AudiofxApplication(ObjectSubclass<AudiofxApplicationImpl>)
        @extends gio::Application, Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl AudiofxApplication {
    pub fn new() -> Self {
        Object::builder()
            .property("application-id", "audiofx.rs")
            .build()
    }
}

pub fn run() {
    AudiofxApplication::new().run_with_args(&Vec::<String>::new());
}
