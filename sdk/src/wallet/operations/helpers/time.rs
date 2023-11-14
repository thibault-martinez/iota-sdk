// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    types::block::{address::Address, output::Output, slot::SlotIndex},
    wallet::types::OutputData,
};

// Check if an output can be unlocked by the wallet address at the current time
pub(crate) fn can_output_be_unlocked_now(
    wallet_address: &Address,
    output_data: &OutputData,
    slot_index: SlotIndex,
    min_committable_age: u32,
    max_committable_age: u32,
) -> crate::wallet::Result<bool> {
    if let Some(unlock_conditions) = output_data.output.unlock_conditions() {
        if unlock_conditions.is_timelocked(slot_index, min_committable_age) {
            return Ok(false);
        }
    }

    let required_address = output_data
        .output
        .required_address(slot_index, min_committable_age, max_committable_age)?
        // TODO
        .unwrap();

    Ok(wallet_address == &required_address)
}

// Check if an output can be unlocked by one of the account addresses at the current time and at any
// point in the future
pub(crate) fn can_output_be_unlocked_forever_from_now_on(
    wallet_address: &Address,
    output: &Output,
    slot_index: SlotIndex,
    min_committable_age: u32,
    max_committable_age: u32,
) -> bool {
    if let Some(unlock_conditions) = output.unlock_conditions() {
        if unlock_conditions.is_timelocked(slot_index, min_committable_age) {
            return false;
        }

        // TODO HELP
        // // If there is an expiration unlock condition, we can only unlock it forever from now on, if it's expired and
        // // the return address belongs to the account
        // if let Some(expiration) = unlock_conditions.expiration() {
        //     if let Some(return_address) =
        //         expiration.return_address_expired(slot_index, min_committable_age, max_committable_age)
        //     {
        //         if wallet_address != return_address {
        //             return false;
        //         };
        //     } else {
        //         return false;
        //     }
        // }

        true
    } else {
        false
    }
}
