use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Fast, secure file transfer over QUIC, built for bad networks.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Send a file and print the code the receiver needs.
    Send {
        /// File to send.
        path: PathBuf,
    },
    /// Receive a file using the code given by the sender.
    Receive {
        /// Code shown by the sender, e.g. 7-tiger-ocean.
        code: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Send { path } => {
            println!("send {} — not implemented yet (phase 1)", path.display());
        }
        Command::Receive { code } => {
            println!("receive {code} — not implemented yet (phase 1)");
        }
    }
    println!("protocol version: {}", quicdrop_core::PROTOCOL_VERSION);
}
