use std::os::raw::c_int;

pub type TextBoxFontType = c_int;
pub const STATE_MASK: TextBoxFontType = !(ALT | HIGHLIGHT | MARKUP | SELECTED);
pub const NORMAL: TextBoxFontType = 0;
pub const URGENT: TextBoxFontType = 1 << 0;
pub const ACTIVE: TextBoxFontType = 1 << 1;
pub const SELECTED: TextBoxFontType = 1 << 2;
pub const MARKUP: TextBoxFontType = 1 << 3;
pub const ALT: TextBoxFontType = 1 << 4;
pub const HIGHLIGHT: TextBoxFontType = 1 << 5;
pub const FMOD_MASK: TextBoxFontType = ALT | HIGHLIGHT;
