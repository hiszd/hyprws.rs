#[derive(Debug)]
pub enum Events {
    Workspace,
    Focusedmon,
    ActiveWindow,
    ActiveWindowv2,
    Fullscreen,
    MonitorRemoved,
    MonitorAdded,
    CreateWorkspace,
    DestroyWorkspace,
    MoveWorkspace,
    ActiveLayout,
    OpenWindow,
    CloseWindow,
    MoveWindow,
    OpenLayer,
    CloseLayer,
    Submap,
    ChangeFloatingMode,
    Urgent,
    Minimize,
    Screencast,
    WindowTitle,
}

impl From<&str> for Events {
    fn from(value: &str) -> Self {
        match value {
            "workspace" => Events::Workspace,
            "focusedmon" => Events::Focusedmon,
            "activewindow" => Events::ActiveWindow,
            "activewindowv2" => Events::ActiveWindowv2,
            "fullscreen" => Events::Fullscreen,
            "monitorremoved" => Events::MonitorRemoved,
            "monitoradded" => Events::MonitorAdded,
            "createworkspace" => Events::CreateWorkspace,
            "destroyworkspace" => Events::DestroyWorkspace,
            "moveworkspace" => Events::MoveWorkspace,
            "activelayout" => Events::ActiveLayout,
            "openwindow" => Events::OpenWindow,
            "closewindow" => Events::CloseWindow,
            "movewindow" => Events::MoveWindow,
            "openlayer" => Events::OpenLayer,
            "closelayer" => Events::CloseLayer,
            "submap" => Events::Submap,
            "changefloatingmode" => Events::ChangeFloatingMode,
            "urgent" => Events::Urgent,
            "minimize" => Events::Minimize,
            "screencast" => Events::Screencast,
            "windowtitle" => Events::WindowTitle,
        }
    }
}

impl From<String> for Events {
    fn from(value: String) -> Self {
        match value.as_str() {
            "workspace" => Events::Workspace,
            "focusedmon" => Events::Focusedmon,
            "activewindow" => Events::ActiveWindow,
            "activewindowv2" => Events::ActiveWindowv2,
            "fullscreen" => Events::Fullscreen,
            "monitorremoved" => Events::MonitorRemoved,
            "monitoradded" => Events::MonitorAdded,
            "createworkspace" => Events::CreateWorkspace,
            "destroyworkspace" => Events::DestroyWorkspace,
            "moveworkspace" => Events::MoveWorkspace,
            "activelayout" => Events::ActiveLayout,
            "openwindow" => Events::OpenWindow,
            "closewindow" => Events::CloseWindow,
            "movewindow" => Events::MoveWindow,
            "openlayer" => Events::OpenLayer,
            "closelayer" => Events::CloseLayer,
            "submap" => Events::Submap,
            "changefloatingmode" => Events::ChangeFloatingMode,
            "urgent" => Events::Urgent,
            "minimize" => Events::Minimize,
            "screencast" => Events::Screencast,
            "windowtitle" => Events::WindowTitle,
        }
    }
}
