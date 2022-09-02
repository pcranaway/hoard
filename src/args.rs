use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
   /// Address for the RPC to listen to 
   #[clap(short = 'h', long, value_parser, default_value = "0.0.0.0")]
   pub rpc_address: String,

    /// Port for the RPC to listen to 
   #[clap(short = 'p', long, value_parser, default_value = "3820")]
   pub rpc_port: u16,
}


