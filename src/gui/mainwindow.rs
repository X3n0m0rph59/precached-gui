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

use std::thread;
use gtk::prelude::*;
use gtk::{TreeView, TreeViewColumn, CellRendererText, AboutDialog, WindowPosition};
use gdk_pixbuf::{Pixbuf};
use gio::{SimpleActionExt, ActionMapExt};

use ipc;

use globals;

// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

#[derive(Clone)]
pub struct MainWindow {
    pub globals: globals::Globals,

    pub builder: gtk::Builder,
    pub window: gtk::ApplicationWindow,

    pub tracked_processes_model: gtk::ListStore,
    pub active_traces_model: gtk::ListStore,
    pub prefetcher_threads_model: gtk::ListStore,
    pub events_model: gtk::ListStore,

    pub internal_state_model: gtk::ListStore,
}

macro_rules! list_store {
    ($($x:tt),+) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x::static_type());
        )+

        gtk::ListStore::new(&temp_vec)
    }};
}

impl MainWindow {
    pub fn new(app: &gtk::Application, globals: globals::Globals) -> MainWindow {
        let main_window_layout = include_str!("../../assets/MainWindow.glade");

        let builder = gtk::Builder::new_from_string(main_window_layout);
        let window: gtk::ApplicationWindow = builder.get_object("MainWindow")
                                                    .expect("Could not load UI!");

        window.set_position(WindowPosition::Center);
        // window.set_title("precached-GUI");
        // window.set_wm_class("precached-GUI", "precached-GUI");

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        // Initialize global actions
        Self::init_quit_action(app);
        Self::init_about_action(app);

        let result = MainWindow {
            globals: globals, 
            builder: builder.clone(), 
            window: window,

            // Tree View Models 
            tracked_processes_model: list_store!(String),
            active_traces_model: list_store!(String),
            prefetcher_threads_model: list_store!(String),
            events_model: list_store!(u32, String),
            internal_state_model: list_store!(String, String),
        };

        Self::init_tree_views(&builder, &result);

        let mut wnd = result.clone();
        gtk::timeout_add(500, move || {
            Self::timer(&mut wnd)
        });

        result
    }

    pub fn set_title_message(&mut self, msg: &str) {
        let header_bar: gtk::HeaderBar = self.builder.get_object("HeaderBar")
                                                     .expect("Could not get an UI element!");

        header_bar.set_subtitle(msg);
    }

    pub fn set_status_message(&mut self, msg: &str) {
        let status_bar: gtk::Statusbar = self.builder.get_object("StatusBar")
                                                     .expect("Could not get an UI element!");

        status_bar.push(0, msg);
    }

    pub fn show_all(&mut self) {        
        self.window.show_all();

        self.connect();
    }

    fn timer(main_window: &mut MainWindow) -> gtk::Continue {
        let data = main_window.globals.data.read().expect("Could not lock a global data structure!");
        let data = data.clone();

        // Tracked Processes
        let model = &main_window.tracked_processes_model;
        model.clear();

        let entries = &data.tracked_processes;
        for entry in entries.iter() {
            model.insert_with_values(None, &[0], &[&format!("{}", entry.comm)]);
        }
        
        // Active Traces
        let model = &main_window.active_traces_model;
        model.clear();

        let entries = &data.active_traces;
        for entry in entries.iter() {
            model.insert_with_values(None, &[0], &[&format!("{}", entry.exe.to_string_lossy())]);
        }
        
        // Prefetcher Threads
        let model = &main_window.prefetcher_threads_model;
        model.clear();

        if let Some(s) = data.prefetch_stats {
            let entries = &s.thread_states;
            for entry in entries.iter() {
                model.insert_with_values(None, &[0], &[&format!("{:?}", entry)]);
            }
        }

        // Events
        let model = &main_window.events_model;
        model.clear();

        let entries = &data.events;
        let mut cnt = 0;
        for entry in entries.iter().rev() {
            model.insert_with_values(None, &[0, 1], &[&((entries.len() - cnt) as u32), &entry.msg]);
            cnt += 1;
        }
        
        Continue(true)
    }

    fn connect(&mut self) {
        // Spawn the IPC connection thread                
        let global_data = self.globals.clone().data;

        thread::Builder::new()
                .name(String::from("ipc"))
                .spawn(move || {
            info!("Initializing IPC...");                    
            ipc::ipc_thread_main(global_data);
        }).unwrap();

        self.set_title_message("Connected to precached");
        self.set_status_message("Successfuly connected to the daemon.");
    }

    fn init_quit_action(app: &gtk::Application) {
        let quit = gio::SimpleAction::new("quit", None);
        
        quit.connect_activate(move |_, _| {
            // app.quit();
        });

        app.add_action(&quit);
    }

    fn init_about_action(app: &gtk::Application) {
        let about = gio::SimpleAction::new("about", None);
        
        about.connect_activate(move |_, _| {
            let p = AboutDialog::new();

            p.set_authors(&["X3n0m0rph59"]);
            p.set_website_label(Some("Project Website"));
            p.set_website(Some("http://x3n0m0rph59.github.io/precached"));
            p.set_comments(Some("A GUI for precached"));
            p.set_copyright(Some("The precached Team"));

            // p.set_transient_for(Some(window));
            p.set_program_name("precached GUI");

            let logo = Pixbuf::new_from_file("assets/precached.png");
            if let Ok(logo) = logo {
                p.set_logo(Some(&logo));
            }

            p.run();
            p.destroy();
        });

        app.add_action(&about);
    }

