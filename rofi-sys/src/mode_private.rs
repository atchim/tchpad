use std::os::raw::{c_char, c_int, c_uint, c_void};

use cairo_sys::cairo_surface_t;
use glib_sys::GList;

use crate::{mode::ModeMode, types::rofi_int_matcher};

pub const ABI_VERSION: c_uint = 6;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct GModule {
  _unused: [u8; 0],
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Mode {
  pub abi_version: c_uint,
  pub name: *mut c_char,
  pub cfg_name_key: [c_char; 128],
  pub display_name: *mut c_char,
  pub _init: Option<unsafe extern "C" fn(sw: *mut Mode) -> c_int>,
  pub _destroy: Option<unsafe extern "C" fn(sw: *mut Mode)>,
  pub _get_num_entries:
    Option<unsafe extern "C" fn(sw: *const Mode) -> c_uint>,
  pub _result: Option<
    unsafe extern "C" fn(
      sw: *mut Mode,
      menu_retv: c_int,
      input: *mut *mut c_char,
      selected_line: c_uint,
    ) -> ModeMode
  >,
  pub _token_match: Option<
    unsafe extern "C" fn(
      data: *const Mode,
      tokens: *mut *mut rofi_int_matcher,
      index: c_uint,
    ) -> c_int
  >,
  pub _get_display_value: Option<
    unsafe extern "C" fn(
      sw: *const Mode,
      selected_line: c_uint,
      state: *mut c_int,
      attribute_list: *mut *mut GList,
      get_entry: c_int,
    ) -> *mut c_char
  >,
  pub _get_icon: Option<
    unsafe extern "C" fn(
      sw: *const Mode,
      selected_line: c_uint,
      height: c_int,
    ) -> *mut cairo_surface_t
  >,
  pub _get_completion: Option<
    unsafe extern "C" fn(sw: *const Mode, selected_line: c_uint) -> *mut c_char
  >,
  pub _preprocess_input: Option<
    unsafe extern "C" fn(sw: *mut Mode, input: *const c_char) -> *mut c_char
  >,
  pub _get_message:
    Option<unsafe extern "C" fn(sw: *const Mode) -> *mut c_char>,
  pub private_data: *mut c_void,
  pub free: Option<unsafe extern "C" fn(data: *mut Mode)>,
  pub ed: *mut c_void,
  pub module: *mut GModule,
}
