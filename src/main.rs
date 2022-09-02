use clap::Parser;
use jsonrpsee_http_server::{HttpServerBuilder, RpcModule};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Address for the RPC to listen to 
   #[clap(short = 'h', long, value_parser, default_value = "0.0.0.0")]
   rpc_address: String,

    /// Port for the RPC to listen to 
   #[clap(short = 'p', long, value_parser, default_value = "3820")]
   rpc_port: u16,
}

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
