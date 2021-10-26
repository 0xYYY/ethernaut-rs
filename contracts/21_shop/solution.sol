// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

interface Shop {
    function isSold() external view returns (bool);
}

contract Solution {
    Shop _level;

    constructor(address _levelInstance) public {
        _level = Shop(_levelInstance);
    }

    function price() public returns (uint256) {
        if (_level.isSold()) {
            return 0;
        }
        return 100;
    }
}
