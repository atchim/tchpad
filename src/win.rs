use std::cmp::Ordering::{Equal, Greater, Less};

use regex::{Captures, Regex};
use xcb::{Atom, Window};

use xcb_util::{
  ewmh::{Connection as Ewmh, get_wm_desktop, get_wm_name, get_wm_state},
  icccm::get_wm_class,
};

pub struct Win {
  class: String,
  desktop: String,
  hidden: bool,
  instance: String,
  name: String,
}

impl Win {
  pub fn display_value(&self, win_fmt: &str, hidden_fmt: &str) -> String {
    let r = Regex::new(r"\{(c|d|i|n)(:(\d+))?\}").unwrap();

    let s = match self.hidden() {
      false => win_fmt,
      true => hidden_fmt,
    };

    let s = r.replace_all(s, |caps: &Captures| {
      let prop = match &caps[1] {
        "c" => &self.class,
        "d" => &self.desktop,
        "i" => &self.instance,
        "n" => &self.name,
        _ => unreachable!(),
      };

      match caps.get(3) {
        None => prop.into(),
        Some(len) => {
          let cap_len: usize = len.as_str().parse().unwrap();
          let prop_len = prop.len();
          match cap_len.cmp(&prop_len) {
            Equal => prop.into(),
            Greater => format!("{}{}", prop, " ".repeat(cap_len - prop_len)),
            Less => prop[..cap_len].into(),
          }
        }
      }
    });

    s.into()
  }

  pub fn hidden(&self) -> bool {
    self.hidden
  }

  pub fn new<T: ToString>(
    id: Window,
    e: &Ewmh,
    desktops: &[T],
    hidden: &Atom,
    ignored: &[Atom],
  ) -> Option<Self> {
    let atoms = get_wm_state(e, id)
      .get_reply()
      .map(|s| s.atoms().iter().map(|a| *a).collect::<Vec<Atom>>())
      .unwrap_or(vec![]);
    if atoms.iter().filter(|a| ignored.contains(a)).count() > 0 {return None;}
    let hidden = atoms.contains(hidden);
    let class = get_wm_class(e, id).get_reply().unwrap();
    let instance = class.instance().to_owned();
    let class = class.class().to_owned();
    let n = get_wm_desktop(e, id).get_reply().unwrap();
    let desktop = desktops[n as usize].to_string();
    let name = get_wm_name(e, id).get_reply().unwrap().string().into();
    Some(Win {class, desktop, hidden, instance, name})
  }
}
