use serde_json::to_vec;
use url::Url;

use crate::crypto;
use crate::error::Error;
use crate::error::Result;
use crate::jwk::JwkOperation;
use crate::jwk::JwkParams;
use crate::jwk::JwkParamsEc;
use crate::jwk::JwkParamsOct;
use crate::jwk::JwkParamsOkp;
use crate::jwk::JwkParamsRsa;
use crate::jwk::JwkType;
use crate::jwk::JwkUse;
use crate::lib::*;
use crate::utils::encode_b64;

/// JSON Web Key.
///
/// [More Info](https://tools.ietf.org/html/rfc7517#section-4)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Jwk {
  /// Key Type.
  ///
  /// Identifies the cryptographic algorithm family used with the key.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7517#section-4.1)
  kty: JwkType,
  /// Public Key Use.
  ///
  /// Identifies the intended use of the public key.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7517#section-4.2)
  #[serde(rename = "use", skip_serializing_if = "Option::is_none")]
  use_: Option<JwkUse>,
  /// Key Operations.
  ///
  /// Identifies the operation(s) for which the key is intended to be used.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7517#section-4.3)
  #[serde(skip_serializing_if = "Option::is_none")]
  key_ops: Option<Vec<JwkOperation>>,
  /// Algorithm.
  ///
  /// Identifies the algorithm intended for use with the key.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7517#section-4.4)
  #[serde(skip_serializing_if = "Option::is_none")]
  alg: Option<String>,
  /// Key ID.
  ///
  /// Used to match a specific key among a set of keys within a JWK Set.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7517#section-4.5)
  #[serde(skip_serializing_if = "Option::is_none")]
  kid: Option<String>,
  /// X.509 URL.
  ///
  /// A URI that refers to a resource for an X.509 public key certificate or
  /// certificate chain.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7517#section-4.6)
  #[serde(skip_serializing_if = "Option::is_none")]
  x5u: Option<Url>,
  /// X.509 Certificate Chain.
  ///
  /// Contains a chain of one or more PKIX certificates.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7517#section-4.7)
  #[serde(skip_serializing_if = "Option::is_none")]
  x5c: Option<Vec<String>>,
  /// X.509 Certificate SHA-1 Thumbprint.
  ///
  /// A base64url-encoded SHA-1 thumbprint of the DER encoding of an X.509
  /// certificate.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7517#section-4.8)
  #[serde(skip_serializing_if = "Option::is_none")]
  x5t: Option<String>,
  /// X.509 Certificate SHA-256 Thumbprint.
  ///
  /// A base64url-encoded SHA-256 thumbprint of the DER encoding of an X.509
  /// certificate.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7517#section-4.9)
  #[serde(rename = "x5t#S256", skip_serializing_if = "Option::is_none")]
  x5t_s256: Option<String>,
  /// Type-Specific Key Properties.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7517#section-4)
  #[serde(flatten)]
  params: JwkParams,
}

impl Jwk {
  /// Creates a new `Jwk` with the given `kty` parameter.
  pub const fn new(kty: JwkType) -> Self {
    Self {
      kty,
      use_: None,
      key_ops: None,
      alg: None,
      kid: None,
      x5u: None,
      x5c: None,
      x5t: None,
      x5t_s256: None,
      params: JwkParams::new(kty),
    }
  }

  /// Creates a new `Jwk` from the given params.
  pub fn from_params(params: impl Into<JwkParams>) -> Self {
    let params: JwkParams = params.into();

    Self {
      kty: params.kty(),
      use_: None,
      key_ops: None,
      alg: None,
      kid: None,
      x5u: None,
      x5c: None,
      x5t: None,
      x5t_s256: None,
      params,
    }
  }

  /// Returns the value for the key type parameter (kty).
  pub fn kty(&self) -> JwkType {
    self.kty
  }

  /// Sets a value for the key type parameter (kty).
  pub fn set_kty(&mut self, value: impl Into<JwkType>) {
    self.kty = value.into();
    self.params = JwkParams::new(self.kty);
  }

  /// Returns the value for the use property (use).
  pub fn use_(&self) -> Option<JwkUse> {
    self.use_
  }

  /// Sets a value for the key use parameter (use).
  pub fn set_use(&mut self, value: impl Into<JwkUse>) {
    self.use_ = Some(value.into());
  }

  /// Returns the value for the key operations parameter (key_ops).
  pub fn key_ops(&self) -> Option<&[JwkOperation]> {
    self.key_ops.as_deref()
  }

  /// Sets values for the key operations parameter (key_ops).
  pub fn set_key_ops(&mut self, value: impl IntoIterator<Item = impl Into<JwkOperation>>) {
    self.key_ops = Some(value.into_iter().map(Into::into).collect());
  }

  /// Returns the value for the algorithm property (alg).
  pub fn alg(&self) -> Option<&str> {
    self.alg.as_deref()
  }

