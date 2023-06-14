use crate::{MonList, Wksp, WkspList};

pub struct Disp {
    monname: String,
    parent: (String, String),
}

pub fn workspaceEl(wsnm: &str) -> String {
    "(button :onclick \"hyprctl dispatch workspace ".to_owned()
        + wsnm
        + "\" :onrightclick \"hyprctl dispatch workspace "
        + wsnm
        + " && /home/zion/.config/hypr/default_app\" :class \"wksp\" \"${ic["
        + wsnm
        + "]}\")"
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
        }
    }

    pub fn build(&self, ws: Vec<Wksp>) -> String {
        let wsels: Vec<String> = ws.into_iter().map(|e| e.name).collect();
        let mut wselsstr: String = String::new();
        wsels
            .into_iter()
            .for_each(|e| wselsstr = wselsstr.to_owned() + &workspaceEl(&e));
        println!("wsel {:?}", &wselsstr);
        self.parent.0.to_owned() + "\n" + &wselsstr + "\n" + &self.parent.1
    }

    pub fn update(&mut self, ws: WkspList, mons: MonList) {
        // monitor index
        let moni = mons.findByName(&self.monname);
        // workspace index
        let wsi = ws.filterByMonName(&self.monname);
        let wsel = self.build(wsi);
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
