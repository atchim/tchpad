#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

use cairo_sys::cairo_surface_t;
use glib_sys::{GList, GRegex, gboolean, gpointer};

//
// helper.h
//

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

//
// mode.h
//

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum ModeMode {
  MODE_EXIT = 1000,
  NEXT_DIALOG = 1001,
  RELOAD_DIALOG = 1002,
  PREVIOUS_DIALOG = 1003,
  RESET_DIALOG = 1004,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum MenuReturn {
  MENU_OK = 0x00010000,
  MENU_CANCEL = 0x00020000,
  MENU_NEXT = 0x00040000,
  MENU_CUSTOM_INPUT = 0x00080000,
  MENU_ENTRY_DELETE = 0x00100000,
  MENU_QUICK_SWITCH = 0x00200000,
  MENU_CUSTOM_COMMAND = 0x00800000,
  MENU_PREVIOUS = 0x00400000,
  MENU_COMPLETE = 0x01000000,
  MENU_CUSTOM_ACTION = 0x10000000,
  MENU_LOWER_MASK = 0x0000FFFF
}

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

//
// mode-private.h
//

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
  pub _init: Option<extern "C" fn(sw: *mut Mode) -> c_int>,
  pub _destroy: Option<extern "C" fn(sw: *mut Mode)>,
  pub _get_num_entries: Option<extern "C" fn(sw: *const Mode) -> c_uint>,

  pub _result: Option<
    extern "C" fn(
      sw: *mut Mode,
      menu_retv: c_int,
      input: *mut *mut c_char,
      selected_line: c_uint,
    ) -> ModeMode
  >,

  pub _token_match: Option<
    extern "C" fn(
      data: *const Mode,
      tokens: *mut *mut rofi_int_matcher,
      index: c_uint,
    ) -> c_int
  >,

  pub _get_display_value: Option<
    extern "C" fn(
      sw: *const Mode,
      selected_line: c_uint,
      state: *mut c_int,
      attribute_list: *mut *mut GList,
      get_entry: c_int,
    ) -> *mut c_char
  >,

  pub _get_icon: Option<
    extern "C" fn(
      sw: *const Mode,
      selected_line: c_uint,
      height: c_int,
    ) -> *mut cairo_surface_t
  >,

  pub _get_completion: Option<
    extern "C" fn(sw: *const Mode, selected_line: c_uint) -> *mut c_char
  >,

  pub _preprocess_input: Option<
    extern "C" fn(sw: *mut Mode, input: *const c_char) -> *mut c_char
  >,

  pub _get_message: Option<extern "C" fn(sw: *const Mode) -> *mut c_char>,
  pub private_data: *mut c_void,
  pub free: Option<extern "C" fn(data: *mut Mode)>,
  pub ed: *mut c_void,
  pub module: *mut GModule,
}

//
// rofi-icon-fetcher.h
//

extern "C" {
  pub fn rofi_icon_fetcher_destroy();
  pub fn rofi_icon_fetcher_file_is_image(path: *const c_char) -> gboolean;
  pub fn rofi_icon_fetcher_get(uid: u32) -> *mut cairo_surface_t;
  pub fn rofi_icon_fetcher_init();
  pub fn rofi_icon_fetcher_query(name: *const c_char, size: c_int) -> u32;

  pub fn rofi_icon_fetcher_query_advanced(
    name: *const c_char,
    wsize: c_int,
    hsize: c_int,
  ) -> u32;
}

//
// rofi-types.h
//

