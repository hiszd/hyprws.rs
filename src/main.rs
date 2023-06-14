#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod display;
mod events;

use display::Disp;
use events::CB;

extern crate strum; // 0.10.0
#[macro_use]
extern crate strum_macros; // 0.10.0
extern crate clap;
extern crate serde_derive;
extern crate serde_json;
use clap::Parser;
use serde_derive::{Deserialize, Serialize};
use serde_json::de::IoRead;
use serde_json::Value;
use std::{env, ops::IndexMut};
use std::{error::Error, process::Command};
use std::{
    fmt,
    io::{Read, Write},
};
use std::{fs::read, thread};
use std::{
    io,
    os::unix::net::{UnixListener, UnixStream},
};
use std::{
    io::{BufRead, BufReader},
    path::Path,
};
use strum::IntoEnumIterator;

use crate::events::CBType;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Aws {
    id: i32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mon {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonList {
    pub monitors: Vec<Mon>,
}

impl MonList {
    pub fn findById(&self, id: i32) -> usize {
        let mut idx: usize = 0;
        self.monitors.iter().enumerate().for_each(|(i, e)| {
            if e.id == id {
                idx = i;
            }
        });
        idx
    }
    pub fn findByName(&self, name: &str) -> usize {
        let mut idx: usize = 0;
        self.monitors.iter().enumerate().for_each(|(i, e)| {
            if e.name == name {
                idx = i;
            }
        });
        idx
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wksp {
    id: i32,
    name: String,
    monitor: String,
    windows: u32,
    hasfullscreen: bool,
    lastwindow: String,
    lastwindowtitle: String,
}

#[derive(Debug, Clone)]
pub struct WkspList {
    pub workspaces: Vec<Wksp>,
}

impl WkspList {
    pub fn findById(&self, id: i32) -> usize {
        let mut idx: usize = 0;
        self.workspaces.iter().enumerate().for_each(|(i, e)| {
            if e.id == id {
                idx = i;
            }
        });
        idx
    }
    pub fn findByName(&self, name: &str) -> usize {
        let mut idx: usize = 0;
        self.workspaces.iter().enumerate().for_each(|(i, e)| {
            if e.name == name {
                idx = i;
            }
        });
        idx
    }
    pub fn findByMonName(&self, name: &str) -> usize {
        let mut idx: usize = 0;
        self.workspaces.iter().enumerate().for_each(|(i, e)| {
            if e.monitor == name {
                idx = i;
            }
        });
        idx
    }
    pub fn filterByMonName(&self, name: &str) -> Vec<Wksp> {
        let mut ws = self.workspaces.clone();
        ws.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
        ws.into_iter()
            .filter(|e| (e.monitor == name && e.name != "special"))
            .collect()
    }
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = 0, short, long)]
    monitor: u32,
}

fn get_monitors() -> MonList {
    let monoutput = Command::new("/usr/bin/hyprctl")
        .arg("monitors")
        .arg("-j")
        .output()
        .unwrap();
    let monjson: Vec<Mon> = serde_json::from_slice(&monoutput.stdout).unwrap();
    MonList { monitors: monjson }
}

fn get_workspaces() -> WkspList {
    let wsoutput = Command::new("/usr/bin/hyprctl")
        .arg("workspaces")
        .arg("-j")
        .output()
        .unwrap();
    let wsjson: Vec<Wksp> = serde_json::from_slice(&wsoutput.stdout).unwrap();
    WkspList { workspaces: wsjson }
}

fn main() -> ! {
    // get hyprland instance for socket path
    let hyprland_instance = env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap();

    // get socket path
    let filepath = "/tmp/hypr/".to_owned() + &hyprland_instance + "/.socket2.sock";
    let path = Path::new(&filepath);

    let args = Args::parse();

    let mut mon = get_monitors();

    let mut ws = get_workspaces();

    let mut di = Disp::new(&mon.monitors[mon.findById(args.monitor as i32)].name);

    di.update(ws.clone(), mon.clone());

    loop {
        let mut strm = UnixStream::connect(path).unwrap();

        let stream = BufReader::new(strm);

        stream.lines().for_each(|e| {
            let arr = e.as_ref().unwrap().find(">>").unwrap();
            let x = &e.as_ref().unwrap()[0..arr];
            // let args: Vec<&str> = e.as_ref().unwrap()[(arr + 2)..].split(',').collect();
            match CB(x) {
                CBType::Monitors => mon = get_monitors(),
                CBType::Workspaces => ws = get_workspaces(),
                CBType::MandW => {
                    mon = get_monitors();
                    ws = get_workspaces()
                }
            }
            di.update(ws.clone(), mon.clone());
        })
    }
}
