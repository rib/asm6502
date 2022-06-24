use clap::{ArgEnum, Parser};
use anyhow::Result;
use std::io::{self, Write};

use asm6502::assemble;
use asm6502::AssembleResult;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    //#[clap(required=true)]
    filename: String,

    output: Option<String>
}


fn main() -> anyhow::Result<()> {

    let args = Args::parse();
    //println!("args = {args:?}");

    let path = std::path::Path::new(&args.filename);
    let asm = std::fs::read_to_string(path)?;
    //println!("Assembling:\n{}\n", &asm);
    let mut buf = Vec::<u8>::new();
    if let Err(msg) = assemble(asm.as_bytes(), &mut buf) {
        panic!("Failed to assemble: {}", msg);
    }

    //println!("Assembly:\n{:?}", buf);

    match args.output {
        Some(output) => {
            let path = std::path::Path::new(&output);
            std::fs::write(path, &buf)?;
        }
        None => {
            //println!("writting to stdout...");
            io::stdout().write_all(&buf)?;
        }
    }

    Ok(())
}