extern "C" {
  pub static PropertyTypeName:
    [*const c_char; PropertyType::P_NUM_TYPES as usize];
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct _thread_state {
  pub callback: Option<extern "C" fn(t: *mut _thread_state, data: gpointer)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Property {
  pub name: *mut c_char,
  pub type_: PropertyType,
  pub value: PropertyValue,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum PropertyType {
  P_INTEGER,
  P_DOUBLE,
  P_STRING,
  P_CHAR,
  P_BOOLEAN,
  P_COLOR,
  P_IMAGE,
  P_PADDING,
  P_LINK,
  P_POSITION,
  P_HIGHLIGHT,
  P_LIST,
  P_ORIENTATION,
  P_CURSOR,
  P_INHERIT,
  P_NUM_TYPES,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union PropertyValue {
  pub i: c_int,
  pub f: c_double,
  pub s: *mut c_char,
  pub c: c_char,
  pub b: gboolean,
  pub color: ThemeColor,
  pub padding: RofiPadding,
  pub link: PropertyValueLink,
  pub highlight: RofiHighlightColorStyle,
  pub image: RofiImage,
  pub list: *mut GList,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PropertyValueLink {
  pub name: *mut c_char,
  pub ref_: *mut Property,
  pub def_value: *mut Property,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct rofi_int_matcher {
  pub regex: *mut GRegex,
  pub invert: gboolean,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct rofi_range_pair {
  pub start: c_int,
  pub stop: c_int,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum RofiCursorType {
  ROFI_CURSOR_DEFAULT,
  ROFI_CURSOR_POINTER,
  ROFI_CURSOR_TEXT,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum RofiDirection {
  ROFI_DIRECTION_LEFT,
  ROFI_DIRECTION_RIGHT,
  ROFI_DIRECTION_TOP,
  ROFI_DIRECTION_BOTTOM,
  ROFI_DIRECTION_ANGLE,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct RofiDistance {
  pub base: RofiDistanceUnit,
  pub style: RofiLineStyle,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum RofiDistanceModifier {
  ROFI_DISTANCE_MODIFIER_NONE,
  ROFI_DISTANCE_MODIFIER_ADD,
  ROFI_DISTANCE_MODIFIER_SUBTRACT,
  ROFI_DISTANCE_MODIFIER_DIVIDE,
  ROFI_DISTANCE_MODIFIER_MULTIPLY,
  ROFI_DISTANCE_MODIFIER_MODULO,
  ROFI_DISTANCE_MODIFIER_GROUP,
  ROFI_DISTANCE_MODIFIER_MIN,
  ROFI_DISTANCE_MODIFIER_MAX,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct RofiDistanceUnit {
  pub distance: c_double,
  pub type_: RofiPixelUnit,
  pub modtype: RofiDistanceModifier,
  pub left: *mut RofiDistanceUnit,
  pub right: *mut RofiDistanceUnit,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct RofiHighlightColorStyle {
  pub style: RofiHighlightStyle,
  pub color: ThemeColor,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum RofiHighlightStyle {
  ROFI_HL_NONE = 0,
  ROFI_HL_BOLD = 1,
  ROFI_HL_UNDERLINE = 2,
  ROFI_HL_ITALIC = 4,
  ROFI_HL_COLOR = 8,
  ROFI_HL_STRIKETHROUGH = 16,
  ROFI_HL_SMALL_CAPS = 32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct RofiImage {
  pub type_: RofiImageType,
  pub url: *mut c_char,
  pub scaling: RofiScaleType,
  pub wsize: c_int,
  pub hsize: c_int,
  pub dir: RofiDirection,
  pub angle: c_double,
  pub colors: *mut GList,
  pub surface_id: c_uint,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum RofiImageType {
  ROFI_IMAGE_URL,
  ROFI_IMAGE_LINEAR_GRADIENT,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum RofiLineStyle {
  ROFI_HL_SOLID,
  ROFI_HL_DASH,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum RofiOrientation {
  ROFI_ORIENTATION_VERTICAL,
  ROFI_ORIENTATION_HORIZONTAL,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct RofiPadding {
  pub top: RofiDistance,
  pub right: RofiDistance,
  pub bottom: RofiDistance,
  pub left: RofiDistance,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum RofiPixelUnit {
  ROFI_PU_PX,
  ROFI_PU_MM,
  ROFI_PU_EM,
  ROFI_PU_PERCENT,
  ROFI_PU_CH,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum RofiScaleType {
  ROFI_SCALE_NONE,
  ROFI_SCALE_BOTH,
  ROFI_SCALE_HEIGHT,
  ROFI_SCALE_WIDTH,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ThemeColor {
  pub red: c_double,
  pub green: c_double,
  pub blue: c_double,
  pub alpha: c_double,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum WindowLocation {
  WL_CENTER = 0b0000,
  WL_NORTH = 0b0001,
  WL_EAST = 0b0010,
  WL_SOUTH = 0b0100,
  WL_WEST = 0b1000,
  WL_NORTH_WEST = 0b1001,
  WL_NORTH_EAST = 0b0011,
  WL_SOUTH_EAST = 0b0110,
  WL_SOUTH_WEST = 0b1100,
}
