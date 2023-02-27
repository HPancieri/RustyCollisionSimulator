mod controller;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() > 2 {
        if args[1] == "-n" || args[1] == "--number-of-particles" {
            let n: i32 = args[2].parse::<i32>().unwrap();
            controller::run(n);
        } else { print_usage(); }
    } else {
        print_usage();
    }
}

fn print_usage() {
    println!("Collision Simulator Written in Rust");
    println!("\nUsage: rusty-collision-simulator [OPTION]");
    println!("\nOptions:");
    println!("  -h, --help                  Print help information");
    println!("  -n, --number-of-particles   Select the number of particles to be rendered");
}
