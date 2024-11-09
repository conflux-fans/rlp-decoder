// Copyright 2020 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Recursive Length Prefix serialization crate.
//!
//! Allows decoding, and view onto rlp-slice
//!
//! # What should you use when?
//!
//!
//! ### Use `decode` function when:
//! * You want to decode something inline.
//! * You do not work on big set of data.
//! * You want to decode whole rlp at once.
//!
//! ### Use `Rlp` when:
//! * You need to handle data corruption errors.
//! * You are working on input data.
//! * You want to get view onto rlp-slice.
//! * You don't want to decode whole rlp at once.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod error;
mod impls;
mod rlpin;
mod traits;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

pub use self::{
	error::DecoderError,
	rlpin::{PayloadInfo, Prototype, Rlp, RlpIterator},
	traits::Decodable,
};

/// The RLP encoded empty data (used to mean "null value").
pub const NULL_RLP: [u8; 1] = [0x80; 1];
/// The RLP encoded empty list.
pub const EMPTY_LIST_RLP: [u8; 1] = [0xC0; 1];

/// Shortcut function to decode trusted rlp
///
/// ```
/// let data = vec![0x83, b'c', b'a', b't'];
/// let animal: String = rlp::decode(&data).expect("could not decode");
/// assert_eq!(animal, "cat".to_owned());
/// ```
pub fn decode<T>(bytes: &[u8]) -> Result<T, DecoderError>
where
	T: Decodable,
{
	let rlp = Rlp::new(bytes);
	rlp.as_val()
}

pub fn decode_list<T>(bytes: &[u8]) -> Vec<T>
where
	T: Decodable,
{
	let rlp = Rlp::new(bytes);
	rlp.as_list().expect("trusted rlp should be valid")
}
