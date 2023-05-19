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
mod main_window;
mod widgets;

use gtk4::prelude::*;
use gtk4::subclass::prelude::*;

use gtk4::glib::{self, MainContext, Object, WeakRef, PRIORITY_DEFAULT};
use gtk4::{gio, Application};

use main_window::WavehackerWindow;
use std::{cell::RefCell, rc::Rc};

use hound::WavReader;

pub enum GuiEvent {
    OpenFile(gio::File),
    SaveFile(gio::File),
}

#[derive(Default)]
pub struct ApplicationContext {
    audio: Vec<f32>,
}

#[derive(Default)]
pub struct WavehackerApplicationImpl {
    window: RefCell<Option<WeakRef<WavehackerWindow>>>,
    context: Rc<RefCell<ApplicationContext>>,
}

#[glib::object_subclass]
impl ObjectSubclass for WavehackerApplicationImpl {
    const NAME: &'static str = "WavehackerApplication";
    type Type = WavehackerApplication;
    type ParentType = Application;
}

impl ObjectImpl for WavehackerApplicationImpl {}

impl ApplicationImpl for WavehackerApplicationImpl {
    fn activate(&self) {
        self.parent_activate();

        let (tx, rx) = MainContext::channel(PRIORITY_DEFAULT);
        let window = WavehackerWindow::new(&self.obj());

        window.setup_events(tx);
        self.window.replace(Some(window.downgrade()));
        window.present();

        let context = self.context.clone();

        let obj = self.obj();
        rx.attach(
            None,
            glib::clone!(@strong obj as app => move |event| {
                app.event_loop(event)
            }),
        );
    }
}

impl GtkApplicationImpl for WavehackerApplicationImpl {}

glib::wrapper! {
    pub struct WavehackerApplication(ObjectSubclass<WavehackerApplicationImpl>)
        @extends gio::Application, Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl WavehackerApplication {
    pub fn new() -> Self {
        Object::builder()
            .property("application-id", "wavehacker.rs")
            .build()
    }

    fn event_loop(&self, event: GuiEvent) -> Continue {
        match event {
            GuiEvent::OpenFile(file) => {
                // XXX: TODO: handle file format errors here
                // XXX: move this to some model/controller logic
                // TODO: load audio into grid data structure with LODs
                println!("Opened {:?}", file.path().unwrap());
                let file_stream =
                    file.open_readwrite(None::<&gio::Cancellable>).unwrap();
                let input_stream = file_stream.input_stream().into_read();
                let input = WavReader::new(input_stream).unwrap();
                let spec = input.spec();
                let duration = input.duration();
                eprintln!(
                    "channels: {}, sample_rate: {}, length: {}",
                    spec.channels, spec.sample_rate, duration,
                );
            }
            GuiEvent::SaveFile(file) => {
                println!("Saved {:?}", file.path().unwrap());
                // TODO: save result here
            }
        }
        Continue(true)
    }
}

pub fn run() {
    WavehackerApplication::new().run_with_args(&Vec::<String>::new());
}
