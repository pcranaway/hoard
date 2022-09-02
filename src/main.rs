use args::Args;
use clap::Parser;
use jsonrpsee_http_server::{HttpServerBuilder, RpcModule};

pub mod args;
pub mod state;
pub mod download;

#[tokio::main]
async fn main() {
    // parse args
    let args = Args::parse();

    let address = format!("{}:{}", args.rpc_address, args.rpc_port);

    // create RPC
    let server = HttpServerBuilder::default().build(address).await.expect("couldn't bind RPC");
    let module = RpcModule::new(());

    server.start(module).expect("couldn't start RPC").await;
}
