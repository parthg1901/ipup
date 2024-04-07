// Copyright 2021-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use num_derive::FromPrimitive;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};

pub const CUSTOMSYSCALL_ACTOR_NAME: &str = "customsyscall";

#[derive(Default, Debug, Serialize_tuple, Deserialize_tuple)]
pub struct Pbkdf2Params {
    pub password: u64,
    pub salt: u64,
}

#[derive(Default, Debug, Serialize_tuple, Deserialize_tuple)]
pub struct Argon2Params {
    pub password: u64,
    pub salt: u64,
}

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
    Blake2 = frc42_dispatch::method_hash!("Blake2"),
    Crc = frc42_dispatch::method_hash!("Crc"),
    Pbkdf2 = frc42_dispatch::method_hash!("Pbkdf2"),
    Argon2 = frc42_dispatch::method_hash!("Argon2"),
    Rand = frc42_dispatch::method_hash!("Rand"),
}

