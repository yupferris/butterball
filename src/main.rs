#[macro_use]
extern crate nom;

mod ast;
mod parser;

use std::env;
use std::io;
use std::io::Read;
use std::error::Error;
use std::fs::File;

fn main() {
    if let Err(error) = compile() {
        panic!("Error: {}", error);
    }
}

fn compile() -> Result<(), String> {
    let file_name = try!(get_file_name());

    println!("Reading file: {}", file_name);
    let file = try!(read_file(&file_name));

    println!("Parsing...");
    let ast = try!(parser::parse(&file));

    println!("AST: {:?}", ast);

    Ok(())
}

fn get_file_name() -> Result<String, String> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() == 2 {
        Ok(args[1].clone())
    } else {
        Err(String::from("Invalid command line arguments"))
    }
}

fn read_file(file_name: &String) -> Result<String, String> {
    read_file_impl(file_name).map_err(|x| x.description().to_string())
}

fn read_file_impl(file_name: &String) -> io::Result<String> {
    let mut file = try!(File::open(file_name));
    let mut ret = String::new();
    try!(file.read_to_string(&mut ret));
    Ok(ret)
}
