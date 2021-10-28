// SPDX-License-Identifier: MIT
pragma solidity <0.7.0;

contract Solution {
    function sd() public {
        selfdestruct(msg.sender);
    }
}
