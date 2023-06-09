#![allow(unused)]
#![allow(non_snake_case)]

extern crate clap;
extern crate serde_derive;
extern crate serde_json;
use clap::Parser;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::process::Command;
use std::thread;
use std::{env, ops::IndexMut};

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

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = 0, short, long)]
    monitor: u32,
}

fn handle_client(stream: UnixStream) {
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        println!("{}", line.unwrap());
    }
}

fn main() -> ! {
    let hyprland_instance = env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap();
    println!("{:?}", hyprland_instance);

    let path = "/tmp/hypr/".to_owned() + &hyprland_instance + "/.socket2.sock";

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

    std::fs::remove_file(&path).unwrap();

    let listener = UnixListener::bind(&path).unwrap();

    loop {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| handle_client(stream));
                }
                Err(err) => {
                    println!("Error: {}", err);
                    break;
                }
            }
        }
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
