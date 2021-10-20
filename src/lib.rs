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
  mode::{MODE_EXIT, ModeMode, mode_get_private_data, mode_set_private_data},
  mode_private::{ABI_VERSION, Mode},
  text_box::URGENT,
};

use crate::tchpad::Tchpad;

#[export_name = "mode"]
#[no_mangle]
static mut MODE: Mode = Mode {
  abi_version: ABI_VERSION,
  name: null_mut(),
  cfg_name_key: [0; 128],
  display_name: null_mut(),
  _init: Some(init),
  _destroy: Some(destroy),
  _get_num_entries: Some(get_num_entries),
  _result: Some(result),
  _token_match: None,
  _get_display_value: Some(get_display_value),
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
static MODE_INIT: extern "C" fn() = mode_init;

extern "C" fn destroy(mode: *mut Mode) {
  let t = unsafe {mode_get_private_data(mode)};
  assert!(!t.is_null());
  unsafe {Box::from_raw(t as *mut Tchpad);}
}

extern "C" fn get_display_value(
  mode: *const Mode,
  selected_line: c_uint,
  state: *mut c_int,
  _attribute_list: *mut *mut GList,
  get_entry: c_int,
) -> *mut c_char {
  let t = unsafe {mode_get_private_data(mode) as *mut Tchpad};
  assert!(!t.is_null());

  let win = unsafe {&(*t).wins()[selected_line as usize]};
  unsafe {win.urgent().then(|| *state |= URGENT);}

  match get_entry != 0 {
    false => null_mut(),
    true => {
      let s = unsafe {win.display_str((*t).win_fmt())};
      CString::new(s.as_bytes()).unwrap().into_raw()
    }
  }
}

extern "C" fn get_num_entries(mode: *const Mode) -> u32 {
  let t = unsafe {mode_get_private_data(mode) as *mut Tchpad};
  assert!(!t.is_null());
  unsafe {(*t).wins().len() as u32}
}

extern "C" fn init(mode: *mut Mode) -> c_int {
  let t = unsafe {mode_get_private_data(mode)};
  assert!(t.is_null());

  let arg = CString::new("-window-format").unwrap();
  let mut val = null_mut();
  unsafe {find_arg_str(arg.as_ptr() as *const c_char, &mut val);}

  let val = match val.is_null() {
    false => unsafe {CStr::from_ptr(val).to_string_lossy()},
    true => "{w}    {c}   {t}".into(),
  };

  let t = Box::new(Tchpad::new(&val));
  unsafe {mode_set_private_data(mode, Box::into_raw(t) as *mut c_void);}
  return 1;
}

extern "C" fn mode_init() {
  unsafe {MODE.name = CString::new("tchpad").unwrap().into_raw();}
}

extern "C" fn result(
  mode: *mut Mode,
  _menu_retv: c_int,
  _input: *mut *mut c_char,
  selected_line: c_uint,
) -> ModeMode {
  let t = unsafe {mode_get_private_data(mode) as *mut Tchpad};
  assert!(!t.is_null());
  unsafe {(*t).switch_hidden(selected_line as usize);}
  MODE_EXIT
}
