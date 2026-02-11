use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct FileEncryptorArgs{
    /// The File that you want to encrypt
    pub file: String,
}