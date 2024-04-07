# InterPlanetary Utility Package (IPUP)

<div style="text-align: center;">
    <img src="https://cdn.dorahacks.io/static/files/18eb8831790624b9854f72c4774ab220.png">
    <br>
</div>

**‼️ All the modules in the IPC stack (including the contracts) haven't been audited, tested in depth, or otherwise verified. Moreover, the system is missing critical recovery functionality in case of crashes. There are multiple ways in which you may lose funds moved into an IPC subnet, and we strongly advise against deploying IPC on mainnet and/or using it with tokens with real value.**

IPUP was built during the [Filecoin Data Economy Hackathon 2024](https://dorahacks.io/hackathon/filecoin-data-economy/detail)

IPUP is a pack of multiple custom syscalls which allow extending your contract's on-chain functionality beyond the native level. It currently includes 5 custom syscalls including hashing algorithms such as blake2 and crc64, password encryption algorithms such as pbkdf2 and argon2, along with non-verifiable prng.

## Prerequisites
You must create an IPC subnet from this repo in order to use our Custom Kernel.

Checkout [IPC.md](IPC.md), if you don't know to create an IPC subnet.

## How to use

```
ACTOR_ID = 49
```

### Non-verifiable PRNG
```
Method Number = 4203131911
```
The custom syscall for Non-verifiable PRNG is created using Box-Muller method. It uses ```rand``` crate from rust.

It can be used in solidity contracts to generate a random ```uint64``` number.

TODO - Implement Verifiable Randomness.

**Example Smart contract implementation**

```
function rand() public returns (bytes memory) {
    raw_response = Actor.callByID(
        CommonTypes.FilActorId.wrap(49), //uint64, actor id
        4203131911, // method number
        Misc.NONE_CODEC,
        new bytes(0),
        0,
        true
    );


    return raw_response;
}
```
**How it works** - [RNG Syscall](fendermint/vm/interpreter/src/fvm/customsyscalls/mycustomkernel.rs#L194)

### Blake2
```
Method Number = 3062748980
```
Blake2 is a cryptographic hash function which provides high performance, versatility and security to the users. It is not natively supported in solidity, however it's highly needed in cryptographic operations.

Note - IPUP currently supports hashing of strings with length less than or equal to 8 as IPC currently supports only integers as custom syscall inputs.

We have used a workaround to pass strings as uint64 to the syscalls.

**Example Smart contract implementation**

```
function blake2(string memory data) public returns (bytes memory) {
    uint256 capacity = 1;

    CBOR.CBORBuffer memory buf = CBOR.create(capacity);
    CBOR.writeUInt64(buf, stringToUint64(data));
    raw_response = Actor.callByID(
        CommonTypes.FilActorId.wrap(49), //uint64, actor id
        3062748980, // method number
        Misc.CBOR_CODEC,
        buf.data(),
        0,
        true
    );
    return raw_response;

}
```
**How it works** - [Blake2 Syscall](fendermint/vm/interpreter/src/fvm/customsyscalls/mycustomkernel.rs#L81)

### CRC64
```
Method Number = 1931031948
```
CRC (Cyclic Redundancy Check) is an error-detecting code used in digital networks and storage devices to detect accidental changes to raw data. It operates by generating a fixed-size checksum (typically 16, 32, or 64 bits) based on the data being checked, which is then appended to the data or transmitted alongside it.

It is a highly important algorithm in networking related tasks.

Note - IPUP currently supports hashing of strings with length less than or equal to 8 as mentioned earlier.

**Example Smart contract implementation**

```
function crc(string memory data) public returns (bytes memory) {
    uint256 capacity = 1;

    CBOR.CBORBuffer memory buf = CBOR.create(capacity);
    CBOR.writeUInt64(buf, stringToUint64(data));
    raw_response = Actor.callByID(
        CommonTypes.FilActorId.wrap(49), //uint64, actor id
        1931031948, // method number
        Misc.CBOR_CODEC,
        buf.data(),
        0,
        true
    );
    return raw_response;

}
```
**How it works** - [CRC Syscall](fendermint/vm/interpreter/src/fvm/customsyscalls/mycustomkernel.rs#L111)

### Pbkdf2
```
Method Number = 3173791782
```
PBKDF2 (Password-Based Key Derivation Function 2) is a key derivation function that is used to securely derive cryptographic keys from passwords or passphrases. It's a widely-used algorithm for deriving cryptographic keys from passwords, and it's designed to be resistant to brute-force attacks.

It can be used as layer of authentication for web3 applications.

IPUP can be used to generate a Pbkdf2 hash.

Note - IPUP currently supports hashing of strings with length less than or equal to 8 as mentioned earlier.

TODO - Add a custom syscall for password verification using the generated hash.

**Example Smart contract implementation**

```
function pbkdf2(string memory password, string memory salt) public returns (bytes memory) {
    uint256 capacity = 1;

    CBOR.CBORBuffer memory buf = CBOR.create(capacity);
    CBOR.startFixedArray(buf, 2);
    CBOR.writeUInt64(buf, stringToUint64(password));
    CBOR.writeUInt64(buf, stringToUint64(salt));
    raw_response = Actor.callByID(
        CommonTypes.FilActorId.wrap(49), //uint64, actor id
        3173791782, // method number
        Misc.CBOR_CODEC,
        buf.data(),
        0,
        true
    );
    return raw_response;

}
```
**How it works** - [Pbkdf2 Syscall](fendermint/vm/interpreter/src/fvm/customsyscalls/mycustomkernel.rs#L132)

### Argon2
```
Method Number = 2785015011
```
Argon2 is a key derivation function that was selected as the winner of the Password Hashing Competition (PHC) in 2015. It's designed to securely hash passwords and other sensitive information while also being resistant to various types of attacks, including brute-force, dictionary, and side-channel attacks. Argon2 is considered to be one of the most secure and efficient password hashing algorithms available today, and it's being widely adopted in various applications and security protocols.

It's use case is similar to Pbkdf2. But it is relatively slower. Hence, it should be used for applications which are ready to trade off performance for security.

IPUP implements a basic version of Argon2 by hashing password with salt to give a uint64 hash.

Note - IPUP currently supports hashing of strings with length less than or equal to 8 as mentioned earlier.

TODO - Add a custom syscall for password verification using the generated hash.

**Example Smart contract implementation**

```
function argon2(string memory password, string memory salt) public returns (bytes memory) {
    uint256 capacity = 1;

    CBOR.CBORBuffer memory buf = CBOR.create(capacity);
    CBOR.startFixedArray(buf, 2);
    CBOR.writeUInt64(buf, stringToUint64(password));
    CBOR.writeUInt64(buf, stringToUint64(salt));
    raw_response = Actor.callByID(
        CommonTypes.FilActorId.wrap(49), //uint64, actor id
        2785015011, // method number
        Misc.CBOR_CODEC,
        buf.data(),
        0,
        true
    );
    return raw_response;

}
```
**How it works** - [Argon2 Syscall](fendermint/vm/interpreter/src/fvm/customsyscalls/mycustomkernel.rs#L150)

## Handling Outputs
Custom syscalls give outputs in a CBOR byte string, like - 

```0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000091b6ac0442575d817cd0000000000000000000000000000000000000000000000```


Out of this big string, only - ```6ac0442575d817cd``` is useful for us.
So, make sure to play around and figure out your use case accordingly.

## What's next for IPUP
This project will be fully open-sourced and will be open for contributions after the hackathon.

I had two more features in my mind - 

1. JSON String Schema Validation.
2. A syscall to make On-chain api calls.

But, these went out of scope for this hackathon, so I plan to implement these too.