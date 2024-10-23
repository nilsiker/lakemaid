use clap::Parser;
use lakemaid::{interop, Cli};

fn main() {
    let cli = Cli::parse();
    
    unsafe {
        let sum = interop::dotnet_add(1, 2);
        println!("INTEROP SUM: {sum}");
    }

    if let Err(e) = lakemaid::run(&cli.command) {
        eprintln!("lakemaid: {e}")
    }
}
