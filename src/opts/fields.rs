use crate::opts::fetch_arg;

pub struct Fields {
  class: bool,
  desktop: bool,
  instance: bool,
  name: bool,
}

impl Fields {
  pub fn class(&self) -> bool {
    self.class
  }

  pub fn desktop(&self) -> bool {
    self.desktop
  }

  pub fn instance(&self) -> bool {
    self.instance
  }

  pub fn name(&self) -> bool {
    self.name
  }
}

impl Default for Fields {
  fn default() -> Self {
    match fetch_arg("-tchpad-fields") {
      None => Fields {
        class: true,
        desktop: true,
        instance: true,
        name: true,
      },
      Some(s) => {
        let mut class = false;
        let mut desktop = false;
        let mut instance = false;
        let mut name = false;

        s.split_terminator(',').for_each(|field| {
          match field {
            "class" => class = true,
            "desktop" => desktop = true,
            "instance" => instance = true,
            "name" => name = true,
            f => panic!("Invalid field: {}", f),
          }
        });

        Fields {class, desktop, instance, name}
      }
    }
  }
}
