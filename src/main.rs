use std::env;
use weekend_dns::{TYPE_A, resolve};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <domain>", args[0]);
        std::process::exit(1);
    }

    let domain = &args[1];

    println!("Resolving: {}", domain);
    let ip = resolve(domain, TYPE_A);
    println!("Final IP for {}: {}", domain, ip);
}
