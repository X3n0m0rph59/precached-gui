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

use std::sync::{Arc, RwLock};
use ipc;

/// Global system state
#[derive(Clone)]
pub struct Globals {
    /// Global configuration
    pub data: Arc<RwLock<ipc::GlobalData>>,
}

impl Globals {
    pub fn new() -> Globals {
        Globals {
            data: Arc::new(RwLock::new(ipc::GlobalData::default())),
        }
    }
}
