use crate::{Mon, MonList, Wksp, WkspList};

pub fn workspaceEl(wsnm: &str, isfocused: bool) -> String {
    let mut class: String = String::from("wsnorm");
    if isfocused {
        class = "wsfocused".to_owned();
    }
    "(button :width 10 :onclick \"hyprctl dispatch workspace ".to_owned()
        + wsnm
        + "\" :onrightclick \"hyprctl dispatch workspace "
        + wsnm
        + " && /home/zion/.config/hypr/default_app\" :class \""
        + &class
        + "\" \""
        + wsnm
        + "\")"
}

struct WsStatus {
    name: String,
    isfocused: bool,
}

pub struct Disp {
    monname: String,
    parent: (String, String),
    bx: (String, String),
}

impl Disp {
    pub fn new(monitor: &str) -> Self {
        Disp {
            monname: monitor.to_owned(),
            parent: (
                // "(eventbox :onscroll \"hyprctl dispatch workspace\"".to_owned(),
                "(eventbox".to_owned(),
                ")".to_owned(),
            ),
            bx: (
                "(box :halign \"start\" :class \"works\" :orientation \"h\" :space-evenly \"true\""
                    .to_owned(),
                ")".to_owned(),
            ),
        }
    }

    pub fn build(&self, ws: Vec<Wksp>, mon: Mon) -> String {
        let mut wsself: Vec<Wksp> = ws.clone();
        wsself.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
        let wsels: Vec<WsStatus> = ws
            .into_iter()
            .map(|e| WsStatus {
                name: e.name.to_owned(),
                isfocused: (mon.activeWorkspace.name == e.name),
            })
            .collect();
        let mut wselsstr: String = String::new();
        wsels.into_iter().for_each(|e| {
            wselsstr = wselsstr.to_owned() + &workspaceEl(&e.name, e.isfocused);
        });
        // println!("wsel {:?}", &wselsstr);
        self.parent.0.to_owned() + &self.bx.0 + &wselsstr + &self.bx.1 + &self.parent.1
    }

    pub fn update(&mut self, ws: WkspList, mons: MonList) {
        // monitor index
        let moni = mons.findByName(&self.monname);
        // workspace index
        let wsi = ws.filterByMonName(&self.monname);
        let wsel = self.build(wsi, mons.monitors[moni].clone());
        println!("{}", wsel);
    }
}

// "(eventbox :onscroll \"echo {} | sed -e 's/up/-1/g' -e 's/down/+1/g' | xargs hyprctl dispatch workspace\" \
//   (box	:class \"works\"	:orientation \"h\" :space-evenly \"true\" 	\
//       (button :onclick \"hyprctl dispatch workspace 1\" :onrightclick \"hyprctl dispatch workspace 1 && /home/taylor/.config/hypr/default_app\" :class \"r$o1$f1\" \"${ic[1]}\") \
//       (button :onclick \"hyprctl dispatch workspace 2\" :onrightclick \"hyprctl dispatch workspace 2 && /home/taylor/.config/hypr/default_app\"	:class \"r$o2$f2\" \"${ic[2]}\") \
//       (button :onclick \"hyprctl dispatch workspace 3\" :onrightclick \"hyprctl dispatch workspace 3 && /home/taylor/.config/hypr/default_app\"	:class \"r$o3$f3\" \"${ic[3]}\") \
//       (button :onclick \"hyprctl dispatch workspace 4\" :onrightclick \"hyprctl dispatch workspace 4 && /home/taylor/.config/hypr/default_app\"	:class \"r$o4$f4\" \"${ic[4]}\") \
//       (button :onclick \"hyprctl dispatch workspace 5\" :onrightclick \"hyprctl dispatch workspace 5 && /home/taylor/.config/hypr/default_app\"	:class \"r$o5$f5\" \"${ic[5]}\") \
//       (button :onclick \"hyprctl dispatch workspace 6\" :onrightclick \"hyprctl dispatch workspace 6 && /home/taylor/.config/hypr/default_app\"	:class \"r$o6$f6\" \"${ic[6]}\") \
//       (button :onclick \"hyprctl dispatch workspace 7\" :onrightclick \"hyprctl dispatch workspace 7 && /home/taylor/.config/hypr/default_app\"	:class \"r$o7$f7\" \"${ic[7]}\") \
//   )\
// )"
