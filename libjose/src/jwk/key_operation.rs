use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

/// Supported algorithms for the JSON Web Key `key_ops` property.
///
/// [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-key-operations)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum JwkOperation {
  /// Compute digital signature or MAC.
  Sign,
  /// Verify digital signature or MAC.
  Verify,
  /// Encrypt content.
  Encrypt,
  /// Decrypt content and validate decryption, if applicable.
  Decrypt,
  /// Encrypt key.
  WrapKey,
  /// Decrypt key and validate decryption, if applicable.
  UnwrapKey,
  /// Derive key.
  DeriveKey,
  /// Derive bits not to be used as a key.
  DeriveBits,
}

impl JwkOperation {
  /// Returns the JWK "key_ops" as a `str` slice.
  pub const fn name(&self) -> &'static str {
    match self {
      Self::Sign => "sign",
      Self::Verify => "verify",
      Self::Encrypt => "encrypt",
      Self::Decrypt => "decrypt",
      Self::WrapKey => "wrapKey",
      Self::UnwrapKey => "unwrapKey",
      Self::DeriveKey => "deriveKey",
      Self::DeriveBits => "deriveBits",
    }
  }
}

impl Display for JwkOperation {
  fn fmt(&self, f: &mut Formatter) -> Result {
    f.write_str(self.name())
  }
}
