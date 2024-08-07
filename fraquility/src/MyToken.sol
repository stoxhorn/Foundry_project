//SPDX-License-Identifier: MIT
pragma solidity 0.8.26;
 
import "@solmate/tokens/ERC20.sol";
 
contract MyToken is ERC20 {
   
   uint256 public myVar;

   constructor(string memory _name, string memory _ticker) ERC20(_name, _ticker, 8) {
	_mint(msg.sender, 10000000000000);
   }

   function setVar() public{
    myVar = 10;
   }

   function incVar() public{
    myVar = myVar +1;
   }
 
   function mint(uint _qty) public {
       _mint(msg.sender, _qty);
   }
 
   function burn(uint _qty) public {
       _burn(msg.sender, _qty);
   }
 
}