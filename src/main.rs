use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Address for the RPC to listen to 
   #[clap(short, long, value_parser)]
   address: String,

    /// Port for the RPC to listen to 
   #[clap(short, long, value_parser)]
   port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
}
