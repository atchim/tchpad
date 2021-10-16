use std::ops::Not;

use xcb::{
  Atom,
  Connection as Xcb,
  Window,
};

use xcb_util::ewmh::{
  Connection as Ewmh,
  GetClientListReply,
  GetClientListStackingReply,
  get_client_list,
  get_client_list_stacking,
  get_wm_state,
};

pub struct Tchpad {
  conn: Ewmh,
  ignored_atoms: Vec<Atom>,
  screen: i32,
  wins: Vec<Window>,
}

impl Tchpad {
  pub fn new() -> Self {
    let (conn, screen) = Xcb::connect(None).unwrap();
    let conn = Ewmh::connect(conn).map_err(|(e, _)| e).unwrap();
    let mut t = Tchpad {conn, ignored_atoms: vec![], screen, wins: vec![]};
    t.ignore_atoms();
    t.fetch_wins();
    t
  }

  fn fetch_win_atoms(&self, win: Window) -> Vec<Atom> {
    match get_wm_state(&self.conn, win).get_reply() {
      Ok(reply) => reply.atoms().iter().map(|a| *a).collect(),
      _ => vec![],
    }
  }

  fn fetch_wins(&mut self) {
    let cookie = get_client_list_stacking(&self.conn, self.screen);
    let list: GetClientListStackingReply;
    let stacking: GetClientListReply;

    let wins = match cookie.get_reply() {
      Ok(reply) => {
        list = reply;
        list.windows()
      }
      _ => {
        stacking = get_client_list(&self.conn, self.screen)
          .get_reply()
          .unwrap();
        stacking.windows()
      }
    };

    let wins = wins
      .iter()
      .filter_map(|win| self.is_win_ignored(*win).not().then(|| *win))
      .collect::<Vec<Window>>();

    self.wins.extend(wins);
  }

  pub fn get_wins(&self) -> &[Window] {
    &self.wins[..]
  }

  fn ignore_atoms(&mut self) {
    match self.ignored_atoms.len() {
      0 => self.ignored_atoms = vec![
        self.conn.WM_STATE_SKIP_PAGER(),
        self.conn.WM_STATE_SKIP_TASKBAR(),
        self.conn.WM_WINDOW_TYPE_DOCK(),
        self.conn.WM_WINDOW_TYPE_DESKTOP(),
      ],
      _ => (),
    }
  }

  fn is_win_ignored(&self, win: Window) -> bool {
    let ignored_atoms = self.fetch_win_atoms(win)
      .iter()
      .filter(|atom| self.ignored_atoms.contains(atom))
      .count();
    ignored_atoms > 0
  }
}
