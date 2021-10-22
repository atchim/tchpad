use std::{cmp::Ordering::{Equal, Greater, Less}, ffi::CString};

use cairo::ffi::cairo_surface_t;
use regex::{Captures, Regex};
use rofi_sys::icon_fetcher::{rofi_icon_fetcher_get, rofi_icon_fetcher_query};
use xcb::{Atom, CURRENT_TIME, Window, ffi::xcb_window_t};

use xcb_util::{
  ewmh::{
    CLIENT_SOURCE_TYPE_OTHER,
    Connection as Ewmh,
    get_current_desktop,
    get_wm_desktop,
    get_wm_name,
    get_wm_state,
    request_change_active_window,
    request_change_current_desktop,
    request_close_window,
  },
  ffi::ewmh::XCB_EWMH_CLIENT_SOURCE_TYPE_OTHER,
  icccm::get_wm_class,
};

pub struct Win {
  class: String,
  desktop: String,
  desktop_n: u32,
  hidden: bool,
  icon: u32,
  id: Atom,
  instance: String,
  name: String,
}

impl Win {
  pub fn class(&self) -> &str {
    &self.class
  }

  pub fn close(&self, e: &Ewmh, screen: i32) {
    let req = request_close_window(
      e,
      screen,
      self.id,
      CURRENT_TIME,
      CLIENT_SOURCE_TYPE_OTHER,
    );
    req.request_check().unwrap();
  }

  pub fn desktop(&self) -> &str {
    &self.desktop
  }

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

  // NOTE: Implement a more advanced icon fetching.
  pub fn fetch_icon(&mut self, size: i32) -> *mut cairo_surface_t {
    if self.icon == 0 {
      let class = self.class.to_lowercase();
      let class = CString::new(class).unwrap();
      self.icon = unsafe {rofi_icon_fetcher_query(class.as_ptr(), size)};
    }
    unsafe {rofi_icon_fetcher_get(self.icon)}
  }

  pub fn focus(&self, e: &Ewmh, screen: i32) {
    let cur_desktop = get_current_desktop(e, screen).get_reply().unwrap();
    if cur_desktop != self.desktop_n {
      request_change_current_desktop(e, screen, cur_desktop, CURRENT_TIME)
        .request_check().unwrap();
    }

    let req = request_change_active_window(
      e,
      screen,
      self.id,
      XCB_EWMH_CLIENT_SOURCE_TYPE_OTHER,
      CURRENT_TIME,
      unsafe {rofi_view_get_window()},
    );
    req.request_check().unwrap();
  }

  pub fn hidden(&self) -> bool {
    self.hidden
  }

  pub fn id(&self) -> Window {
    self.id
  }

  pub fn instance(&self) -> &str {
    &self.instance
  }

  pub fn name(&self) -> &str {
    &self.name
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
      .map(|s| s.atoms().iter().copied().collect::<Vec<Atom>>())
      .unwrap_or_default();

    match atoms.iter().filter(|a| ignored.contains(a)).count() > 0 {
      false => {
        let class = get_wm_class(e, id).get_reply().unwrap();
        let instance = class.instance().to_owned();
        let class = class.class().to_owned();
        let desktop_n = get_wm_desktop(e, id).get_reply().unwrap();
        let desktop = desktops[desktop_n as usize].to_string();
        let hidden = atoms.contains(hidden);
        let icon = 0;
        let name = get_wm_name(e, id).get_reply().unwrap().string().into();
        Some(Win {class, desktop, desktop_n, hidden, icon, id, instance, name})
      }
      true => None,
    }
  }
}

extern "C" {
  // NOTE: Little hack to get ID of the Rofi window.
  pub fn rofi_view_get_window() -> xcb_window_t;
}
