#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod events;

use events::Event;

extern crate strum; // 0.10.0
#[macro_use]
extern crate strum_macros; // 0.10.0
extern crate clap;
extern crate serde_derive;
extern crate serde_json;
use clap::Parser;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::thread;
use std::{env, ops::IndexMut};
use std::{error::Error, process::Command};
use std::{
    fmt,
    io::{Read, Write},
};
use std::{
    io,
    os::unix::net::{UnixListener, UnixStream},
};
use std::{
    io::{BufRead, BufReader},
    path::Path,
};
use strum::IntoEnumIterator;

use crate::events::{EventBuilder, EventHandler};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
struct Aws<'a> {
    id: i32,
    name: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
struct Mon<'a> {
    id: i32,
    name: &'a str,
    description: &'a str,
    make: &'a str,
    model: &'a str,
    serial: &'a str,
    width: u32,
    height: u32,
    refreshRate: f32,
    x: u32,
    y: u32,
    activeWorkspace: Aws<'a>,
    reserved: [u32; 4],
    scale: f32,
    transform: u32,
    focused: bool,
    dpmsStatus: bool,
    vrr: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonList<'a> {
    pub monitors: Vec<Mon<'a>>,
}

impl MonList<'_> {
    pub fn find(&self) {}
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
struct Wksp<'a> {
    id: i32,
    name: &'a str,
    monitor: &'a str,
    windows: u32,
    hasfullscreen: bool,
    lastwindow: &'a str,
    lastwindowtitle: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WkspList<'a> {
    pub workspaces: Vec<Wksp<'a>>,
}

impl WkspList<'_> {
    pub fn find(&self) {}
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = 0, short, long)]
    monitor: u32,
}

fn main() -> ! {
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
    let mut mon: Vec<Mon> = serde_json::from_str(&monstr).unwrap();
    // println!("{:?}", monjson[0]);

    let wsoutput = Command::new("/usr/bin/hyprctl")
        .arg("workspaces")
        .arg("-j")
        .output()
        .unwrap();

    let wsstr = String::from_utf8(wsoutput.stdout).unwrap();
    let mut ws: WkspList = serde_json::from_str(&wsstr).unwrap();

    loop {
        let mut strm = UnixStream::connect(path).unwrap();

        let stream = BufReader::new(strm);

        stream.lines().for_each(|e| {
            let arr = e.as_ref().unwrap().find(">>").unwrap();
            let x = &e.as_ref().unwrap()[0..arr];
            let args: Vec<&str> = e.as_ref().unwrap()[(arr + 2)..].split(',').collect();
            let e = EventBuilder::new(x);
            let (wsn, monn) = e.EventHandler(ws, mon);
            println!("{}", x);
        })
    }

    // if let Some(exit_code) = monoutput.status.code() {
    //     if exit_code == 0 {
    //         println!("Ok.");
    //     } else {
    //         eprintln!("Failed.");
    //     }
    // } else if let Some(exit_code) = wsoutput.status.code() {
    //     if exit_code == 0 {
    //         println!("Ok.");
    //     } else {
    //         eprintln!("Failed.");
    //     }
    // } else {
    //     eprintln!("Interrupted!");
    // }
}
