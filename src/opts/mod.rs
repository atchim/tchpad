mod fields;

use std::{ffi::{CStr, CString}, os::raw::c_char, ptr::null_mut};

use rofi_sys::helper::find_arg_str;

use crate::opts::fields::Fields;

pub struct Opts {
  cmd: String,
  fields: Fields,
  hidden: String,
  sh: String,
  win: String,
}

impl Opts {
  pub fn cmd(&self) -> &str {
    &self.cmd
  }

  pub fn fields(&self) -> &Fields {
    &self.fields
  }

  pub fn hidden(&self) -> &str {
    &self.hidden
  }

  pub fn sh(&self) -> &str {
    &self.sh
  }

  pub fn win(&self) -> &str {
    &self.win
  }
}

impl Default for Opts {
  fn default() -> Self {
    let cmd = fetch_arg("-tchpad-cmd").unwrap_or_default();
    let fields = Fields::default();
    let hidden = fetch_arg("-tchpad-hidden")
      .unwrap_or_else(|| "{d:8}  {c:8} *{n}".into());
    let sh = fetch_arg("-tchpad-sh").unwrap_or_else(|| "sh".into());
    let win = fetch_arg("-tchpad-win")
      .unwrap_or_else(|| "{d:8}  {c:8}  {n}".into());
    Opts {cmd, fields, hidden, sh, win}
  }
}

fn fetch_arg(name: &str) -> Option<String> {
  let arg = CString::new(name.as_bytes()).unwrap();
  let mut val = null_mut();
  unsafe {find_arg_str(arg.as_ptr() as *const c_char, &mut val);}

  match val.is_null() {
    false => {
      let c_str = unsafe {CStr::from_ptr(val as *const c_char)};
      Some(c_str.to_string_lossy().to_string())
    }
    true => None,
  }
}
