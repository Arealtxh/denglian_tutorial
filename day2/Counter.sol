//SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract Counter {
    int counter;

    constructor() {
        counter = 0;
    }

    function get() public view returns (int) {
        return counter;
    }

    function add(int x) public {
        counter += x;
    }
}
