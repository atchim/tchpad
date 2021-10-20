use std::os::raw::{c_char, c_int, c_uint, c_void};

use cairo_sys::cairo_surface_t;
use glib_sys::GList;

use crate::{mode_private::Mode, types::rofi_int_matcher};

pub type ModeMode = c_uint;
pub const MODE_EXIT: ModeMode = 1000;
pub const NEXT_DIALOG: ModeMode = 1001;
pub const RELOAD_DIALOG: ModeMode = 1002;
pub const PREVIOUS_DIALOG: ModeMode = 1003;
pub const RESET_DIALOG: ModeMode = 1004;

pub type MenuReturn = c_uint;
pub const MENU_LOWER_MASK: MenuReturn = (1 << 16) - 1;
pub const MENU_OK: MenuReturn = 1 << 16;
pub const MENU_CANCEL: MenuReturn = 1 << 17;
pub const MENU_NEXT: MenuReturn = 1 << 18;
pub const MENU_CUSTOM_INPUT: MenuReturn = 1 << 19;
pub const MENU_ENTRY_DELETE: MenuReturn = 1 << 20;
pub const MENU_QUICK_SWITCH: MenuReturn = 1 << 21;
pub const MENU_PREVIOUS: MenuReturn = 1 << 22;
pub const MENU_CUSTOM_COMMAND: MenuReturn = 1 << 23;
pub const MENU_COMPLETE: MenuReturn = 1 << 24;
pub const MENU_CUSTOM_ACTION: MenuReturn = 1 << 28;

extern "C" {
  pub fn mode_destroy(mode: *mut Mode);
  pub fn mode_free(mode: *mut *mut Mode);

  pub fn mode_get_completion(
    mode: *const Mode,
    selected_line: c_uint,
  ) -> *mut c_char;

  pub fn mode_get_display_name(mode: *const Mode) -> *const c_char;

  pub fn mode_get_display_value(
    mode: *const Mode,
    selected_line: c_uint,
    state: *mut c_int,
    attribute_list: *mut *mut GList,
    get_entry: c_int,
  ) -> *mut c_char;

  pub fn mode_get_icon(
    mode: *const Mode,
    selected_line: c_uint,
    height: c_int,
  ) -> *mut cairo_surface_t;

  pub fn mode_get_message(mode: *const Mode) -> *mut c_char;
  pub fn mode_get_name(mode: *const Mode) -> *const c_char;
  pub fn mode_get_num_entries(mode: *const Mode) -> c_uint;
  pub fn mode_get_private_data(mode: *const Mode) -> *mut c_void;
  pub fn mode_init(mode: *mut Mode) -> c_int;

  pub fn mode_preprocess_input(
    mode: *mut Mode,
    input: *const c_char,
  ) -> *mut c_char;

  pub fn mode_result(
    mode: *mut Mode,
    menu_retv: c_int,
    input: *mut *mut c_char,
    selected_line: c_uint,
  ) -> ModeMode;

  pub fn mode_set_config(mode: *mut Mode);
  pub fn mode_set_private_data(mode: *mut Mode, pd: *mut c_void);

  pub fn mode_token_match(
    mode: *const Mode,
    tokens: *mut *mut rofi_int_matcher,
    selected_line: c_uint,
  ) -> c_int;
}
