mod tchpad;
mod win;

use std::{
  ffi::{CStr, CString, c_void},
  os::raw::{c_char, c_int, c_uint},
  ptr::null_mut,
};

use glib_sys::GList;

use rofi_sys::{
  helper::find_arg_str,
  mode::{mode_get_private_data, mode_set_private_data},
  mode_private::{ABI_VERSION, Mode},
};

use crate::tchpad::Tchpad;

#[export_name = "mode"]
#[no_mangle]
static mut MODE: Mode = Mode {
  abi_version: ABI_VERSION,
  name: null_mut(),
  cfg_name_key: [0; 128],
  display_name: null_mut(),
  _init: Some(tchpad_init),
  _destroy: Some(tchpad_destroy),
  _get_num_entries: Some(tchpad_get_num_entries),
  _result: None,
  _token_match: None,
  _get_display_value: Some(tchpad_get_display_value),
  _get_icon: None,
  _get_completion: None,
  _preprocess_input: None,
  _get_message: None,
  private_data: null_mut(),
  free: None,
  ed: null_mut(),
  module: null_mut(),
};

#[link_section = ".init_array"]
#[no_mangle]
static TCHPAD_INIT_MODE: extern "C" fn() = tchpad_init_mode;

extern "C" fn tchpad_destroy(mode: *mut Mode) {
  let t = unsafe {mode_get_private_data(mode)};
  assert!(!t.is_null());
  unsafe {Box::from_raw(t as *mut Tchpad);}
}

extern "C" fn tchpad_get_display_value(
  mode: *const Mode,
  selected_line: c_uint,
  _state: *mut c_int,
  _attribute_list: *mut *mut GList,
  get_entry: c_int,
) -> *mut c_char {
  let t = unsafe {mode_get_private_data(mode) as *mut Tchpad};
  assert!(!t.is_null());
  let win = unsafe {&(*t).wins()[selected_line as usize]};

  match get_entry != 0 {
    false => null_mut(),
    true => {
      let s = unsafe {win.display_value((*t).win_fmt(), (*t).hidden_fmt())};
      CString::new(s.as_bytes()).unwrap().into_raw()
    }
  }
}

extern "C" fn tchpad_get_num_entries(mode: *const Mode) -> u32 {
  let t = unsafe {mode_get_private_data(mode) as *mut Tchpad};
  assert!(!t.is_null());
  unsafe {(*t).wins().len() as u32}
}

extern "C" fn tchpad_init(mode: *mut Mode) -> c_int {
  let t = unsafe {mode_get_private_data(mode)};
  assert!(t.is_null());

  let win_arg = CString::new("-tchpad-win").unwrap();
  let mut win_fmt = null_mut();
  unsafe {find_arg_str(win_arg.as_ptr() as *const c_char, &mut win_fmt);}

  let win_fmt = match win_fmt.is_null() {
    false => unsafe {CStr::from_ptr(win_fmt).to_string_lossy()},
    true => "{d:8}  {c:8}  {n}".into(),
  };

  let hidden_arg = CString::new("-tchpad-hidden").unwrap();
  let mut hidden_fmt = null_mut();
  unsafe {find_arg_str(hidden_arg.as_ptr() as *const c_char, &mut hidden_fmt);}

  let hidden_fmt = match hidden_fmt.is_null() {
    false => unsafe {CStr::from_ptr(hidden_fmt).to_string_lossy()},
    true => "{d:8}  {c:8} *{n}".into(),
  };

  let t = Box::new(Tchpad::new(&win_fmt, &hidden_fmt));
  unsafe {mode_set_private_data(mode, Box::into_raw(t) as *mut c_void);}
  return 1;
}

extern "C" fn tchpad_init_mode() {
  unsafe {MODE.name = CString::new("tchpad").unwrap().into_raw();}
}
