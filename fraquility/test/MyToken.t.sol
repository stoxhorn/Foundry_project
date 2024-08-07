// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {MyToken} from "../src/MyToken.sol";

contract MyTokenTest is Test {
    MyToken public token;

    function setUp() public {
        token = new MyToken("Bitcoin", "BTC");
    }

    function test_Decimals() public {
        token.setVar();
        assertEq(token.myVar(), 1);
    }

}
