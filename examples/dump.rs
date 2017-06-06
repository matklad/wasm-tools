extern crate wasmparser;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::str;
use std::env;
use wasmparser::Parser;
use wasmparser::ParserState;

fn get_name(bytes: &[u8]) -> &str {
    str::from_utf8(bytes).ok().unwrap()
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: {} in.wasm", args[0]);
        return;
    }

    let ref buf: Vec<u8> = read_wasm(&args[1]).unwrap();
    let mut parser = Parser::new(buf);
    loop {
        let state = parser.read();
        match *state {
            ParserState::ExportSectionEntry {
                field,
                ref kind,
                index,
            } => {
                println!("ExportSectionEntry {{ field: \"{}\", kind: {:?}, index: {} }}",
                         get_name(field),
                         kind,
                         index);
            }
            ParserState::ImportSectionEntry {
                module,
                field,
                ref ty,
            } => {
                println!("ImportSectionEntry {{ module: \"{}\", field: \"{}\", ty: {:?} }}",
                         get_name(module),
                         get_name(field),
                         ty);
            }
            ParserState::EndWasm => break,
            ParserState::Error(msg) => panic!("Error: {}", msg),
            _ => println!("{:?}", state),
        }
    }
}

fn read_wasm(file: &str) -> io::Result<Vec<u8>> {
    let mut data = Vec::new();
    let mut f = File::open(file)?;
    f.read_to_end(&mut data)?;
    Ok(data)
}
