use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong};

use cairo_sys::cairo_surface_t;
use glib_sys::gboolean;

use crate::types::{rofi_int_matcher, rofi_range_pair};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct RofiHelperExecuteContext {
  pub name: *const c_char,
  pub binary: *const c_char,
  pub description: *const c_char,
  pub icon: *const c_char,
  pub app_id: *const c_char,
  pub wmclass: *const c_char,
  pub command: *const c_char,
}

extern "C" {
  pub fn cairo_image_surface_create_from_svg(
    file: *const c_char,
    height: c_int,
  ) -> *mut cairo_surface_t;

  pub fn cmd_set_arguments(argc: c_int, argv: *mut *mut c_char);
  pub fn config_sanity_check() -> c_int;
  pub fn create_pid_file(pidfile: *const c_char) -> c_int;
  pub fn execute_generator(cmd: *const c_char) -> c_int;
  pub fn find_arg(key: *const c_char) -> c_int;
  pub fn find_arg_char(key: *const c_char, val: *mut c_char) -> c_int;
  pub fn find_arg_int(key: *const c_char, val: *mut c_int) -> c_int;
  pub fn find_arg_str(key: *const c_char, val: *mut *mut c_char) -> c_int;
  pub fn find_arg_strv(key: *const c_char) -> *mut *const c_char;
  pub fn find_arg_uint(key: *const c_char, val: *mut c_uint) -> c_int;

  pub fn helper_execute(
    wd: *const c_char,
    args: *mut *mut c_char,
    error_precmd: *const c_char,
    error_cmd: *const c_char,
    context: *mut RofiHelperExecuteContext,
  ) -> gboolean;

  pub fn helper_execute_command(
    wd: *const c_char,
    cmd: *const c_char,
    run_in_term: gboolean,
    context: *mut RofiHelperExecuteContext,
  ) -> gboolean;

  pub fn helper_get_theme_path(
    file: *const c_char,
    ext: *const c_char,
  ) -> *mut c_char;

  pub fn helper_parse_char(arg: *const c_char) -> c_char;

  pub fn helper_parse_setup(
    string: *mut c_char,
    output: *mut *mut *mut c_char,
    length: *mut c_int,
    ...,
  ) -> c_int;

  pub fn helper_string_replace_if_exists(
    string: *mut c_char,
    ...,
  ) -> *mut c_char;

  pub fn helper_token_match(
    tokens: *const *mut rofi_int_matcher,
    input: *const c_char,
  ) -> c_int;

  pub fn helper_tokenize(
    input: *const c_char,
    case_sensitive: c_int,
  ) -> *mut *mut rofi_int_matcher;

  pub fn helper_tokenize_free(tokens: *mut *mut rofi_int_matcher);

  pub fn levenshtein(
    needle: *const c_char,
    needlelen: c_long,
    haystack: *const c_char,
    haystacklen: c_long,
  ) -> c_uint;

  pub fn parse_ranges(
    input: *mut c_char,
    list: *mut *mut rofi_range_pair,
    length: *mut c_uint,
  );

  pub fn remove_pid_file(fd: c_int);
  pub fn rofi_escape_markup(text: *mut c_char) -> *mut c_char;
  pub fn rofi_expand_path(input: *const c_char) -> *mut c_char;
  pub fn rofi_force_utf8(data: *const c_char, length: c_long) -> *mut c_char;

  pub fn rofi_latin_to_utf8_strdup(
    input: *const c_char,
    length: c_long,
  ) -> *mut c_char;

  pub fn rofi_output_formatted_line(
    format: *const c_char,
    string: *const c_char,
    selected_line: c_int,
    filter: *const c_char,
  );

  pub fn rofi_scorer_fuzzy_evaluate(
    pattern: *const c_char,
    plen: c_long,
    str_: *const c_char,
    slen: c_long,
  ) -> c_int;

  pub fn utf8_strncmp(a: *const c_char, b: *const c_char, n: c_ulong) -> c_int;
}
