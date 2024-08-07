#![allow(unused)]
mod config_reader;
mod sim_interface;
mod sim_structs;
mod solidity_files;
mod actions;

use config_reader::read_config;
use eyre::Result;
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
use std::env;
use std::fmt::Error;
use std::fs;
use futures::executor::block_on;
use serde::Deserializer;
use serde_json::Value;
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use log::{debug, error, log_enabled, info, Level};
use crate::config_reader::SimConf;

pub async fn run_sim(path: &str) -> Result<&str, > {
    info!("starting simulation");

    let result:Result<SimConf> = read_config(path);

    let sim_conf = match result {
        Ok(result) => {info!("config parsed succesfully"); result},
        Err(error) => {
            error!("failed parsing config - panicking");
            panic!("failed parsing config")
        },
    };

    info!("starting sim");
    sim_interface::start_sim(&sim_conf).await.expect("fucked");
    Ok("Sim run successfully")
}


/*
async fn old_code() -> Result<()> {
    let anvil = Anvil::new().try_spawn()?;

    // Set up signer from the first default Anvil account (Alice).
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let wallet = EthereumWallet::from(signer);

    // Create a provider with the wallet.
    let rpc_url = anvil.endpoint().parse()?;
    let provider =
        ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(rpc_url);

    println!("Anvil running at `{}`", anvil.endpoint());

    // Deploy the `Counter` contract.
    //let contract = MyToken::deploy(&provider).await?;

    //let contract = MyToken::new("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse()?, provider);
    
    let tokenName: String = String::from("Bitcoin");
    let ticker: String = String::from("BTC");

    let contract = MyToken::deploy(&provider, tokenName, ticker).await?;
    //let contract = Counter::deploy(&provider).await?;

    println!("Deployed contract at address: {}", contract.address());

    let builder = contract.setVar();
    let tx_hash = builder.send().await?.watch().await?;

    println!("Set number: {tx_hash}");

    let builder = contract.incVar();
    let tx_hash = builder.send().await?.watch().await?;

    println!("Incremented number: {tx_hash}");

    // Retrieve the number
    let builder = contract.myVar();
    let number = builder.call().await?._0.to_string();

    println!("Retrieved number: {number}");

    //println!("result `{}`", res.ok());
    Ok(())
}*/