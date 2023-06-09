#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate clap;
extern crate serde_derive;
extern crate serde_json;
use clap::Parser;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::os::unix::net::{UnixListener, UnixStream};
use std::process::Command;
use std::thread;
use std::{env, ops::IndexMut};
use std::{
    fmt,
    io::{Read, Write},
};
use std::{
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug, Serialize, Deserialize)]
struct Aws {
    id: u32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Mon {
    id: i32,
    name: String,
    description: String,
    make: String,
    model: String,
    serial: String,
    width: u32,
    height: u32,
    refreshRate: f32,
    x: u32,
    y: u32,
    activeWorkspace: Aws,
    reserved: [u32; 4],
    scale: f32,
    transform: u32,
    focused: bool,
    dpmsStatus: bool,
    vrr: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Wksp {
    id: i32,
    name: String,
    monitor: String,
    windows: u32,
    hasfullscreen: bool,
    lastwindow: String,
    lastwindowtitle: String,
}

#[derive(Clone, Copy, Debug)]
enum Events {
    workspace,
    focusedmon,
    activewindow,
    activewindowv2,
    fullscreen,
    monitorremoved,
    monitoradded,
    createworkspace,
    destroyworkspace,
    moveworkspace,
    activelayout,
    openwindow,
    closewindow,
    movewindow,
    openlayer,
    closelayer,
    submap,
    changefloatingmode,
    urgent,
    minimize,
    screencast,
    windowtitle,
}

impl std::convert::From<&str> for Events {
    fn from(value: &str) -> Self {
        for e in Events {
            if value == e.to_string() {
                return e;
            }
        }
    }
}

impl fmt::Display for Events {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Events::workspace => write!(f, "workspace"),
            Events::focusedmon => write!(f, "focusedmon"),
            Events::activewindow => write!(f, "activewindow"),
            Events::activewindowv2 => write!(f, "activewindowv2"),
            Events::fullscreen => write!(f, "fullscreen"),
            Events::monitorremoved => write!(f, "monitorremoved"),
            Events::monitoradded => write!(f, "monitoradded"),
            Events::createworkspace => write!(f, "createworkspace"),
            Events::destroyworkspace => write!(f, "destroyworkspace"),
            Events::moveworkspace => write!(f, "moveworkspace"),
            Events::activelayout => write!(f, "activelayout"),
            Events::openwindow => write!(f, "openwindow"),
            Events::closewindow => write!(f, "closewindow"),
            Events::movewindow => write!(f, "movewindow"),
            Events::openlayer => write!(f, "openlayer"),
            Events::closelayer => write!(f, "closelayer"),
            Events::submap => write!(f, "submap"),
            Events::changefloatingmode => write!(f, "changefloatingmode"),
            Events::urgent => write!(f, "urgent"),
            Events::minimize => write!(f, "minimize"),
            Events::screencast => write!(f, "screencast"),
            Events::windowtitle => write!(f, "windowtitle"),
        }
    }
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = 0, short, long)]
    monitor: u32,
}

fn main() {
    // get hyprland instance for socket path
    let hyprland_instance = env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap();

    // get socket path
    let filepath = "/tmp/hypr/".to_owned() + &hyprland_instance + "/.socket2.sock";
    let path = Path::new(&filepath);

    let args = Args::parse();

    let monoutput = Command::new("/usr/bin/hyprctl")
        .arg("monitors")
        .arg("-j")
        .output()
        .unwrap();

    let monstr = String::from_utf8(monoutput.stdout).unwrap();
    let monjson: Vec<Mon> = serde_json::from_str(&monstr).unwrap();

    let wsoutput = Command::new("/usr/bin/hyprctl")
        .arg("workspaces")
        .arg("-j")
        .output()
        .unwrap();

    let wsstr = String::from_utf8(wsoutput.stdout).unwrap();
    let wsjson: Vec<Wksp> = serde_json::from_str(&wsstr).unwrap();

    let mut stream = UnixStream::connect(path).unwrap();

    let stream = BufReader::new(stream);
    for line in stream.lines() {
        println!("{:?}", line.unwrap());
        let arr = line.unwrap().find(">>");
        let e: Events = line.unwrap()[0..arr].into();
        match e {
            Events::workspace => {}
            Events::focusedmon => {}
            Events::activewindow => {}
            Events::activewindowv2 => {}
            Events::fullscreen => {}
            Events::monitorremoved => {}
            Events::monitoradded => {}
            Events::createworkspace => {}
            Events::destroyworkspace => {}
            Events::moveworkspace => {}
            Events::activelayout => {}
            Events::openwindow => {}
            Events::closewindow => {}
            Events::movewindow => {}
            Events::openlayer => {}
            Events::closelayer => {}
            Events::submap => {}
            Events::changefloatingmode => {}
            Events::urgent => {}
            Events::minimize => {}
            Events::screencast => {}
            Events::windowtitle => {}
        }
    }

    if let Some(exit_code) = monoutput.status.code() {
        if exit_code == 0 {
            println!("Ok.");
        } else {
            eprintln!("Failed.");
        }
    } else if let Some(exit_code) = wsoutput.status.code() {
        if exit_code == 0 {
            println!("Ok.");
        } else {
            eprintln!("Failed.");
        }
    } else {
        eprintln!("Interrupted!");
    }
}
