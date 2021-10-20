use xcb::{Atom, Window, map_window, unmap_window};

use xcb_util::{
  ewmh::{Connection as Ewmh, get_wm_desktop, get_wm_name, get_wm_state},
  icccm::get_wm_class,
};

pub struct Win {
  atoms: Vec<Atom>,
  class: String,
  desktop: String,
  hidden_atom: Atom,
  id: Window,
  instance: String,
  name: String,
  urgent_atom: Atom,
}

impl Win {
  pub fn display_str(&self, fmt: &str) -> String {
    // TODO: Find out a better way to do this.
    let s = fmt.replacen("{c}", &self.class, 1)
      .replacen("{n}", &self.instance, 1)
      .replacen("{r}", "", 1) // NOTE: Not implemented yet.
      .replacen("{t}", &self.name, 1)
      .replacen("{w}", &self.desktop, 1);

    match self.hidden() {
      false => s,
      true => format!("[{}]", &s),
    }
  }

  fn fetch_atoms(e: &Ewmh, id: Window) -> Vec<Atom> {
    get_wm_state(e, id)
      .get_reply()
      .map(|s| s.atoms().iter().map(|a| *a).collect::<Vec<Atom>>())
      .unwrap_or(vec![])
  }

  fn hidden(&self) -> bool {
    self.atoms.contains(&self.hidden_atom)
  }

  pub fn new<T: ToString>(
    id: Window,
    e: &Ewmh,
    ignored: &[Atom],
    desktops: &[T],
  ) -> Option<Self> {
    let atoms = Self::fetch_atoms(e, id);
    if atoms.iter().filter(|a| ignored.contains(a)).count() > 0 {return None;}
    let class = get_wm_class(e, id).get_reply().unwrap();
    let instance = class.instance().to_owned();
    let class = class.class().to_owned();
    let name = get_wm_name(e, id).get_reply().unwrap().string().to_owned();
    let n = get_wm_desktop(e, id).get_reply().unwrap();
    let desktop = desktops[n as usize].to_string();
    let hidden_atom = e.WM_STATE_HIDDEN();
    let urgent_atom = e.WM_STATE_DEMANDS_ATTENTION();

    Some(Win {
      atoms,
      class,
      desktop,
      hidden_atom,
      id,
      instance,
      name,
      urgent_atom
    })
  }

  pub fn switch_hidden(&mut self, e: &Ewmh) {
    match self.hidden() {
      false => unmap_window(&e, self.id).request_check().unwrap(),
      true => map_window(&e, self.id).request_check().unwrap(),
    }
    self.update_atoms(e);
  }

  fn update_atoms(&mut self, e: &Ewmh) {
    self.atoms = Self::fetch_atoms(e, self.id)
  }

  pub fn urgent(&self) -> bool {
    self.atoms.contains(&self.urgent_atom)
  }
}
