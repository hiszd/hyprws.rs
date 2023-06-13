struct Workspace;
impl EventHandler for Workspace {
    fn EventHandler(&self) {}
}
struct Focusedmon;
impl EventHandler for Focusedmon {
    fn EventHandler(&self) {}
}
struct ActiveWindow;
impl EventHandler for ActiveWindow {
    fn EventHandler(&self) {}
}
struct ActiveWindowv2;
impl EventHandler for ActiveWindowv2 {
    fn EventHandler(&self) {}
}
struct Fullscreen;
impl EventHandler for Fullscreen {
    fn EventHandler(&self) {}
}
struct MonitorRemoved;
impl EventHandler for MonitorRemoved {
    fn EventHandler(&self) {}
}
struct MonitorAdded;
impl EventHandler for MonitorAdded {
    fn EventHandler(&self) {}
}
struct CreateWorkspace;
impl EventHandler for CreateWorkspace {
    fn EventHandler(&self) {}
}
struct DestroyWorkspace;
impl EventHandler for DestroyWorkspace {
    fn EventHandler(&self) {}
}
struct MoveWorkspace;
impl EventHandler for MoveWorkspace {
    fn EventHandler(&self) {}
}
struct ActiveLayout;
impl EventHandler for ActiveLayout {
    fn EventHandler(&self) {}
}
struct OpenWindow;
impl EventHandler for OpenWindow {
    fn EventHandler(&self) {}
}
struct CloseWindow;
impl EventHandler for CloseWindow {
    fn EventHandler(&self) {}
}
struct MoveWindow;
impl EventHandler for MoveWindow {
    fn EventHandler(&self) {}
}
struct OpenLayer;
impl EventHandler for OpenLayer {
    fn EventHandler(&self) {}
}
struct CloseLayer;
impl EventHandler for CloseLayer {
    fn EventHandler(&self) {}
}
struct Submap;
impl EventHandler for Submap {
    fn EventHandler(&self) {}
}
struct ChangeFloatingMode;
impl EventHandler for ChangeFloatingMode {
    fn EventHandler(&self) {}
}
struct Urgent;
impl EventHandler for Urgent {
    fn EventHandler(&self) {}
}
struct Minimize;
impl EventHandler for Minimize {
    fn EventHandler(&self) {}
}
struct Screencast;
impl EventHandler for Screencast {
    fn EventHandler(&self) {}
}
struct WindowTitle;
impl EventHandler for WindowTitle {
    fn EventHandler(&self) {}
}

#[derive(AsRefStr)]
enum Events {
    Workspace(Workspace),
    Focusedmon(Focusedmon),
    ActiveWindow(ActiveWindow),
    ActiveWindowv2(ActiveWindowv2),
    Fullscreen(Fullscreen),
    MonitorRemoved(MonitorRemoved),
    MonitorAdded(MonitorAdded),
    CreateWorkspace(CreateWorkspace),
    DestroyWorkspace(DestroyWorkspace),
    MoveWorkspace(MoveWorkspace),
    ActiveLayout(ActiveLayout),
    OpenWindow(OpenWindow),
    CloseWindow(CloseWindow),
    MoveWindow(MoveWindow),
    OpenLayer(OpenLayer),
    CloseLayer(CloseLayer),
    Submap(Submap),
    ChangeFloatingMode(ChangeFloatingMode),
    Urgent(Urgent),
    Minimize(Minimize),
    Screencast(Screencast),
    WindowTitle(WindowTitle),
}

pub trait EventHandler {
    fn EventHandler(&self);
}

pub struct Event<T>
where
    T: EventHandler,
{
    event: T,
}
