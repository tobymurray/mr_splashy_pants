pub struct Fullname {
  pub value: String,
}

impl Fullname {
  pub fn new(value: &str) -> Fullname {
    Fullname {
      value: value.to_string(),
    }
  }
}

pub trait Listing {
  fn comments(&self, article: &Fullname);
  fn hot(&self);
  fn new(&self);
  fn random(&self);
  fn rising(&self);
  fn top(&self);
  fn controversial(&self);
}
