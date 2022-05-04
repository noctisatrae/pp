#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    #[clap(short, long)]
    path: String,
}



fn main() {

    let args = Args::parse();

    rocket::ignite()
        .mount("/files", StaticFiles::from(args.path))
        .launch();
}