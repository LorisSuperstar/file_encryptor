mod args;
use std::fs;
use args::FileEncryptorArgs;
use clap::Parser;

fn main() -> std::io::Result<()> {
    let arg = FileEncryptorArgs::parse();

    let content = fs::read_to_string(&arg.file)?;

    print!("{}", content);

    Ok(())
}
