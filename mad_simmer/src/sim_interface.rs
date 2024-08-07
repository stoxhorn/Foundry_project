//use alloy_primitives::{address, b256, Address};
use std::any::Any;
use std::collections::HashMap;
use std::env::current_exe;
use std::fmt::Error;
use std::ops::Deref;
use std::thread;
use alloy::{
    network::EthereumWallet,
    node_bindings::Anvil,
    primitives::U256,
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
    node_bindings::AnvilInstance,
    dyn_abi::DynSolValue
};
use alloy::network::Ethereum;
use alloy::primitives::{address, Address, B256, b256};
use alloy::providers::fillers::{ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, RecommendedFiller, WalletFiller};
use alloy::providers::{Provider, ReqwestProvider, RootProvider};
use alloy::rpc::client::BuiltInConnectionString::Http;
use alloy::signers::local::yubihsm::Client;
//use alloy::sol_types::sol_data::String;
use alloy::transports::http::reqwest::Url;
use log::{debug, error, log_enabled, info, Level};
use eyre::Result;
use bytes::Bytes;
use serde_json::to_string;
use tokio::process::Command;
use crate::actions;
use crate::config_reader::{AnvilConfig, ContractConf, SimConf};
use crate::sim_structs::Contract;
use crate::solidity_files::{Counter, MyContract, MyCounter, MyToken, Token};
use crate::solidity_files::Counter::CounterInstance;
use crate::solidity_files::MyToken::MyTokenInstance;

pub struct Sim{
    anvil: AnvilInstance,
    def_signer: PrivateKeySigner,
    pub(crate) def_wallet: EthereumWallet,
    rpc_url: Url,

} impl Sim{
    pub fn new(anvil_config: &AnvilConfig) -> Sim{
        let x: u16 = u16::try_from(anvil_config.port).unwrap();
        info!("spawning AnvilInstance");
        let new_anvil = Anvil::new().arg("--disable-console-log")
            .port(x)
            .block_time(anvil_config.block_time)
            .try_spawn().expect("error deploying anvil");

        let signer: PrivateKeySigner = (&new_anvil).keys()[0].clone().into();
        // This was needed to have a signer object i had ownership of
        let signer_bytes: B256 = signer.to_bytes();
        let def_signer = PrivateKeySigner::from_bytes(&signer_bytes).unwrap();
        let def_wallet:EthereumWallet = EthereumWallet::new(signer);
        let rpc_url = (&new_anvil).endpoint().parse().unwrap();
        info!("creating sim struct");
        Sim{
            anvil: new_anvil,
            def_signer,
            def_wallet,
            rpc_url
        }

        }

//let provider =
//         ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(rpc_url);
    pub fn get_url(&self) -> Url {
        (&self.anvil).endpoint().parse().unwrap()
    }

}

struct Contracts {
    addresses: HashMap<String, MyContract>,
}
impl Contracts {
    pub fn new () -> Contracts{
        Contracts{addresses: HashMap::new()}
    }

    pub fn add_contract(&mut self, name: String, mc: MyContract){
        self.addresses.insert(name,mc);
    }

    pub async fn get_contract(&self, name: String) -> Option<&MyContract> {
        self.addresses.get(&name.to_string())
        //let address = Address::parse_checksummed(&name[..], None).expect("valid checksum");

    }
}

pub async fn start_sim(conf: &SimConf) -> Result<()>{

    let mut current_sim = Sim::new(&conf.anvil_conf);
    let mut sc = Contracts::new();

    info!("deploying MyToken(Bitcoin btc)");
    //let bitcoin = Token::new(&current_sim, "Bitcoin".to_string(), "BTC".to_string()).await.expect("create Bitcoin");
    let mc = MyContract::new(&current_sim, "Bitcoin".to_string(), "Token", vec!["Bitcoin".to_string(), "BTC".to_string()]).await;
    sc.add_contract("Bitcoin".to_string(), mc);


    info!("deploying MyToken(Bitcoin btc)");
    //let ether = Token::new(&current_sim, "Ether".to_string(), "ETH".to_string()).await.expect("create Bitcoin");
    let ec = MyContract::new(&current_sim, "Ether".to_string(), "Token", vec!["Ether".to_string(), "ETH".to_string()]).await;

    info!("incrementing ether");
    ec.call_func("setVar".to_string(), &current_sim).await.expect("setting var");

    let mvar = ec.make_token_call("myVar".to_string(), &current_sim).await.expect("getting myVar");
    ec.call_func("incVar".to_string(), &current_sim).await.expect("setting var");
    info!("{}", mvar);

    info!("adding ether to HashMap");
    sc.add_contract("Ether".to_string(), ec);

    info!("getting ether from hashmap");
    let e_t = sc.get_contract("Ether".to_string());

    let new_ether_t= e_t.await.expect("getting ether").make_token_call("myVar".to_string(), &current_sim).await.expect("getting myVar");


    info!("{}", new_ether_t);


    //let provider =
      //  ProviderBuilder::new().with_recommended_fillers().wallet(&current_sim.def_wallet).on_http(current_sim.get_url());
    //let ether = MyToken::deploy(provider, "Ether".to_string(), "ETH".to_string()).await?;

    //let x :Contracts<Http<Client>, FillProvider<JoinFill<RecommendedFiller, WalletFiller<&EthereumWallet>>, ReqwestProvider, Http<Client>, Ethereum>> =

    Ok(())
/*
    // Set up signer from the first default Anvil account (Alice).
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let wallet = EthereumWallet::from(signer);

    // Create a provider with the wallet.
    let rpc_url = anvil.endpoint().parse()?;
    let provider =
        ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(rpc_url);

    println!("Anvil running at `{}`", anvil.endpoint());
*/
}


