use xcb::{Atom, Connection as Xcb};

use xcb_util::ewmh::{
  Connection as Ewmh,
  GetClientListReply,
  GetClientListStackingReply,
  get_client_list,
  get_client_list_stacking,
  get_desktop_names,
};

use crate::{opts::Opts, win::Win};

pub struct Tchpad {
  desktops: Vec<String>,
  e: Ewmh,
  hidden_atom: Atom,
  ignored_atoms: Vec<Atom>,
  opts: Opts,
  screen: i32,
  wins: Vec<Win>,
}

impl Tchpad {
  pub fn e(&self) -> &Ewmh {
    &self.e
  }

  fn fetch_desktops(&mut self) {
    let reply = get_desktop_names(&self.e, self.screen).get_reply().unwrap();
    self.desktops = reply.strings().iter().map(|s| s.to_string()).collect();
  }

  fn fetch_ignored_atoms(&mut self) {
    self.ignored_atoms = vec![
      self.e.WM_STATE_SKIP_PAGER(),
      self.e.WM_STATE_SKIP_TASKBAR(),
      self.e.WM_WINDOW_TYPE_DOCK(),
      self.e.WM_WINDOW_TYPE_DESKTOP(),
    ];
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
      .filter_map(|id| Win::new(
        *id,
        &self.e,
        &self.desktops,
        &self.hidden_atom,
        &self.ignored_atoms,
      ))
      .collect();
  }

  pub fn opts(&self) -> &Opts {
    &self.opts
  }

  pub fn screen(&self) -> i32 {
    self.screen
  }

  pub fn win(&mut self, index: usize) -> &mut Win {
    &mut self.wins[index]
  }

  pub fn wins(&self) -> &[Win] {
    &self.wins[..]
  }
}

impl Default for Tchpad {
  fn default() -> Self {
    let (x, screen) = Xcb::connect(None).unwrap();
    let e = Ewmh::connect(x).map_err(|(err, _)| err).unwrap();

    let mut t = Tchpad {
      desktops: vec![],
      hidden_atom: e.WM_STATE_HIDDEN(),
      e,
      ignored_atoms: vec![],
      opts: Opts::default(),
      screen,
      wins: vec![],
    };

    t.fetch_ignored_atoms();
    t.fetch_desktops();
    t.fetch_wins();
    t
  }
}
