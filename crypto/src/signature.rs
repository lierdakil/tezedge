// SPDX-FileCopyrightText: 2023 TriliTech <contact@trili.tech>
// Copyright (c) SimpleStaking, Viable Systems and Tezedge Contributors
//
// Ported from octez: lib_crypto/signature_v1.ml
//
// Copyright (c) 2018 Dynamic Ledger Solutions, Inc. <contact@tezos.com>
// Copyright (c) 2020 Metastate AG <hello@metastate.dev>
// Copyright (c) 2022 Nomadic Labs <contact@nomadic-labs.com>
//
// SPDX-License-Identifier: MIT

use crate::base58::FromBase58CheckError;
use crate::hash::{
    BlsSignature, Ed25519Signature, FromBytesError, HashTrait, HashType, P256Signature,
    Secp256k1Signature, UnknownSignature,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Signature {
    Ed25519(Ed25519Signature),
    Secp256k1(Secp256k1Signature),
    P256(P256Signature),
    Bls(BlsSignature),
    Unknown(UnknownSignature),
}

impl Signature {
    pub fn from_base58_check(data: &str) -> Result<Self, FromBase58CheckError> {
        if data.starts_with("edsig") {
            Ok(Signature::Ed25519(Ed25519Signature::from_b58check(data)?))
        } else if data.starts_with("spsig1") {
            Ok(Signature::Secp256k1(Secp256k1Signature::from_b58check(
                data,
            )?))
        } else if data.starts_with("p2sig") {
            Ok(Signature::P256(P256Signature::from_b58check(data)?))
        } else if data.starts_with("BLsig") {
            Ok(Signature::Bls(BlsSignature::from_b58check(data)?))
        } else {
            Ok(Signature::Unknown(UnknownSignature::from_b58check(data)?))
        }
    }

    pub fn to_base58_check(&self) -> String {
        match self {
            Self::Ed25519(s) => s.to_b58check(),
            Self::Secp256k1(s) => s.to_b58check(),
            Self::P256(s) => s.to_b58check(),
            Self::Bls(s) => s.to_b58check(),
            Self::Unknown(s) => s.to_b58check(),
        }
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Ed25519(s) => s.0.as_ref(),
            Self::Secp256k1(s) => s.0.as_ref(),
            Self::P256(s) => s.0.as_ref(),
            Self::Bls(s) => s.0.as_ref(),
            Self::Unknown(s) => s.0.as_ref(),
        }
    }
}

impl From<Signature> for Vec<u8> {
    fn from(s: Signature) -> Self {
        match s {
            Signature::Ed25519(s) => s.0,
            Signature::Secp256k1(s) => s.0,
            Signature::P256(s) => s.0,
            Signature::Bls(s) => s.0,
            Signature::Unknown(s) => s.0,
        }
    }
}

impl TryFrom<Vec<u8>> for Signature {
    type Error = FromBytesError;

    fn try_from(hash: Vec<u8>) -> Result<Self, Self::Error> {
        if hash.len() == HashType::BlsSignature.size() {
            Ok(Signature::Bls(BlsSignature(hash)))
        } else if hash.len() == HashType::UnknownSignature.size() {
            Ok(Signature::Unknown(UnknownSignature(hash)))
        } else {
            Err(FromBytesError::InvalidSize)
        }
    }
}

impl TryFrom<&[u8]> for Signature {
    type Error = FromBytesError;

    fn try_from(hash: &[u8]) -> Result<Self, Self::Error> {
        if hash.len() == HashType::BlsSignature.size() {
            Ok(Signature::Bls(BlsSignature(hash.to_vec())))
        } else if hash.len() == HashType::UnknownSignature.size() {
            Ok(Signature::Unknown(UnknownSignature(hash.to_vec())))
        } else {
            Err(FromBytesError::InvalidSize)
        }
    }
}

impl TryFrom<&str> for Signature {
    type Error = FromBase58CheckError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Signature::from_base58_check(s)
    }
}

impl ::core::str::FromStr for Signature {
    type Err = FromBase58CheckError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Signature::from_base58_check(s)
    }
}

impl ::std::fmt::Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO - this could be done without the need to perform a heap allocation.
        write!(f, "{}", self.to_base58_check())
    }
}
