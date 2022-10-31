use kuber_rs::{self, app::App, ui::UI};

fn main() {
    println!("Started");
    let args = Args::parse();

    println!("namespace: {:?}", args.namespace);

    let mut app = App::new(args.namespace);
    let mut ui = UI::new(&mut app);
    ui.start().expect("should start");
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
