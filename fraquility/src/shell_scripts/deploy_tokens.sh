#!/bin/bash
forge create --rpc-url http://127.0.0.1:5000 src/MyToken.sol:MyToken --constructor-args "Bitcoin" "BTC" --unlocked --from 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
forge create --rpc-url http://127.0.0.1:5000 src/MyToken.sol:MyToken --constructor-args "Ether" "ETH" --unlocked --from 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
forge create --rpc-url http://127.0.0.1:5000 src/MyToken.sol:MyToken --constructor-args "USD-Coin" "USDC" --unlocked --from 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266