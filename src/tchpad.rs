use xcb::{Atom, Connection as Xcb};

use xcb_util::ewmh::{
  Connection as Ewmh,
  GetClientListReply,
  GetClientListStackingReply,
  get_client_list,
  get_client_list_stacking,
  get_desktop_names,
};

use crate::win::Win;

pub struct Tchpad {
  desktops: Vec<String>,
  e: Ewmh,
  ignored_atoms: Vec<Atom>,
  screen: i32,
  win_fmt: String,
  wins: Vec<Win>,
}

impl Tchpad {
  fn fetch_desktops(&mut self) {
    let reply = get_desktop_names(&self.e, self.screen).get_reply().unwrap();
    self.desktops = reply.strings()
      .iter()
      .map(|s| String::from(*s))
      .collect();
  }

  fn fetch_wins(&mut self) {
    let cookie = get_client_list_stacking(&self.e, self.screen);
    let list: GetClientListStackingReply;
    let stacking: GetClientListReply;

    let ids = match cookie.get_reply() {
      Ok(reply) => {
        list = reply;
        list.windows()
      }
      _ => {
        stacking = get_client_list(&self.e, self.screen)
          .get_reply()
          .unwrap();
        stacking.windows()
      }
    };

    self.wins = ids
      .iter()
      .filter_map(
        |id| Win::new(*id, &self.e, &self.ignored_atoms, &self.desktops)
      )
      .collect();
  }

  fn ignore_atoms(&mut self) {
    self.ignored_atoms = vec![
      self.e.WM_STATE_SKIP_PAGER(),
      self.e.WM_STATE_SKIP_TASKBAR(),
      self.e.WM_WINDOW_TYPE_DOCK(),
      self.e.WM_WINDOW_TYPE_DESKTOP(),
    ];
  }

  pub fn new(win_fmt: &str) -> Self {
    let (x, screen) = Xcb::connect(None).unwrap();
    let e = Ewmh::connect(x).map_err(|(err, _)| err).unwrap();

    let mut t = Tchpad {
      desktops: vec![],
      e,
      ignored_atoms: vec![],
      screen,
      win_fmt: String::from(win_fmt),
      wins: vec![],
    };

    t.ignore_atoms();
    t.fetch_desktops();
    t.fetch_wins();
    t
  }

  pub fn switch_hidden(&mut self, index: usize) {
    self.wins[index].switch_hidden(&self.e);
  }

  pub fn win_fmt(&self) -> &str {
    &self.win_fmt
  }
  pub fn wins(&self) -> &[Win] {
    &self.wins[..]
  }
}
