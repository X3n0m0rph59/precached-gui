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

extern crate libc;
extern crate serde;
extern crate serde_json;
extern crate chrono;

use chrono::{DateTime, Utc};
use std::sync::{Arc, RwLock};
use std::path::{PathBuf};
use zmq;

use ipc;

/// Represents a process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Process {
    pub pid: libc::pid_t,
    pub comm: String,
}

/// Represents an in-flight trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracerEntry {
    pub start_time: DateTime<Utc>,
    pub trace_time_expired: bool,
    pub exe: PathBuf,
}

/// The states a prefetcher thread can be in
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreadState {
    Uninitialized,
    Idle,
    Error(PathBuf),
    PrefetchingFile(PathBuf),
    PrefetchingFileMetadata(PathBuf),
    UnmappingFile(PathBuf),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefetchStats {
    pub datetime: DateTime<Utc>,
    pub thread_states: Vec<ThreadState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalEvent {
    pub datetime: DateTime<Utc>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    pub datetime: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IpcCommand {
    Ping,
    Pong,

    Connect,
    ConnectedSuccessfuly,
    Close,

    RequestTrackedProcesses,
    SendTrackedProcesses(Vec<Process>),

    RequestInFlightTracers,
    SendInFlightTracers(Vec<TracerEntry>),

    RequestPrefetchStatus,
    SendPrefetchStatus(PrefetchStats),

    RequestInternalEvents,
    SendInternalEvents(Vec<InternalEvent>),

    RequestCachedFiles,
    SendCachedFiles(Vec<PathBuf>),

    RequestStatistics,
    SendStatistics(Vec<Statistics>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcMessage {
    pub datetime: DateTime<Utc>,
    pub command: IpcCommand,
}

impl IpcMessage {
    pub fn new(command: IpcCommand) -> IpcMessage {
        IpcMessage {
            datetime: Utc::now(),
            command: command,
        }
    }
}

/// Represents a tracked process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessListItem {
    pub pid: libc::pid_t,
    pub comm: String,
}

/// Represents an active trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracerListItem {
    pub start_time: DateTime<Utc>,
    pub trace_time_expired: bool,
    pub exe: PathBuf,
}

/// Represents an InternalEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventListItem {
    pub datetime: DateTime<Utc>,
    pub msg: String,
}

/// Represents an item in the statistics view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsListItem {
    pub datetime: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct GlobalData {
    /// Vec of in flight traces
    pub tracked_processes: Vec<ProcessListItem>,
    pub active_traces: Vec<TracerListItem>,
    /// Prefetcher threads states
    pub prefetch_stats: Option<ipc::PrefetchStats>,
    /// Vec of daemon internal events
    pub events: Vec<EventListItem>,
    /// Vec of currently cached files
    pub cached_files: Vec<PathBuf>,    
    /// Vec of statistics
    pub statistics: Vec<StatisticsListItem>,
}

impl Default for GlobalData {
    fn default() -> GlobalData {
        GlobalData { 
            tracked_processes: vec![],  
            active_traces: vec![],
            prefetch_stats: None,
            events: vec![],
            cached_files: vec![],
            statistics: vec![],
        }
    }
}

pub fn ipc_thread_main(global_data: Arc<RwLock<GlobalData>>) {
    let ctx = zmq::Context::new();
    match ctx.socket(zmq::REQ) {
        Ok(socket) => {
            match socket.connect("ipc:///run/precached.sock") {
                Ok(_) => {
                    // Send initial connection request
                    match do_request(&socket, ipc::IpcCommand::Connect) {
                        Ok(_data) => {            
                            trace!("Request succeeded: {}", stringify!($command));
                        }

                        Err(e) => {            
                            error!("Request failed: {}", e);
                        }
                    }

                    'IPC_LOOP: loop {
                        macro_rules! request {
                            ($socket:ident, $command:expr) => {
                                match do_request(&$socket, $command) {
                                        Ok(data) => {                            
                                            trace!("Request succeeded: {}", stringify!($command));

                                            let mut global_data = global_data.write()
                                                                    .expect("Could not lock a global data structure!");
                                            ipc::process_message(&mut global_data, data);
                                        },

                                        Err(e) => {
                                            error!("Request failed: {}", e);
                                        }
                                    }
                                };
                        }

                        // Request current data
                        request!(socket, ipc::IpcCommand::RequestTrackedProcesses);

                        // Request currently traced processes
                        request!(socket, ipc::IpcCommand::RequestInFlightTracers);

                        // Request states of prefetcher threads
                        request!(socket, ipc::IpcCommand::RequestPrefetchStatus);

                        // Request daemon internal events
                        request!(socket, ipc::IpcCommand::RequestInternalEvents);

                        // Request cached files
                        request!(socket, ipc::IpcCommand::RequestCachedFiles);
                    }
                }
                
                Err(e) => {
                    error!("Could not connect to socket: {}", e);
                }
            }
        }

        Err(e) => {
            error!("Could not create socket: {}", e);
        }
    }
}

fn do_request(socket: &zmq::Socket, command: ipc::IpcCommand) -> Result<ipc::IpcMessage, String> {
    let cmd = ipc::IpcMessage::new(command);
    let buf = serde_json::to_string(&cmd).expect("Could not serialize data!");

    match socket.send(&buf, 0) {
        Ok(()) => {
            // Receive the daemon's reply
            match socket.recv_string(0) {
                Ok(data) => match data {
                    Ok(data) => {
                        let deserialized_data: ipc::IpcMessage = serde_json::from_str(&data)
                                                                    .expect("Could not deserialize data!");

                        Ok(deserialized_data)
                    }

                    Err(e) => Err(format!("Invalid data received: {:?}", e)),
                },

                Err(e) => Err(format!("Could not receive data from socket: {}", e)),
            }
        }

        Err(e) => Err(format!("Could not send data via a socket: {}", e)),
    }
}

pub fn process_message(data: &mut GlobalData, msg: ipc::IpcMessage) {
    match msg.command {
        ipc::IpcCommand::Connect => {
            info!("IPC connected succesfuly!");
        }

        ipc::IpcCommand::SendTrackedProcesses(processes) => {
            let mut tmp = vec![];
            for p in processes {
                let i = ProcessListItem {
                    pid: p.pid,
                    comm: p.comm,
                };

                tmp.push(i);
            }

            data.tracked_processes = tmp;
        }

        ipc::IpcCommand::SendInFlightTracers(tracers) => {
            let mut tmp = vec![];
            for a in tracers {
                let i = TracerListItem {
                    start_time: a.start_time,
                    trace_time_expired: a.trace_time_expired,
                    exe: a.exe,
                };

                tmp.push(i);
            }

            data.active_traces = tmp;
        }

        ipc::IpcCommand::SendPrefetchStatus(stats) => {
            data.prefetch_stats = Some(stats);
        }

        ipc::IpcCommand::SendCachedFiles(files) => {
            data.cached_files = files;
        }

        ipc::IpcCommand::SendInternalEvents(events) => {
            let mut tmp = vec![];
            for e in events {
                let i = EventListItem {
                    datetime: e.datetime,
                    msg: e.name,
                };
                tmp.push(i);
            }

            data.events.append(&mut tmp);
        }

        ipc::IpcCommand::SendStatistics(stats) => {
            let mut tmp = vec![];
            for e in stats {
                let i = StatisticsListItem {
                    datetime: e.datetime,
                };
                tmp.push(i);
            }

            data.statistics.append(&mut tmp);
        }

        _ => {
            warn!("Unknown IPC command received");
        }
    }
}
