// Copyright 2021-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT
use fvm::call_manager::CallManager;
use fvm::gas::Gas;
use fvm::kernel::prelude::*;
use fvm::kernel::Result;
use fvm::kernel::{
    ActorOps, CryptoOps, DebugOps, EventOps, IpldBlockOps, MessageOps, NetworkOps, RandomnessOps,
    SelfOps, SendOps, SyscallHandler, UpgradeOps,
};
use fvm::syscalls::Linker;
use fvm::DefaultKernel;
use fvm_shared::clock::ChainEpoch;
use fvm_shared::randomness::RANDOMNESS_LENGTH;
use fvm_shared::sys::out::network::NetworkContext;
use fvm_shared::sys::out::vm::MessageContext;
use fvm_shared::{address::Address, econ::TokenAmount, ActorID, MethodNum};

use ambassador::Delegate;
use cid::Cid;

use blake2::Blake2bVar;
use blake2::digest::{Update, VariableOutput};

use pbkdf2::{pbkdf2_hmac, pbkdf2_hmac_array};
use sha2::Sha256;

use crc::{Crc, Algorithm};

use argon2::{Config, Variant, Version};

use rand::Rng;

// we define a single custom syscall which simply doubles the input
pub trait CustomKernel: Kernel {
    fn blake_2_hash(&self, hash_integer: u64) -> Result<u64>;
    fn crc_hash(&self, hash_integer: u64) -> Result<u64>;
    fn pbkdf2_hash(&self, password: u64, salt: u64) -> Result<u64>;
    fn argon2_hash(&self, password: u64, salt: u64) -> Result<u64>;
    fn random_num(&self) -> Result<u64>;

}

// our custom kernel extends the filecoin kernel
#[derive(Delegate)]
#[delegate(IpldBlockOps, where = "C: CallManager")]
#[delegate(ActorOps, where = "C: CallManager")]
#[delegate(CryptoOps, where = "C: CallManager")]
#[delegate(DebugOps, where = "C: CallManager")]
#[delegate(EventOps, where = "C: CallManager")]
#[delegate(MessageOps, where = "C: CallManager")]
#[delegate(NetworkOps, where = "C: CallManager")]
#[delegate(RandomnessOps, where = "C: CallManager")]
#[delegate(SelfOps, where = "C: CallManager")]
#[delegate(SendOps<K>, generics = "K", where = "K: CustomKernel")]
#[delegate(UpgradeOps<K>, generics = "K", where = "K: CustomKernel")]
pub struct CustomKernelImpl<C>(pub DefaultKernel<C>);

fn int_to_str(hash_integer: u64) -> String {
    println!("Integer representation of the hexadecimal string: {}", hash_integer);


    // Convert integer to hexadecimal string
    let hex_string = format!("{:X}", hash_integer);

    // Print hexadecimal string
    println!("Hexadecimal representation of {} is {}", hash_integer, hex_string);

    let bytes = hex::decode(hex_string).unwrap();

    // Convert bytes to string
    let string = String::from_utf8_lossy(&bytes);
    String::from(string)
}

