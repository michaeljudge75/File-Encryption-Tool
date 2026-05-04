use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "File Encryption Tool", version = "1.0", author = "Michael Judge", about = "A simple tool used to encrypt and decrypt files via the terminal")]
struct Cli{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands{
    Encrypt{
        #[arg(short, long, help = "Path to the file you would like to encrypt")]
        input: String,
        
        #[arg(short, long, help = "Path where you would like the encrypted file to be saved")]
        output: String,
    },

    Decrypt{
        #[arg(short,long, help = "Path to the file you would like to encrypt")]
        input: String,
        
        #[arg(short,long, help = "Path where you would like the encrypted file to be saved")]
        output: String,
    }
}
