use crate::pants::Pants;

pub struct Users<'a> {
  pants: &'a Pants,
}

impl<'a> Users<'a> {
  pub fn build(pants: &'a Pants) -> Users<'a> {
    Users { pants }
  }
}