impl<C> CustomKernel for CustomKernelImpl<C>
where
    C: CallManager,
    CustomKernelImpl<C>: Kernel,
{
    fn blake_2_hash(&self, hash_integer: u64) -> Result<u64> {
        // Here we have access to the Kernel structure and can call
        // any of its methods, send messages, etc.

        // We can also run an external program, link to any rust library
        // access the network, etc.

        let string = int_to_str(hash_integer.clone());

        // Print the string
        println!("String: {}", string);

        let mut hasher = Blake2bVar::new(10).unwrap();

        // write input message
        hasher.update(string.as_bytes());


        let mut buf = [0u8; 10];
        hasher.finalize_variable(&mut buf).unwrap();
        let mut hash: u64 = 0;
        for byte in &buf {
            hash = (hash << 8) | (*byte as u64);
        }

        // Print the hash as an integer
        println!("Hash as integer: {}", hash);
        Ok(hash)
    }

    fn crc_hash(&self, hash_integer: u64) -> Result<u64> {
        let string = int_to_str(hash_integer.clone());
        const CUSTOM_ALG: Algorithm<u64> = Algorithm {
            width: 64,
            poly: 0x8005,
            init: 0xffff,
            refin: false,
            refout: false,
            xorout: 0x0000,
            check: 0xaee7,
            residue: 0x0000
        };
        let crc = Crc::<u64>::new(&CUSTOM_ALG);
        let mut digest = crc.digest();
        digest.update(string.as_bytes());
        let hash: u64 = digest.finalize();
        // // Print the hash as an integer
        println!("Hash as integer: {}", hash);
        Ok(hash)
    }

    fn pbkdf2_hash(&self, password: u64, salt: u64) -> Result<u64> {
        let password_string = int_to_str(password.clone());
        let salt_string = int_to_str(salt.clone());

        let n = 1000;
        let mut key1 = [0u8; 8];
        pbkdf2_hmac::<Sha256>(password_string.as_bytes(), salt_string.as_bytes(), n, &mut key1);

        let mut hash_integer: u64 = 0;
        for byte in &key1 {
            hash_integer = (hash_integer << 8) | (*byte as u64);
        }

        // Print the hash as an integer
        println!("Hash as integer: {}", hash_integer);
        Ok(hash_integer as u64)
    }

    fn argon2_hash(&self, password: u64, salt: u64) -> Result<u64> {
        let password_string = int_to_str(password.clone());
        let salt_string = int_to_str(salt.clone());

        // Argon2 configuration
        let config = Config {
            variant: Variant::Argon2i,
            version: Version::Version13,
            mem_cost: 65,
            time_cost: 1,
            lanes: 4,
            secret: &[],
            ad: &[],
            hash_length: 6
        };

        // Hash the password
        let hash = argon2::hash_encoded(password_string.as_bytes(), salt_string.as_bytes(), &config).unwrap();

        // Print the hashed password
        println!("Hashed password: {}", hash);  


        let parts: Vec<&str> = hash.split('$').collect();

        // Extract the hash from the parts
        let hash = parts[5]; // The hash is at index 5

        println!("Hash extracted: {}", hash);
        let hex_string: &str = &hash.bytes()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .concat();

        // Print the hexadecimal representation
        println!("Hexadecimal representation {}", hex_string);
    
        let int_value = match u64::from_str_radix(hex_string, 16) {
            Ok(i) => i,
            Err(e) => 0
        };
        Ok(int_value as u64)
    }

    fn random_num(&self) -> Result<u64> {
        let mut rng = rand::thread_rng();


        let u1 = rng.gen::<f64>();
        let u2 = rng.gen::<f64>();

        let z0: u64 = (((-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()) * 10_f64.powf(16.0)).abs() as u64;
        let z1: u64 = (((-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).sin()) * 10_f64.powf(16.0)).abs() as u64;

        println!("{}",z0 + z1);
        Ok(z0+z1)
    }
}

impl<C> Kernel for CustomKernelImpl<C>
where
    C: CallManager,
{
    type CallManager = C;
    type Limiter = <DefaultKernel<C> as Kernel>::Limiter;

    fn into_inner(self) -> (Self::CallManager, BlockRegistry)
    where
        Self: Sized,
    {
        self.0.into_inner()
    }

    fn new(
        mgr: C,
        blocks: BlockRegistry,
        caller: ActorID,
        actor_id: ActorID,
        method: MethodNum,
        value_received: TokenAmount,
        read_only: bool,
    ) -> Self {
        CustomKernelImpl(DefaultKernel::new(
            mgr,
            blocks,
            caller,
            actor_id,
            method,
            value_received,
            read_only,
        ))
    }

    fn machine(&self) -> &<Self::CallManager as CallManager>::Machine {
        self.0.machine()
    }

    fn limiter_mut(&mut self) -> &mut Self::Limiter {
        self.0.limiter_mut()
    }

    fn gas_available(&self) -> Gas {
        self.0.gas_available()
    }

    fn charge_gas(&self, name: &str, compute: Gas) -> Result<GasTimer> {
        self.0.charge_gas(name, compute)
    }
}

impl<K> SyscallHandler<K> for CustomKernelImpl<K::CallManager>
where
    K: CustomKernel
        + ActorOps
        + SendOps
        + UpgradeOps
        + IpldBlockOps
        + CryptoOps
        + DebugOps
        + EventOps
        + MessageOps
        + NetworkOps
        + RandomnessOps
        + SelfOps,
{
    fn link_syscalls(linker: &mut Linker<K>) -> anyhow::Result<()> {
        DefaultKernel::<K::CallManager>::link_syscalls(linker)?;

        linker.link_syscall("my_custom_kernel", "blake_2_hash", blake_2_hash)?;

        linker.link_syscall("my_custom_kernel", "crc_hash", crc_hash)?;

        linker.link_syscall("my_custom_kernel", "pbkdf2_hash", pbkdf2_hash)?;

        linker.link_syscall("my_custom_kernel", "argon2_hash", argon2_hash)?;

        linker.link_syscall("my_custom_kernel", "random_num", random_num)?;

        Ok(())
    }
}

pub fn blake_2_hash(context: fvm::syscalls::Context<'_, impl CustomKernel>, hash_integer: u64) -> Result<u64> {
    context.kernel.blake_2_hash(hash_integer)
}

pub fn crc_hash(context: fvm::syscalls::Context<'_, impl CustomKernel>, hash_integer: u64) -> Result<u64> {
    context.kernel.crc_hash(hash_integer)
}

pub fn pbkdf2_hash(context: fvm::syscalls::Context<'_, impl CustomKernel>, password: u64, salt: u64) -> Result<u64> {
    context.kernel.pbkdf2_hash(password, salt)
}

pub fn argon2_hash(context: fvm::syscalls::Context<'_, impl CustomKernel>, password: u64, salt: u64) -> Result<u64> {
    context.kernel.argon2_hash(password, salt)
}

pub fn random_num(context: fvm::syscalls::Context<'_, impl CustomKernel>) -> Result<u64> {
    context.kernel.random_num()
}
