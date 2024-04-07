// Copyright 2021-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use fil_actors_runtime::actor_dispatch;
use fil_actors_runtime::actor_error;
use fil_actors_runtime::builtin::singletons::SYSTEM_ACTOR_ADDR;
use fil_actors_runtime::runtime::{ActorCode, Runtime};
use fil_actors_runtime::ActorError;

use crate::{Method, Pbkdf2Params, Argon2Params, CUSTOMSYSCALL_ACTOR_NAME};

fil_actors_runtime::wasm_trampoline!(Actor);

fvm_sdk::sys::fvm_syscalls! {
    module = "my_custom_kernel";
    pub fn blake_2_hash(hash_integer: u64) -> Result<u64>;
    pub fn crc_hash(hash_integer: u64) -> Result<u64>;
    pub fn pbkdf2_hash(password: u64, salt: u64) -> Result<u64>;
    pub fn argon2_hash(password: u64, salt: u64) -> Result<u64>;
    pub fn random_num() -> Result<u64>;
}

pub struct Actor;
impl Actor {
    fn blake_2(rt: &impl Runtime, hash_integer: u64) -> Result<u64, ActorError> {
        rt.validate_immediate_caller_accept_any()?;

        unsafe {
            let value = blake_2_hash(hash_integer).unwrap();
            Ok(value)
        }
    }

    fn crc(rt: &impl Runtime, hash_integer: u64) -> Result<u64, ActorError> {
        rt.validate_immediate_caller_accept_any()?;

        unsafe {
            let value = crc_hash(hash_integer).unwrap();
            Ok(value)
        }
    }
    
    fn pbkdf2(rt: &impl Runtime, params: Pbkdf2Params) -> Result<u64, ActorError> {
        rt.validate_immediate_caller_accept_any()?;

        unsafe {
            let value = pbkdf2_hash(params.password, params.salt).unwrap();
            Ok(value)
        }
    }

    fn argon2(rt: &impl Runtime, params: Argon2Params) -> Result<u64, ActorError> {
        rt.validate_immediate_caller_accept_any()?;

        unsafe {
            let value = argon2_hash(params.password, params.salt).unwrap();
            Ok(value)
        }
    }

    fn rand(rt: &impl Runtime) -> Result<u64, ActorError> {
        rt.validate_immediate_caller_accept_any()?;

        unsafe {
            let value = random_num().unwrap();
            Ok(value)
        }
    }
}

impl ActorCode for Actor {
    type Methods = Method;

    fn name() -> &'static str {
        CUSTOMSYSCALL_ACTOR_NAME
    }

    actor_dispatch! {
        Blake2 => blake_2,
        Crc => crc,
        Pbkdf2 => pbkdf2,
        Argon2 => argon2,
        Rand => rand,
    }
}