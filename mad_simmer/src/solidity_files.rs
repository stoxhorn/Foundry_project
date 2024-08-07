use futures::executor::block_on;
use std::any::Any;
use std::collections::HashMap;
use alloy::contract::Error;
use alloy::network::{Ethereum, EthereumWallet};
use alloy::primitives::{Address, TxHash};
use alloy::providers::fillers::{FillProvider, JoinFill, RecommendedFiller, WalletFiller};
use alloy::providers::{ProviderBuilder, ReqwestProvider};
use alloy::signers::local::yubihsm::Client;
use alloy::sol;
use alloy::transports::http::Http;
use crate::sim_interface::Sim;
use crate::sim_structs::Contract;
use crate::solidity_files::Counter::CounterInstance;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    MyToken,
    "/home/madpacket/Documents/Git/Fraquility_Prototype/code/fraquility/out/MyToken.sol/MyToken.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Counter,
    "/home/madpacket/Documents/Git/Fraquility_Prototype/code/fraquility/out/Counter.sol/Counter.json"
);


pub struct MyContract {
    pub name: String,
    pub address: alloy::primitives::Address,
    pub c_type: String
} impl MyContract {
    pub async fn new(sim: &Sim, name: String, c_type: &str, args: Vec<String>) -> MyContract {

        let address_fut = MyContract::deploy_contract(sim, c_type, args);
        let address = address_fut.await;
        MyContract{name, address, c_type: c_type.to_string() }
    }

    async fn deploy_contract(sim: &Sim, c_type: &str, args: Vec<String>) -> Address{
        let provider=
            ProviderBuilder::new().with_recommended_fillers().wallet(&sim.def_wallet).on_http(sim.get_url());

        let address = match c_type {
            "Token" => Ok(MyToken::deploy(provider, args[0].clone(), args[1].clone()).await.expect("deploying token").address().clone()),
            "Counter" => Ok(Counter::deploy(provider).await.expect("Deploying counter").address().clone()),
            _ => {Err(())}
        };
        address.expect("unwrapping deployed address").clone()
    }

    pub(crate) async fn call_func(&self, func_name: String, sim: &Sim) -> Result<TxHash, Error>{

        let future = match &self.c_type[..] {
            "Token" => Ok(self.call_token_func(func_name, sim).await),
            "Counter" => Ok(self.call_counter_func(func_name, sim).await),
            _ => {Err(()) }
        };
        future.unwrap()
    }
    async fn call_counter_func(&self, func_name: String, sim: &Sim) -> Result<TxHash, Error> {

        let provider=
            ProviderBuilder::new().with_recommended_fillers().wallet(&sim.def_wallet).on_http(sim.get_url());
        let contract = Counter::new(self.address.clone(), &provider);

        let future = match func_name.as_str(){
            "increment" => Ok(contract.increment().send().await.expect("called {func_name.as_str()}").watch().await.expect("")),
            _ => {Err(())}
        };

        let txn_hash = future.unwrap();
        Ok(txn_hash)
    }

    async fn call_token_func(&self, func_name: String, sim: &Sim) -> Result<TxHash, Error> {
        let provider=
            ProviderBuilder::new().with_recommended_fillers().wallet(&sim.def_wallet).on_http(sim.get_url());
        let contract = MyToken::new(self.address.clone(), &provider);

        let future = match func_name.as_str(){
            "incVar" => Ok(contract.incVar().send().await.expect("called {func_name.as_str()}").watch().await.expect("")),
            "setVar" => Ok(contract.setVar().send().await.expect("called {func_name.as_str()}").watch().await.expect("")),
            _ => {Err(()) }
        };

        let txn_hash = future.unwrap();
        Ok(txn_hash)
    }

    pub async fn make_token_call(&self, func_name: String, sim: &Sim) -> Result<String, Error> {
        let provider=
            ProviderBuilder::new().with_recommended_fillers().wallet(&sim.def_wallet).on_http(sim.get_url());
        let contract = MyToken::new(self.address.clone(), &provider);

        let future = match func_name.as_str(){
            "myVar" => Ok(contract.myVar().call().await?._0.to_string()),
            _ => {Err(()) }
        };

        let txn_hash = future.unwrap();
        Ok(txn_hash)
    }

}

