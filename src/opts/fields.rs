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

    s.chars().for_each(|c| {
      match c {
        'c' => f.class = true,
        'd' => f.desktop = true,
        'i' => f.instance = true,
        'n' => f.name = true,
        c => g_warning!(G_LOG_DOMAIN, "Invalid field flag: '{}'", c),
      }
    });

    f
  }
}