    fn tree_view_cursor_changed(_tree: &gtk::TreeView) {
        // let selection = tree.get_selection();

        // if let Some((model, iter)) = selection.get_selected() {        
            
        // }
    }

    fn init_tree_views(builder: &gtk::Builder, main_window: &MainWindow) {
        // Tab Overview -> Tree View "Tracked Processes"
        let tree_view_overview_tracked_processes: gtk::TreeView = builder.get_object("TreeViewOverviewTrackedProcesses")
                                                                            .expect("Could not get an UI element!");

        // tree_view_overview_tracked_processes.set_headers_visible(true);

        // Self::append_column(&tree_view_overview_tracked_processes);
        Self::append_column(&tree_view_overview_tracked_processes);

        tree_view_overview_tracked_processes.set_model(&main_window.tracked_processes_model);
        tree_view_overview_tracked_processes.connect_cursor_changed(|tree| Self::tree_view_cursor_changed(tree));


        // Tab Overview -> Tree View "Active Traces"
        let tree_view_overview_active_traces: gtk::TreeView = builder.get_object("TreeViewOverviewActiveTraces")
                                                                            .expect("Could not get an UI element!");

        // tree_view_overview_active_traces.set_headers_visible(true);

        // Self::append_column(&tree_view_overview_active_traces);
        Self::append_column(&tree_view_overview_active_traces);

        tree_view_overview_active_traces.set_model(&main_window.active_traces_model);
        tree_view_overview_active_traces.connect_cursor_changed(|tree| Self::tree_view_cursor_changed(tree));


        // Tab Overview -> Tree View "Prefetcher Threads"
        let tree_view_overview_prefetcher_threads: gtk::TreeView = builder.get_object("TreeViewOverviewPrefetcherThreads")
                                                                                .expect("Could not get an UI element!");

        // tree_view_overview_prefetcher_threads.set_headers_visible(true);

        // Self::append_column(&tree_view_overview_prefetcher_threads);
        Self::append_column(&tree_view_overview_prefetcher_threads);

        tree_view_overview_prefetcher_threads.set_model(&main_window.prefetcher_threads_model);
        tree_view_overview_prefetcher_threads.connect_cursor_changed(|tree| Self::tree_view_cursor_changed(tree));


        // Tab Overview -> Tree View "Events"
        let tree_view_overview_events: gtk::TreeView = builder.get_object("TreeViewOverviewEvents")
                                                                                .expect("Could not get an UI element!");

        // tree_view_overview_events.set_headers_visible(true);

        Self::append_column_with_id(&tree_view_overview_events, 0);
        Self::append_column_with_id(&tree_view_overview_events, 1);

        tree_view_overview_events.set_model(&main_window.events_model);
        tree_view_overview_events.connect_cursor_changed(|tree| Self::tree_view_cursor_changed(tree));


        // Tab Events -> Tree View "Events"
        let tree_view_events: gtk::TreeView = builder.get_object("TreeViewEvents")
                                                                    .expect("Could not get an UI element!");

        // tree_view_events.set_headers_visible(true);

        Self::append_column_with_id(&tree_view_events, 0);
        Self::append_column_with_id(&tree_view_events, 1);

        tree_view_events.set_model(&main_window.events_model);
        tree_view_events.connect_cursor_changed(|tree| Self::tree_view_cursor_changed(tree));


        // TODO: Statistics
        // // Tab Overview -> Tree View "Tracked Processes"
        // let tree_view_overview_tracked_processes: gtk::TreeView = builder.get_object("TreeViewOverviewTrackedProcesses")
        //                                                                     .expect("Could not get an UI element!");

        // // tree_view_overview_tracked_processes.set_headers_visible(true);

        // // Self::append_column(&tree_view_overview_tracked_processes);
        // Self::append_column(&tree_view_overview_tracked_processes);

        // tree_view_overview_tracked_processes.set_model(&main_window.tracked_processes_model);
        // tree_view_overview_tracked_processes.connect_cursor_changed(|tree| Self::tree_view_cursor_changed(tree));


        // // Tab Overview -> Tree View "Active Traces"
        // let tree_view_overview_active_traces: gtk::TreeView = builder.get_object("TreeViewOverviewActiveTraces")
        //                                                                     .expect("Could not get an UI element!");

        // // tree_view_overview_active_traces.set_headers_visible(true);

        // // Self::append_column(&tree_view_overview_active_traces);
        // Self::append_column(&tree_view_overview_active_traces);

        // tree_view_overview_active_traces.set_model(&main_window.active_traces_model);
        // tree_view_overview_active_traces.connect_cursor_changed(|tree| Self::tree_view_cursor_changed(tree));
    }

    fn append_column(tree: &TreeView) {
        let column = TreeViewColumn::new();
        let cell = CellRendererText::new();

        column.pack_start(&cell, true);
        
        column.add_attribute(&cell, "text", 0);
        tree.append_column(&column);
    }

    fn append_column_with_id(tree: &TreeView, id: i32) {
        let column = TreeViewColumn::new();
        let cell = CellRendererText::new();

        column.pack_start(&cell, true);
        
        column.add_attribute(&cell, "text", id);
        tree.append_column(&column);
    }
}