#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_double, c_int, c_uint};
use glib_sys::{GList, GRegex, gboolean, gpointer};

extern "C" {
  pub static PropertyTypeName: [*const c_char; P_NUM_TYPES as usize];
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

pub type PropertyType = c_uint;
pub const P_INTEGER: PropertyType = 0;
pub const P_DOUBLE: PropertyType = 1;
pub const P_STRING: PropertyType = 2;
pub const P_CHAR: PropertyType = 3;
pub const P_BOOLEAN: PropertyType = 4;
pub const P_COLOR: PropertyType = 5;
pub const P_IMAGE: PropertyType = 6;
pub const P_PADDING: PropertyType = 7;
pub const P_LINK: PropertyType = 8;
pub const P_POSITION: PropertyType = 9;
pub const P_HIGHLIGHT: PropertyType = 10;
pub const P_LIST: PropertyType = 11;
pub const P_ORIENTATION: PropertyType = 12;
pub const P_CURSOR: PropertyType = 13;
pub const P_INHERIT: PropertyType = 14;
pub const P_NUM_TYPES: PropertyType = 15;

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

pub type RofiCursorType = c_uint;
pub const ROFI_CURSOR_DEFAULT: RofiCursorType = 0;
pub const ROFI_CURSOR_POINTER: RofiCursorType = 1;
pub const ROFI_CURSOR_TEXT: RofiCursorType = 2;

pub type RofiDirection = c_uint;
pub const ROFI_DIRECTION_LEFT: RofiDirection = 0;
pub const ROFI_DIRECTION_RIGHT: RofiDirection = 1;
pub const ROFI_DIRECTION_TOP: RofiDirection = 2;
pub const ROFI_DIRECTION_BOTTOM: RofiDirection = 3;
pub const ROFI_DIRECTION_ANGLE: RofiDirection = 4;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct RofiDistance {
  pub base: RofiDistanceUnit,
  pub style: RofiLineStyle,
}

pub type RofiDistanceModifier = c_uint;
pub const ROFI_DISTANCE_MODIFIER_NONE: RofiDistanceModifier = 0;
pub const ROFI_DISTANCE_MODIFIER_ADD: RofiDistanceModifier = 1;
pub const ROFI_DISTANCE_MODIFIER_SUBTRACT: RofiDistanceModifier = 2;
pub const ROFI_DISTANCE_MODIFIER_DIVIDE: RofiDistanceModifier = 3;
pub const ROFI_DISTANCE_MODIFIER_MULTIPLY: RofiDistanceModifier = 4;
pub const ROFI_DISTANCE_MODIFIER_MODULO: RofiDistanceModifier = 5;
pub const ROFI_DISTANCE_MODIFIER_GROUP: RofiDistanceModifier = 6;
pub const ROFI_DISTANCE_MODIFIER_MIN: RofiDistanceModifier = 7;
pub const ROFI_DISTANCE_MODIFIER_MAX: RofiDistanceModifier = 8;

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

pub type RofiHighlightStyle = c_uint;
pub const ROFI_HL_NONE: RofiHighlightStyle = 0;
pub const ROFI_HL_BOLD: RofiHighlightStyle = 1 << 0;
pub const ROFI_HL_UNDERLINE: RofiHighlightStyle = 1 << 1;
pub const ROFI_HL_ITALIC: RofiHighlightStyle = 1 << 2;
pub const ROFI_HL_COLOR: RofiHighlightStyle = 1 << 3;
pub const ROFI_HL_STRIKETHROUGH: RofiHighlightStyle = 1 << 4;
pub const ROFI_HL_SMALL_CAPS: RofiHighlightStyle = 1 << 5;

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

pub type RofiImageType = c_uint;
pub const ROFI_IMAGE_URL: RofiImageType = 0;
pub const ROFI_IMAGE_LINEAR_GRADIENT: RofiImageType = 1;

pub type RofiLineStyle = c_uint;
pub const ROFI_HL_SOLID: RofiLineStyle = 0;
pub const ROFI_HL_DASH: RofiLineStyle = 1;

pub type RofiOrientation = c_uint;
pub const ROFI_ORIENTATION_VERTICAL: RofiOrientation = 0;
pub const ROFI_ORIENTATION_HORIZONTAL: RofiOrientation = 1;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct RofiPadding {
  pub top: RofiDistance,
  pub right: RofiDistance,
  pub bottom: RofiDistance,
  pub left: RofiDistance,
}

pub type RofiPixelUnit = c_uint;
pub const ROFI_PU_PX: RofiPixelUnit = 0;
pub const ROFI_PU_MM: RofiPixelUnit = 1;
pub const ROFI_PU_EM: RofiPixelUnit = 2;
pub const ROFI_PU_PERCENT: RofiPixelUnit = 3;
pub const ROFI_PU_CH: RofiPixelUnit = 4;

pub type RofiScaleType = c_uint;
pub const ROFI_SCALE_NONE: RofiScaleType = 0;
pub const ROFI_SCALE_BOTH: RofiScaleType = 1;
pub const ROFI_SCALE_HEIGHT: RofiScaleType = 2;
pub const ROFI_SCALE_WIDTH: RofiScaleType = 3;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ThemeColor {
  pub red: c_double,
  pub green: c_double,
  pub blue: c_double,
  pub alpha: c_double,
}

pub type WindowLocation = c_uint;
pub const WL_CENTER: WindowLocation = 0;
pub const WL_NORTH: WindowLocation = 1 << 0;
pub const WL_EAST: WindowLocation = 1 << 1;
pub const WL_SOUTH: WindowLocation = 1 << 2;
pub const WL_WEST: WindowLocation = 1 << 3;
pub const WL_NORTH_WEST: WindowLocation = WL_NORTH | WL_WEST;
pub const WL_NORTH_EAST: WindowLocation = WL_NORTH | WL_EAST;
pub const WL_SOUTH_EAST: WindowLocation = WL_SOUTH | WL_EAST;
pub const WL_SOUTH_WEST: WindowLocation = WL_SOUTH | WL_WEST;
