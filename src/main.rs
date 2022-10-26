use kuber_rs;
mod ui_example;
mod ui;

fn main() {
    println!("Started");
    let args = Args::parse();

    println!("namespace: {:?}", args.namespace);

    // kuber_rs::load_all_pods(&args.namespace);
    // kuber_rs::load_namespaces();
    // ui_example::main();
    
}

use clap::Parser;

/// CLI UI for kubernetes cluster
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Namespace to be used
    #[arg(short, long, default_value = "default")]
    namespace: String,
}
