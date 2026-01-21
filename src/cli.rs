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
        #[arg(short,long)]
        input: String,
        
        #[arg(short,long)]
        output: String,

        #[arg(short,long)]
        password: String,
    },

    Decrypt{
        #[arg(short,long)]
        input: String,
        
        #[arg(short,long)]
        output: String,

        #[arg(short,long)]
        password: String,
    }
}
