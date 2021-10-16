mod tchpad;

use std::{
  ffi::{CString, c_void},
  os::raw::c_int,
  ptr::null_mut
};

use rofi_sys::{
  ABI_VERSION,
  Mode,
  mode_get_private_data,
  mode_set_private_data,
};

use crate::tchpad::Tchpad;

#[export_name = "mode"]
#[no_mangle]
static mut MODE: Mode = Mode {
  abi_version: ABI_VERSION,
  name: null_mut(),
  cfg_name_key: [0; 128],
  display_name: null_mut(),
  _init: None,
  _destroy: None,
  _get_num_entries: None,
  _result: None,
  _token_match: None,
  _get_display_value: None,
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
  let ptr = unsafe {mode_get_private_data(mode)};
  assert!(!ptr.is_null());
  unsafe {Box::from_raw(ptr as *mut Tchpad);}
}

extern "C" fn get_num_entries(mode: *const Mode) -> u32 {
  let ptr = unsafe {mode_get_private_data(mode)};
  assert!(!ptr.is_null());
  let t = unsafe {Box::from_raw(ptr as *mut Tchpad)};
  t.get_wins().len() as u32
}

extern "C" fn mode_init() {
  unsafe {
    MODE._destroy = Some(destroy);
    MODE._get_num_entries = Some(get_num_entries);
    MODE._init = Some(init);
    MODE.name = CString::new("tchpad").unwrap().into_raw();
  }
}

extern "C" fn init(mode: *mut Mode) -> c_int {
  let ptr = unsafe {mode_get_private_data(mode)};
  assert!(ptr.is_null());
  let t = Box::new(Tchpad::new());
  unsafe {mode_set_private_data(mode, Box::into_raw(t) as *mut c_void);}
  return 1;
}