  /// Sets a value for the algorithm property (alg).
  pub fn set_alg(&mut self, value: impl Into<String>) {
    self.alg = Some(value.into());
  }

  /// Returns the value of the key ID property (kid).
  pub fn kid(&self) -> Option<&str> {
    self.kid.as_deref()
  }

  /// Sets a value for the key ID property (kid).
  pub fn set_kid(&mut self, value: impl Into<String>) {
    self.kid = Some(value.into());
  }

  /// Returns the value of the X.509 URL property (x5u).
  pub fn x5u(&self) -> Option<&Url> {
    self.x5u.as_ref()
  }

  /// Sets a value for the X.509 URL property (x5u).
  pub fn set_x5u(&mut self, value: impl Into<Url>) {
    self.x5u = Some(value.into());
  }

  /// Returns the value of the X.509 certificate chain property (x5c).
  pub fn x5c(&self) -> Option<&[String]> {
    self.x5c.as_deref()
  }

  /// Sets values for the X.509 certificate chain property (x5c).
  pub fn set_x5c(&mut self, value: impl IntoIterator<Item = impl Into<String>>) {
    self.x5c = Some(value.into_iter().map(Into::into).collect());
  }

  /// Returns the value of the X.509 certificate SHA-1 thumbprint property
  /// (x5t).
  pub fn x5t(&self) -> Option<&str> {
    self.x5t.as_deref()
  }

  /// Sets a value for the X.509 certificate SHA-1 thumbprint property (x5t).
  pub fn set_x5t(&mut self, value: impl Into<String>) {
    self.x5t = Some(value.into());
  }

  /// Returns the value of the X.509 certificate SHA-256 thumbprint property
  /// (x5t#S256).
  pub fn x5t_s256(&self) -> Option<&str> {
    self.x5t_s256.as_deref()
  }

  /// Sets a value for the X.509 certificate SHA-256 thumbprint property
  /// (x5t#S256).
  pub fn set_x5t_s256(&mut self, value: impl Into<String>) {
    self.x5t_s256 = Some(value.into());
  }

  /// Returns a reference to the custom JWK properties.
  pub fn params(&self) -> &JwkParams {
    &self.params
  }

  /// Returns a mutable reference to the custom JWK properties.
  pub fn params_mut(&mut self) -> &mut JwkParams {
    &mut self.params
  }

  /// Sets the value of the custom JWK properties.
  pub fn set_params(&mut self, value: impl Into<JwkParams>) {
    match (self.kty, value.into()) {
      (JwkType::Ec, value @ JwkParams::Ec(_)) => {
        self.set_params_unchecked(value);
      }
      (JwkType::Rsa, value @ JwkParams::Rsa(_)) => {
        self.set_params_unchecked(value);
      }
      (JwkType::Oct, value @ JwkParams::Oct(_)) => {
        self.set_params_unchecked(value);
      }
      (JwkType::Okp, value @ JwkParams::Okp(_)) => {
        self.set_params_unchecked(value);
      }
      (_, _) => {
        // TODO: Return an error
      }
    }
  }

  /// Sets the value of the custom JWK properties. Does not assert valid params.
  pub fn set_params_unchecked(&mut self, value: impl Into<JwkParams>) {
    self.params = value.into();
  }

  pub fn try_ec_params(&self) -> Result<&JwkParamsEc> {
    match self.params() {
      JwkParams::Ec(params) => Ok(params),
      _ => Err(Error::KeyError("Ec")),
    }
  }

  pub fn try_ec_params_mut(&mut self) -> Result<&mut JwkParamsEc> {
    match self.params_mut() {
      JwkParams::Ec(params) => Ok(params),
      _ => Err(Error::KeyError("Ec")),
    }
  }

  pub fn try_rsa_params(&self) -> Result<&JwkParamsRsa> {
    match self.params() {
      JwkParams::Rsa(params) => Ok(params),
      _ => Err(Error::KeyError("Rsa")),
    }
  }

  pub fn try_rsa_params_mut(&mut self) -> Result<&mut JwkParamsRsa> {
    match self.params_mut() {
      JwkParams::Rsa(params) => Ok(params),
      _ => Err(Error::KeyError("Rsa")),
    }
  }

  pub fn try_oct_params(&self) -> Result<&JwkParamsOct> {
    match self.params() {
      JwkParams::Oct(params) => Ok(params),
      _ => Err(Error::KeyError("Oct")),
    }
  }

  pub fn try_oct_params_mut(&mut self) -> Result<&mut JwkParamsOct> {
    match self.params_mut() {
      JwkParams::Oct(params) => Ok(params),
      _ => Err(Error::KeyError("Oct")),
    }
  }

  pub fn try_okp_params(&self) -> Result<&JwkParamsOkp> {
    match self.params() {
      JwkParams::Okp(params) => Ok(params),
      _ => Err(Error::KeyError("Okp")),
    }
  }

