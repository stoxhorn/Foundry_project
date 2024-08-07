use mad_simmer;
use log::{debug, error, log_enabled, info, Level};
use env_logger::Builder;
use log::LevelFilter;

#[tokio::main]
async fn main() -> Result<(), String> {


    let mut builder = Builder::new();

    builder.filter_level(LevelFilter::Info);
    builder.init();
    println!("cargo:rustc-env=RUST_LOG=TRACE");
    let result = mad_simmer::run_sim("/home/madpacket/Documents/Git/Fraquility_Prototype/code/mad_simmer/src/sim_config.json");

    println!("{}", &result.await.expect("oof"));

    Ok(())
}