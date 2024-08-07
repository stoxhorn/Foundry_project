use std::collections::HashMap;
use alloy::node_bindings::Anvil;
use std::any::{Any, TypeId};
use crate::sim_interface::Sim;
use crate::sim_structs;
use crate::solidity_files;
use crate::solidity_files::Token;

type Cfunc = dyn Fn(&Sim);
pub trait Address {
    fn get_address(&self) -> String;
}

pub trait Contract {
    fn get_type(&self) -> String;
    async fn call_func(&self, func_name: String, sim: &Sim);
    fn get_state(&self) -> HashMap<String, String>;
    fn get_wallet(&self) -> Wallet;
    fn get_address(&self) -> String;
    fn get_name(&self) -> &String;

}

struct User{
    name: String,
    address: String
} impl Address for User {
    fn get_address(&self) -> String {
        self.address.clone()
    }
} impl User {
    fn get_name(&self) -> &String {
        &self.name
    }
}
pub(crate) struct Wallet{
}

struct Transaction{
    address: String,
    actions: Vec<FuncCall>
} impl Address for Transaction {
    fn get_address(&self) -> String {
        self.address.clone()
    }
}
struct FuncCall{
    args: Vec<String>,
    func: Function,
    result: String
}


struct SwapPool{
    address: String,
    name: String,
    tokens: (Token, Token),
    liquidity: HashMap<String, u64>
} impl Address for SwapPool {
    fn get_address(&self) -> String {
        self.address.clone()
    }
} impl Contract for SwapPool {
    fn get_type(&self) -> String {
        return "Token".to_string();
    }

    async fn call_func(&self, func_name: String, sim: &Sim){
        todo!()
    }

    fn get_state(&self) -> HashMap<String, String> {
        todo!()
    }

    fn get_wallet(&self) -> Wallet {
        todo!()
    }



    fn get_address(&self) -> String {
        todo!()
    }

    fn get_name(&self) -> &String {
        todo!()
    }


}

pub(crate) struct Function{
    args: HashMap<String, String>,
    action: String
}


struct Network{
    states: Vec<Block>,
    connection: Anvil,
    signers: Vec<Wallet>
}

struct Block{
    address: String,
    new_txn: Vec<Transaction>,
    addresses: Vec<Box<dyn Address>>,
    net_state: NetworkState
} impl Address for Block {
    fn get_address(&self) -> String {
        self.address.clone()
    }
}

struct NetworkState{
    data: String
}
