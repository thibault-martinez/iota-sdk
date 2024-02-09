// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    address::{AccountAddress, Address, AnchorAddress, Ed25519Address, ImplicitAccountCreationAddress, NftAddress},
    output::{AccountId, AnchorId, NftId},
    rand::{bytes::rand_bytes_array, number::rand_number},
};

/// Generates a random [`Ed25519Address`].
pub fn rand_ed25519_address() -> Ed25519Address {
    Ed25519Address::new(rand_bytes_array())
}

/// Generates a random [`AccountAddress`].
pub fn rand_account_address() -> AccountAddress {
    AccountAddress::new(AccountId::from(rand_bytes_array()))
}

/// Generates a random [`NftAddress`].
pub fn rand_nft_address() -> NftAddress {
    NftAddress::new(NftId::from(rand_bytes_array()))
}

/// Generates a random [`AnchorAddress`].
pub fn rand_anchor_address() -> AnchorAddress {
    AnchorAddress::new(AnchorId::from(rand_bytes_array()))
}

/// Generates a random [`ImplicitAccountCreationAddress`].
pub fn rand_implicit_address() -> ImplicitAccountCreationAddress {
    ImplicitAccountCreationAddress::from(rand_ed25519_address())
}

/// Generates a random [`Address`].
pub fn rand_address() -> Address {
    match rand_number::<u64>() % 5 {
        0 => rand_ed25519_address().into(),
        1 => rand_account_address().into(),
        2 => rand_nft_address().into(),
        3 => rand_anchor_address().into(),
        4 => rand_implicit_address().into(),
        _ => unreachable!(),
    }
}

/// Generates a random base [`Address`].
pub fn rand_base_address() -> Address {
    match rand_number::<u64>() % 4 {
        0 => rand_ed25519_address().into(),
        1 => rand_account_address().into(),
        2 => rand_nft_address().into(),
        3 => rand_anchor_address().into(),
        _ => unreachable!(),
    }
}
