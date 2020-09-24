use crate::{api::generated::wrapper::users as users_wrapper, pants::Pants};

pub struct Users<'a> {
  pants: &'a mut Pants,
}

impl<'a> Users<'a> {
  pub fn build(pants: &'a mut Pants) -> Users<'a> {
    Users { pants }
  }
}
