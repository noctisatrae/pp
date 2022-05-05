#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;
use rocket::config::{Config, Environment};
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    #[clap(long)]
    path: String,

    #[clap(long)]
    port: u16
}

fn main() {

    let args = Args::parse();

    let config = Config::build(Environment::Staging)
        .address("127.0.0.1")
        .port(args.port)
        .unwrap();
 
    rocket::custom(config)
        .mount("/", StaticFiles::from(args.path))
        .launch();
}