  pub fn try_okp_params_mut(&mut self) -> Result<&mut JwkParamsOkp> {
    match self.params_mut() {
      JwkParams::Okp(params) => Ok(params),
      _ => Err(Error::KeyError("Okp")),
    }
  }

  // ===========================================================================
  // Thumbprint
  // ===========================================================================

  /// Creates a Thumbprint of the JSON Web Key according to [RFC7638](https://tools.ietf.org/html/rfc7638).
  ///
  /// `SHA2-256` is used as the hash function *H*.
  ///
  /// The thumbprint is returned as a base64url-encoded string.
  pub fn thumbprint_b64(&self) -> Result<String> {
    self.thumbprint_raw().map(|thumbprint| encode_b64(&thumbprint))
  }

  /// Creates a Thumbprint of the JSON Web Key according to [RFC7638](https://tools.ietf.org/html/rfc7638).
  ///
  /// `SHA2-256` is used as the hash function *H*.
  ///
  /// The thumbprint is returned as an unencoded vector of bytes.
  pub fn thumbprint_raw(&self) -> Result<[u8; SHA256_LEN]> {
    let mut data: BTreeMap<&str, &str> = BTreeMap::new();

    data.insert("kty", self.kty.name());

    match &self.params {
      JwkParams::Ec(JwkParamsEc { crv, x, y, .. }) => {
        data.insert("crv", crv);
        data.insert("x", x);
        data.insert("y", y);
      }
      JwkParams::Rsa(JwkParamsRsa { n, e, .. }) => {
        data.insert("e", e);
        data.insert("n", n);
      }
      JwkParams::Oct(JwkParamsOct { k }) => {
        data.insert("k", k);
      }
      JwkParams::Okp(JwkParamsOkp { crv, x, .. }) => {
        data.insert("crv", crv);
        data.insert("x", x);
      }
    }

    Ok(crypto::sha256(&to_vec(&data)?))
  }

  // ===========================================================================
  // Validations
  // ===========================================================================

  pub fn check_use(&self, expected: JwkUse) -> Result<()> {
    match self.use_() {
      Some(value) if value == expected => Ok(()),
      Some(_) => Err(Error::InvalidClaim("use")),
      None => Ok(()),
    }
  }

  pub fn check_ops(&self, expected: JwkOperation) -> Result<()> {
    match self.key_ops() {
      Some(ops) if ops.contains(&expected) => Ok(()),
      Some(_) => Err(Error::InvalidClaim("key_ops")),
      None => Ok(()),
    }
  }

  pub fn check_alg(&self, expected: &str) -> Result<()> {
    match self.alg() {
      Some(value) if value == expected => Ok(()),
      Some(_) => Err(Error::InvalidClaim("alg")),
      None => Ok(()),
    }
  }

  pub fn check_signing_key(&self, algorithm: &str) -> Result<()> {
    self.check_use(JwkUse::Signature)?;
    self.check_ops(JwkOperation::Sign)?;
    self.check_alg(algorithm)?;

    Ok(())
  }

  pub fn check_verifying_key(&self, algorithm: &str) -> Result<()> {
    self.check_use(JwkUse::Signature)?;
    self.check_ops(JwkOperation::Verify)?;
    self.check_alg(algorithm)?;

    Ok(())
  }

  pub fn check_encryption_key(&self, algorithm: &str) -> Result<()> {
    self.check_use(JwkUse::Encryption)?;
    self.check_ops(JwkOperation::Encrypt)?;
    self.check_alg(algorithm)?;

    Ok(())
  }

  pub fn check_decryption_key(&self, algorithm: &str) -> Result<()> {
    self.check_use(JwkUse::Encryption)?;
    self.check_ops(JwkOperation::Decrypt)?;
    self.check_alg(algorithm)?;

    Ok(())
  }

  pub fn try_ec_curve(&self) -> Result<EcCurve> {
    if let JwkParams::Ec(inner) = self.params() {
      inner.try_ec_curve()
    } else {
      Err(Error::KeyError("Ec Curve"))
    }
  }

  pub fn try_ed_curve(&self) -> Result<EdCurve> {
    if let JwkParams::Okp(inner) = self.params() {
      inner.try_ed_curve()
    } else {
      Err(Error::KeyError("Ed Curve"))
    }
  }

  pub fn try_ecx_curve(&self) -> Result<EcxCurve> {
    if let JwkParams::Okp(inner) = self.params() {
      inner.try_ecx_curve()
    } else {
      Err(Error::KeyError("Ecx Curve"))
    }
  }

  pub fn to_public(&self) -> Jwk {
    let mut public: Jwk = Jwk::from_params(self.params().to_public());

    if let Some(value) = self.use_() {
      public.set_use(value);
    }

    if let Some(value) = self.key_ops() {
      public.set_key_ops(value.iter().map(|op| op.invert()));
    }

    if let Some(value) = self.alg() {
      public.set_alg(value);
    }

    if let Some(value) = self.kid() {
      public.set_kid(value);
    }

    public
  }
}