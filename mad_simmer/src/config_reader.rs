
use std::process::Command;
use std::thread;
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


#[derive(serde::Deserialize, Debug, Clone)]
pub struct ActionConf {
    new_block: bool,
    signer_address: String,
    arguments: Vec<String>,
    function: String,
    receiver_address: String
}
impl ActionConf {
    fn new(new_block: bool,
           signer_address: String,
           arguments: Vec<String>,
           function: String,
           receiver_address: String) -> ActionConf {
        ActionConf {new_block, signer_address, arguments, function, receiver_address}}
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct AnvilConfig{
    pub port: u32,
    pub block_time: u64
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct UserConf {
    name: String,
    address: String
}
impl UserConf {
    fn new(name: String, address: String) -> UserConf {
        UserConf {name, address}
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ContractConf {
    pub name: String,
    pub contract_name: String,
    pub deploy_arguments: Vec<String>
}
impl ContractConf {
    fn new(name: String, contract_name: String, deploy_arguments: Vec<String>) -> ContractConf {
        ContractConf {name, contract_name, deploy_arguments}
    }

    //pub fn get_contract_code(&self) -> std::io::Result<String>{
//        fs::read_to_string(&self.path)
//    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct SimConf {
    users: Vec<UserConf>,
    pub contracts: Vec<ContractConf>,
    pub anvil_conf: AnvilConfig,
    actions: Vec<ActionConf>
} impl SimConf {
    fn new(users: Vec<UserConf>, contracts: Vec<ContractConf>, anvil_conf: AnvilConfig, actions: Vec<ActionConf>) -> SimConf {
        SimConf {users, contracts, anvil_conf, actions}}}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn parse_conf_object<T>(input: &Value) -> T
where T: DeserializeOwned {
    serde_json::from_value((*input).clone()).unwrap()
}

fn parse_conf_list<T>(input: Value, config_list: &str) -> Result<Vec<T>, Error>
where T: DeserializeOwned {

    let mut new_objects: Vec<T> = Vec::new();
    let x = input[config_list].as_array();
    for object in x.unwrap().iter() {
        let new_object: T = parse_conf_object(object);
        new_objects.push(new_object);
    };
    Ok(new_objects)
}

pub fn read_config(path: &str) -> Result<SimConf> {

    info!("Reading config at path: {path}");
    let data = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    info!("Converting to serde_json::Value");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Failed to create JSON");
    info!("Converted successfully");


    info!("Parsing Anvil Config");
    let anvil_conf: AnvilConfig = parse_conf_object(&res["anvil_config"]);
    info!("success");

    info!("Parsing Users");
    let result: Result<Vec<UserConf>, Error> = parse_conf_list(res.clone(), "users");
    let users = match result {
        Ok(result) => result,
        Err(error) => {
            error!("failed parsing users - panicking");
            panic!("failed parsing users")
        },
    };
    info!("success");

    info!("Parsing contracts");
    let result: Result<Vec<ContractConf>, Error> = parse_conf_list(res.clone(), "contracts");
    let contracts = match result {
        Ok(result) => result,
        Err(error) => {
            error!("failed parsing Contracts - panicking");
            panic!("failed parsing Contracts")
        },
    };
    info!("success");

    info!("Parsing Actions");
    let result: Result<Vec<ActionConf>, Error> = parse_conf_list(res.clone(), "actions");
    let actions = match result {
        Ok(result) => result,
        Err(error) => {
            error!("failed parsing Actions - panicking");
            panic!("failed parsing Actions")
        },
    };
    info!("success");


    Ok(SimConf {users, contracts, anvil_conf, actions})
}
