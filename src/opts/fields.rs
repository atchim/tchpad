use crate::G_LOG_DOMAIN;
use glib::g_warning;

pub struct Fields {
  pub class: bool,
  pub desktop: bool,
  pub instance: bool,
  pub name: bool,
}

impl Default for Fields {
  fn default() -> Self {
    Fields {
      class: true,
      desktop: true,
      instance: true,
      name: true,
    }
  }
}

impl From<&str> for Fields {
  fn from(s: &str) -> Self {
    let mut f = Fields {
      class: false,
      desktop: false,
      instance: false,
      name: false,
    };

    s.split_terminator(',').for_each(|field| {
      match field {
        "class" => f.class = true,
        "desktop" => f.desktop = true,
        "instance" => f.instance = true,
        "name" => f.name = true,
        s => g_warning!(G_LOG_DOMAIN, "Invalid field: '{}'", s),
      }
    });

    f
  }
}
