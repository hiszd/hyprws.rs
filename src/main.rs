#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

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
    let mut monjson: Vec<Mon> = serde_json::from_str(&monstr).unwrap();
    println!("{:?}", monjson[0]);

    let wsoutput = Command::new("/usr/bin/hyprctl")
        .arg("workspaces")
        .arg("-j")
        .output()
        .unwrap();

    let wsstr = String::from_utf8(wsoutput.stdout).unwrap();
    let mut wsjson: Vec<Wksp> = serde_json::from_str(&wsstr).unwrap();

    let mut stream = UnixStream::connect(path).unwrap();

    let stream = BufReader::new(stream);
    for line in stream.lines() {
        // println!("{:?}", line.as_ref().unwrap());
        let arr = line.as_ref().unwrap().find(">>").unwrap();
        let e = &line.as_ref().unwrap()[0..arr];
        let args: Vec<&str> = line.as_ref().unwrap()[(arr + 2)..].split(',').collect();
        let argsstring = args.join(" ");
        match e {
            "workspace" => {
                let wsnum: i32 = args[0].parse().unwrap();
                let wksp: &Wksp = wsjson.iter().find(|&e| e.id == wsnum).unwrap();
                monjson = monjson.iter().to_owned().map(|&e| {
                    if e.id = wksp.monitor {
                        e.activeWorkspace = wksp.id;
                    }
                    return e;
                });
                println!("{:?}", wksp.id);
                println!("workspace: {}\n", &argsstring);
            }
            "focusedmon" => {
                println!("focusedmon: {:?}\n", &argsstring);
            }
            "activewindow" => {
                println!("activewindow: {:?}\n", &argsstring);
            }
            "activewindowv2" => {
                println!("activewindowv2: {:?}\n", &argsstring);
            }
            "fullscreen" => {
                println!("fullscreen: {:?}\n", &argsstring);
            }
            "monitorremoved" => {
                println!("monitorremoved: {:?}\n", &argsstring);
            }
            "monitoradded" => {
                println!("monitoradded: {:?}\n", &argsstring);
            }
            "createworkspace" => {
                println!("createworkspace: {:?}\n", &argsstring);
            }
            "destroyworkspace" => {
                println!("destroyworkspace: {:?}\n", &argsstring);
            }
            "moveworkspace" => {
                println!("moveworkspace: {:?}\n", &argsstring);
            }
            "activelayout" => {
                println!("activelayout: {:?}\n", &argsstring);
            }
            "openwindow" => {
                println!("openwindow: {:?}\n", &argsstring);
            }
            "closewindow" => {
                println!("closewindow: {:?}\n", &argsstring);
            }
            "movewindow" => {
                println!("movewindow: {:?}\n", &argsstring);
            }
            "openlayer" => {
                println!("openlayer: {:?}\n", &argsstring);
            }
            "closelayer" => {
                println!("closelayer: {:?}\n", &argsstring);
            }
            "submap" => {
                println!("submap: {:?}\n", &argsstring);
            }
            "changefloatingmode" => {
                println!("changefloatingmode: {:?}\n", &argsstring);
            }
            "urgent" => {
                println!("urgent: {:?}\n", &argsstring);
            }
            "minimize" => {
                println!("minimize: {:?}\n", &argsstring);
            }
            "screencast" => {
                println!("screencast: {:?}\n", &argsstring);
            }
            "windowtitle" => {
                println!("windowtitle: {:?}\n", &argsstring);
            }
            _ => {
                println!("nothing\n");
            }
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
