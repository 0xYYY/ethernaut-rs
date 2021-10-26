// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "../@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract Solution is ERC20 {
    constructor(
        string memory name,
        string memory symbol,
        uint256 initialSupply
    ) public ERC20(name, symbol) {
        _mint(msg.sender, initialSupply);
    }
}
