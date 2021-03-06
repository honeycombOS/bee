// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_common::packable::Packable;
use bee_message::prelude::*;

use core::str::FromStr;

const OUTPUT_ID: &str = "52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c6492a00";

#[test]
fn valid_new() {
    let output_id = OutputId::from_str(OUTPUT_ID).unwrap();
    let input = UTXOInput::new(*output_id.transaction_id(), output_id.index()).unwrap();

    assert_eq!(*input.output_id(), output_id);
}

#[test]
fn valid_from() {
    let output_id = OutputId::from_str(OUTPUT_ID).unwrap();
    let input: UTXOInput = output_id.into();

    assert_eq!(*input.output_id(), output_id);
}

#[test]
fn valid_from_str() {
    let output_id = OutputId::from_str(OUTPUT_ID).unwrap();
    let input = UTXOInput::from_str(OUTPUT_ID).unwrap();

    assert_eq!(*input.output_id(), output_id);
}

#[test]
fn from_str_to_str() {
    let input = UTXOInput::from_str(OUTPUT_ID).unwrap();

    assert_eq!(input.to_string(), OUTPUT_ID);
}

#[test]
fn pack_unpack() {
    let output_id = OutputId::from_str(OUTPUT_ID).unwrap();
    let input_1 = UTXOInput::new(*output_id.transaction_id(), output_id.index()).unwrap();
    let input_2 = UTXOInput::unpack(&mut input_1.pack_new().as_slice()).unwrap();

    assert_eq!(input_1, input_2);
}
