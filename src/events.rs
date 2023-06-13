pub enum CBType {
    Monitors = 0,
    Workspaces = 1,
    MandW = 2,
}

pub fn CB(e: &str) -> CBType {
    match e {
        "workspace" => CBType::MandW,
        "focusedmon" => CBType::MandW,
        "activewindow" => CBType::MandW,
        "activewindowv2" => CBType::MandW,
        "fullscreen" => CBType::MandW,
        "montorremoved" => CBType::MandW,
        "montoradded" => CBType::MandW,
        "createworkspace" => CBType::MandW,
        "destroyworkspace" => CBType::MandW,
        "moveworkspace" => CBType::MandW,
        "activelayout" => CBType::MandW,
        "openwindow" => CBType::MandW,
        "closewindow" => CBType::MandW,
        "movewindow" => CBType::MandW,
        "openlayer" => CBType::MandW,
        "closelayer" => CBType::MandW,
        "submap" => CBType::MandW,
        "changefloatingmode" => CBType::MandW,
        "urgent" => CBType::MandW,
        "minimize" => CBType::MandW,
        "screencast" => CBType::MandW,
        "windowtitle" => CBType::MandW,
        _ => CBType::MandW,
    }
}
