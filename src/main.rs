#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate duct;
extern crate chrono;
extern crate clap;

use chrono::prelude::*;
use clap::{App, Arg};

use duct::cmd;

fn main() {

    let matches = App::new("MyApp")
        .version("1.0")
        .author("david")
        .about("git cli")
        .arg(Arg::with_name("input").help("function to exec").index(1))
        .get_matches();

    if let Some(o) = matches.value_of("input") {
        println!("INPUT: {}", o);
        match o {
            "f" => first(),
            _ => println!("0"),
        }
    }
}
fn first () {
   println!("hi");
    let command = format!("cd /home/ice/d/first;pwd");
    let args = &["-c", command.as_str()];
    let stdout = cmd("sh", args).read().unwrap();
    println!("{}", stdout);
    println!("{}", dotenv!("HOME"));
}
