// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub type Result<T, E = Error> = ::core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
  CoreError(identity_core::Error),
  IotaError(identity_iota::Error),
  CryptoError(crypto::Error),
  IoError(std::io::Error),
  ActorSystemError(riker::system::SystemError),
  StrongholdError(iota_stronghold::Error),
  StrongholdResult(String),
  StrongholdPasswordNotSet,
  StrongholdProcedureFailure,
  StrongholdInvalidAddress,
  IdentityDuplicateName,
  MissingStorageAdapter,
  PartialStorageEncryption,
  GenerateMnemonicError,
}

impl From<identity_core::Error> for Error {
  fn from(other: identity_core::Error) -> Self {
    Self::CoreError(other)
  }
}

impl From<identity_iota::Error> for Error {
  fn from(other: identity_iota::Error) -> Self {
    Self::IotaError(other)
  }
}

impl From<std::io::Error> for Error {
  fn from(other: std::io::Error) -> Self {
    Self::IoError(other)
  }
}

impl From<riker::system::SystemError> for Error {
  fn from(other: riker::system::SystemError) -> Self {
    Self::ActorSystemError(other)
  }
}

impl From<iota_stronghold::Error> for Error {
  fn from(other: iota_stronghold::Error) -> Self {
    Self::StrongholdError(other)
  }
}

pub trait PleaseDontMakeYourOwnResult<T> {
  fn to_result(self) -> Result<T>;
}