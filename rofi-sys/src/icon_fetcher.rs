use std::os::raw::{c_char, c_int};

use cairo_sys::cairo_surface_t;
use glib_sys::gboolean;

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
