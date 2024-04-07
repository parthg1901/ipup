// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import "@zondax/filecoin-solidity/contracts/v0.8/types/CommonTypes.sol";
import "@zondax/filecoin-solidity/contracts/v0.8/utils/Misc.sol";
import "@zondax/filecoin-solidity/contracts/v0.8/utils/Actor.sol";
import "./CBOR.sol";

contract IPUP {
    using CBOR for CBOR.CBORBuffer;

    bytes public raw_response;
    int256 public num_response;
    bytes public buffer;

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

    function stringToUint64(string memory s) public pure returns (uint64) {
        // Encode the string as bytes
        bytes memory stringBytes = bytes(s);
        
        // Convert the bytes to a uint64
        return bytesToUint64(stringBytes);
    }

    function bytesToUint64(bytes memory data) public pure returns (uint64) {
        uint result = 0;
        for (uint i = 0; i < data.length && i < 8; i++) {
            result = result + uint64(uint8(data[i])) * (2 ** (8 * (data.length - 1 - i)));
        }
        return uint64(result);
    }

}