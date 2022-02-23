mod opts;
mod tchpad;
mod win;

use cairo::ffi::cairo_surface_t;
use glib_sys::GList;
use rofi_sys::{
  helper::helper_token_match,
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
  types::rofi_int_matcher,
};
use self::tchpad::Tchpad;
use std::{
  ffi::{CString, c_void},
  os::raw::{c_char, c_int, c_uint},
  process::Command,
  ptr::null_mut,
};

/// Domain name for the Tchpad plugin used by GLib logging functions.
static G_LOG_DOMAIN: &'_ str = "Plugins.Tchpad";

#[export_name = "mode"]
#[no_mangle]
static mut MODE: Mode = Mode {
  _destroy: Some(tchpad_destroy),
  _get_completion: None,
  _get_display_value: Some(tchpad_get_display_value),
  _get_icon: Some(tchpad_get_icon),
  _get_message: None,
  _get_num_entries: Some(tchpad_get_num_entries),
  _init: Some(tchpad_init),
  _preprocess_input: None,
  _result: Some(tchpad_result),
  _token_match: Some(tchpad_token_match),
  abi_version: ABI_VERSION,
  cfg_name_key: [0; 128],
  display_name: null_mut(),
  ed: null_mut(),
  free: None,
  module: null_mut(),
  name: null_mut(),
  private_data: null_mut(),
};

#[link_section = ".init_array"]
#[no_mangle]
static TCHPAD_BOOTSTRAP: unsafe extern "C" fn() = tchpad_bootstrap;

unsafe extern "C" fn tchpad_bootstrap() {
  MODE.name = CString::new("tchpad").unwrap().into_raw();
}

unsafe extern "C" fn tchpad_destroy(mode: *mut Mode) {
  let t = mode_get_private_data(mode);
  Box::from_raw(t as *mut Tchpad);
}

unsafe extern "C" fn tchpad_get_display_value(
  mode: *const Mode,
  selected_line: c_uint,
  _state: *mut c_int,
  _attribute_list: *mut *mut GList,
  get_entry: c_int,
) -> *mut c_char {
  match get_entry != 0 {
    false => null_mut(),
    true => {
      let t = mode_get_private_data(mode) as *mut Tchpad;
      let win = &(*t).wins[selected_line as usize];
      let s = win.display_value(&(*t).opts.win, &(*t).opts.hidden);
      CString::new(s.as_bytes()).unwrap().into_raw()
    }
  }
}

unsafe extern "C" fn tchpad_get_icon(
  mode: *const Mode,
  selected_line: c_uint,
  height: c_int,
) -> *mut cairo_surface_t {
  let t = mode_get_private_data(mode) as *mut Tchpad;
  let win = &mut (*t).wins[selected_line as usize];
  win.fetch_icon(height)
}

unsafe extern "C" fn tchpad_get_num_entries(mode: *const Mode) -> u32 {
  let t = mode_get_private_data(mode) as *mut Tchpad;
  (*t).wins.len() as u32
}

unsafe extern "C" fn tchpad_init(mode: *mut Mode) -> c_int {
  let t = Box::new(Tchpad::default());
  mode_set_private_data(mode, Box::into_raw(t) as *mut c_void);
  1
}

unsafe extern "C" fn tchpad_result(
  mode: *mut Mode,
  menu_retv: c_int,
  _input: *mut *mut c_char,
  selected_line: c_uint,
) -> ModeMode {
  if (menu_retv as u32 & MENU_CANCEL) == 0 {
    let t = mode_get_private_data(mode) as *mut Tchpad;
    let win = &(*t).wins[selected_line as usize];

    match (menu_retv as u32 & MENU_OK) > 0 {
      false => win.close(&(*t).e, (*t).screen),
      true => match (menu_retv as u32 & MENU_CUSTOM_ACTION) > 0 {
        false => win.focus(&(*t).e, (*t).screen),
        true => {
          let opts = &(*t).opts;
          match opts.cmd.is_empty() {
            false => {
              let _ = Command::new(&opts.sh)
                .arg("-c")
                .arg(&opts.cmd)
                .env("TCHPAD_WIN", win.id.to_string())
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

// NOTE: This is almost the same code as Rofi's window dialog, but in Rust.
unsafe extern "C" fn tchpad_token_match(
  mode: *const Mode,
  tokens: *mut *mut rofi_int_matcher,
  index: c_uint,
) -> c_int {
  let mut matched = true;

  if !tokens.is_null() {
    let t = mode_get_private_data(mode) as *mut Tchpad;
    let win = &(*t).wins[index as usize];
    let mut i = 0;

    loop {
      // Check if this loop must break...
      if !matched {break;}
      if tokens.is_null() {break;}
      let token = *(tokens).offset(i);
      if token.is_null() {break;}

      // Same hack as the one from Rofi.
      let ftokens: [*mut rofi_int_matcher; 2] = [token, null_mut()];
      let mut test = 0;

      if (*t).opts.fields.class {
        let class = CString::new(win.class.as_bytes()).unwrap();
        test = helper_token_match(ftokens.as_ptr(), class.as_ptr());
      }

      if test == (*token).invert && (*t).opts.fields.desktop {
        let desktop = CString::new(win.desktop.as_bytes()).unwrap();
        test = helper_token_match(ftokens.as_ptr(), desktop.as_ptr());
      }

      if test == (*token).invert && (*t).opts.fields.instance {
        let instance = CString::new(win.instance.as_bytes()).unwrap();
        test = helper_token_match(ftokens.as_ptr(), instance.as_ptr());
      }

      if test == (*token).invert && (*t).opts.fields.name {
        let name = CString::new(win.name.as_bytes()).unwrap();
        test = helper_token_match(ftokens.as_ptr(), name.as_ptr());
      }

      matched = test != 0;
      i += 1;
    }
  }

  matched.into()
}
