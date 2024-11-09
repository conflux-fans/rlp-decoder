# RLP-decoder

Recursive-length-prefix decoding in Rust.

This project is forked from parity rlp library, and removed all the encoding functions. To be used in embedded systems.

The original parity rlp library heavily relies on [Bytes](https://github.com/tokio-rs/bytes) crate, which can not work on chipsets do not support CAS operations. 