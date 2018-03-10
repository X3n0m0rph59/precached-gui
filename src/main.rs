/*
    precached-GUI - A GUI for precached
    Copyright (C) 2018 the precached developers

    This file is part of precached-GUI.

    Precached-GUI is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Precached-GUI is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with precached-GUI.  If not, see <http://www.gnu.org/licenses/>.
*/

extern crate gtk;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate gio;

extern crate libc;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate zmq;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::env::args;
use gio::{ApplicationExt, ApplicationExtManual};

mod gui;
mod ipc;
mod globals;

use gui::MainWindow;

fn build_ui(app: &gtk::Application) {
    let mut globals = globals::Globals::new();
    let mut main_window = MainWindow::new(&app, globals);

    main_window.show_all();    
}

fn main() {
    pretty_env_logger::init(); //.expect("Could not initialize the logging subsystem!");

    gtk::init().expect("Failed to initialize GTK!");        

    let application = gtk::Application::new("org.x3n0m0rph59.precached-gui", 
                                            gio::ApplicationFlags::empty())
                                        .expect("Initialization failed!");

    application.connect_startup(move |app| {
        build_ui(app);
    });

    application.connect_activate(|_| {});
    
    application.run(&args().collect::<Vec<_>>());

    gtk::main();
}
