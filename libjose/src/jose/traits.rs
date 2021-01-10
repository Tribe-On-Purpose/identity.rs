use crate::jwt::JwtHeader;

pub trait JoseHeader {
  fn common(&self) -> &JwtHeader;

  fn has_claim(&self, claim: &str) -> bool;
}

impl<'a, T: 'a> JoseHeader for &'a T
where
  T: JoseHeader,
{
  fn common(&self) -> &JwtHeader {
    (**self).common()
  }

  fn has_claim(&self, claim: &str) -> bool {
    (**self).has_claim(claim)
  }
}
