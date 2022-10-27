use kuber_rs::{self, errors::Error, ui::UI};

fn main() {
    println!("Started");
    let args = Args::parse();

    println!("namespace: {:?}", args.namespace);

    let pods = match kuber_rs::load_all_pods(&args.namespace) {
        Ok(pods) => pods,
        Err(err) => match err {
            Error::KubecltNotFound(io_err) => {
                panic!("Failed to execute kubectl command. Error: {io_err}")
            }
            Error::ParseOutputError => {
                panic!("should not fail here, because parse error should be handled in lib")
            }
        },
    };

    let mut ui = UI::init().expect("should be able to init UI");
    ui.start(&args.namespace, pods).expect("should start");
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
