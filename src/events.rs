use crate::{Aws, Mon, MonList, Wksp, WkspList};

#[derive(Default)]
pub struct Workspace;
impl EventHandler for Workspace {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct Focusedmon;
impl EventHandler for Focusedmon {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct ActiveWindow;
impl EventHandler for ActiveWindow {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct ActiveWindowv2;
impl EventHandler for ActiveWindowv2 {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct Fullscreen;
impl EventHandler for Fullscreen {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct MonitorRemoved;
impl EventHandler for MonitorRemoved {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct MonitorAdded;
impl EventHandler for MonitorAdded {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct CreateWorkspace;
impl EventHandler for CreateWorkspace {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct DestroyWorkspace;
impl EventHandler for DestroyWorkspace {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct MoveWorkspace;
impl EventHandler for MoveWorkspace {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct ActiveLayout;
impl EventHandler for ActiveLayout {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct OpenWindow;
impl EventHandler for OpenWindow {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct CloseWindow;
impl EventHandler for CloseWindow {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct MoveWindow;
impl EventHandler for MoveWindow {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct OpenLayer;
impl EventHandler for OpenLayer {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct CloseLayer;
impl EventHandler for CloseLayer {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct Submap;
impl EventHandler for Submap {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct ChangeFloatingMode;
impl EventHandler for ChangeFloatingMode {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct Urgent;
impl EventHandler for Urgent {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct Minimize;
impl EventHandler for Minimize {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct Screencast;
impl EventHandler for Screencast {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}
#[derive(Default)]
pub struct WindowTitle;
impl EventHandler for WindowTitle {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList) {
        let wsid = ws.findById(args[0].parse::<i32>().unwrap());
        let mut aws: Aws = Aws {
            id: ws.workspaces[wsid].id,
            name: ws.workspaces[wsid].name,
        };
        let monnm = ws.workspaces[wsid].monitor;
        let monr: MonList = MonList {
            monitors: mon
                .monitors
                .iter()
                .map(|e| {
                    if e.name == monnm {
                        e.activeWorkspace = aws;
                    }
                    return e.to_owned();
                })
                .collect(),
        };
        return (ws, monr);
    }
}

pub struct EventBuilder;

impl EventBuilder {
    pub fn new(e: &str) -> Box<dyn EventHandler> {
        match e {
            "workspace" => Box::new(Workspace {}),
            "focusedmon" => Box::new(Focusedmon {}),
            "activewindow" => Box::new(ActiveWindow {}),
            "activewindowv2" => Box::new(ActiveWindowv2 {}),
            "fullscreen" => Box::new(Fullscreen {}),
            "montorremoved" => Box::new(MonitorRemoved {}),
            "montoradded" => Box::new(MonitorAdded {}),
            "createworkspace" => Box::new(CreateWorkspace {}),
            "destroyworkspace" => Box::new(DestroyWorkspace {}),
            "moveworkspace" => Box::new(MoveWorkspace {}),
            "activelayout" => Box::new(ActiveLayout {}),
            "openwindow" => Box::new(OpenWindow {}),
            "closewindow" => Box::new(CloseWindow {}),
            "movewindow" => Box::new(MoveWindow {}),
            "openlayer" => Box::new(OpenLayer {}),
            "closelayer" => Box::new(CloseLayer {}),
            "submap" => Box::new(Submap {}),
            "changefloatingmode" => Box::new(ChangeFloatingMode {}),
            "urgent" => Box::new(Urgent {}),
            "minimize" => Box::new(Minimize {}),
            "screencast" => Box::new(Screencast {}),
            "windowtitle" => Box::new(WindowTitle {}),
            _ => Box::new(WindowTitle {}),
        }
    }
}

pub trait EventHandler {
    fn EventHandler(&self, ws: WkspList, mon: MonList, args: Vec<&str>) -> (WkspList, MonList);
}

pub struct Event<T>
where
    T: EventHandler,
{
    pub event: T,
}
