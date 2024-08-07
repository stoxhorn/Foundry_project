#!/bin/bash
anvil -p 5000 --block-time 5&
#forge create --rpc-url http://127.0.0.1:8545 src/MyToken.sol:MyToken --constructor-args "Bitcoin" "BTC" --unlocked --from 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266