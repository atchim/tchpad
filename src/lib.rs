mod opts;
mod tchpad;
mod win;

use std::{
  ffi::{CString, c_void},
  os::raw::{c_char, c_int, c_uint},
  process::Command,
  ptr::null_mut,
};

use glib_sys::GList;

use rofi_sys::{
  mode::{
    MENU_CANCEL,
    MENU_CUSTOM_ACTION,
    MENU_OK,
    MODE_EXIT,
    ModeMode,
    mode_get_private_data,
    mode_set_private_data,
  },
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
  _result: Some(tchpad_result),
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
      let s = unsafe {
        win.display_value((*t).opts().win(), (*t).opts().hidden())
      };
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
  let t = Box::new(Tchpad::default());
  unsafe {mode_set_private_data(mode, Box::into_raw(t) as *mut c_void);}
  1
}

extern "C" fn tchpad_init_mode() {
  unsafe {MODE.name = CString::new("tchpad").unwrap().into_raw();}
}

extern "C" fn tchpad_result(
  mode: *mut Mode,
  menu_retv: c_int,
  _input: *mut *mut c_char,
  selected_line: c_uint,
) -> ModeMode {
  if (menu_retv as u32 & MENU_CANCEL) == 0 {
    let t = unsafe {mode_get_private_data(mode) as *mut Tchpad};
    assert!(!t.is_null());
    let win = unsafe {&(*t).wins()[selected_line as usize]};
    match (menu_retv as u32 & MENU_OK) > 0 {
      false => unsafe {win.close((*t).e(), (*t).screen())},
      true => match (menu_retv as u32 & MENU_CUSTOM_ACTION) > 0 {
        false => unsafe {win.focus((*t).e(), (*t).screen())},
        true => {
          let opts = unsafe {(*t).opts()};
          match opts.cmd().is_empty() {
            false => {
              let _ = Command::new(opts.sh())
                .arg("-c")
                .arg(opts.cmd())
                .env("TCHPAD_WIN", win.id().to_string())
                .spawn();
            }
            true => (),
          }
        }
      }
    }
  }
  MODE_EXIT
}
