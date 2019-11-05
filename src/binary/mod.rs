//! This module provides the necessary structures and logic to read values from a binary Ion
//! data stream.
mod header;
mod nibbles;
mod type_code;
mod var_uint;
mod var_int;

pub(crate) use type_code::IonTypeCode;
