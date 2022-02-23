mod fields;

use rofi_sys::helper::find_arg_str;
use self::fields::Fields;
use std::{ffi::{CStr, CString}, os::raw::c_char, ptr::null_mut};

pub struct Opts {
  pub cmd: String,
  pub fields: Fields,
  pub hidden: String,
  pub sh: String,
  pub win: String,
}

impl Opts {
  pub fn from_args() -> Self {
    let cmd = fetch_arg("-tchpad-cmd").unwrap_or_default();
    let fields = match fetch_arg("-tchpad-fields") {
      None => Fields::default(),
      Some(s) => Fields::from(s.as_str()),
    };
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
