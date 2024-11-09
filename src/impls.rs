// Copyright 2020 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(not(feature = "std"))]
use alloc::{borrow::ToOwned, boxed::Box, string::String, vec::Vec};
use core::{
	mem, str,
};

use crate::{
	error::DecoderError,
	rlpin::Rlp,
	traits::Decodable,
};

pub fn decode_usize(bytes: &[u8]) -> Result<usize, DecoderError> {
	match bytes.len() {
		l if l <= mem::size_of::<usize>() => {
			if bytes[0] == 0 {
				return Err(DecoderError::RlpInvalidIndirection)
			}
			let mut res = 0usize;
			for (i, byte) in bytes.iter().enumerate().take(l) {
				let shift = (l - 1 - i) * 8;
				res += (*byte as usize) << shift;
			}
			Ok(res)
		},
		_ => Err(DecoderError::RlpIsTooBig),
	}
}

impl<T: Decodable> Decodable for Box<T> {
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
		T::decode(rlp).map(Box::new)
	}
}

impl Decodable for bool {
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
		let as_uint = <u8 as Decodable>::decode(rlp)?;
		match as_uint {
			0 => Ok(false),
			1 => Ok(true),
			_ => Err(DecoderError::Custom("invalid boolean value")),
		}
	}
}

impl Decodable for Vec<u8> {
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
		rlp.decoder().decode_value(|bytes| Ok(bytes.to_vec()))
	}
}

// impl Decodable for Bytes {
// 	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
// 		rlp.decoder().decode_value(|bytes| Ok(Bytes::copy_from_slice(bytes)))
// 	}
// }

// impl Decodable for BytesMut {
// 	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
// 		rlp.decoder().decode_value(|bytes| Ok(bytes.into()))
// 	}
// }

impl<T> Decodable for Option<T>
where
	T: Decodable,
{
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
		let items = rlp.item_count()?;
		match items {
			1 => rlp.val_at(0).map(Some),
			0 => Ok(None),
			_ => Err(DecoderError::RlpIncorrectListLen),
		}
	}
}

impl Decodable for u8 {
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
		rlp.decoder().decode_value(|bytes| match bytes.len() {
			1 if bytes[0] != 0 => Ok(bytes[0]),
			0 => Ok(0),
			1 => Err(DecoderError::RlpInvalidIndirection),
			_ => Err(DecoderError::RlpIsTooBig),
		})
	}
}

macro_rules! impl_decodable_for_u {
	($name: ident) => {
		impl Decodable for $name {
			fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
				rlp.decoder().decode_value(|bytes| match bytes.len() {
					0 | 1 => u8::decode(rlp).map(|v| v as $name),
					l if l <= mem::size_of::<$name>() => {
						if bytes[0] == 0 {
							return Err(DecoderError::RlpInvalidIndirection)
						}
						let mut res = 0 as $name;
						for (i, byte) in bytes.iter().enumerate().take(l) {
							let shift = (l - 1 - i) * 8;
							res += (*byte as $name) << shift;
						}
						Ok(res)
					},
					_ => Err(DecoderError::RlpIsTooBig),
				})
			}
		}
	};
}

impl_decodable_for_u!(u16);
impl_decodable_for_u!(u32);
impl_decodable_for_u!(u64);
impl_decodable_for_u!(u128);

impl Decodable for usize {
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
		u64::decode(rlp).map(|value| value as usize)
	}
}

impl Decodable for String {
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
		rlp.decoder().decode_value(|bytes| {
			match str::from_utf8(bytes) {
				Ok(s) => Ok(s.to_owned()),
				// consider better error type here
				Err(_err) => Err(DecoderError::RlpExpectedToBeData),
			}
		})
	}
}
