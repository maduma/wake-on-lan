use clap::{Parser, Subcommand};
use wake_on_lan::wol::wake_on_lan;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    device: Option<String>,

    #[arg(short, long)]
    source_ip: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    CreateAlias { mac: String, alias: String },
    RemoveAlias { alias: String },
    SetDefaultAlias { alias: String },
    SetDefaultSourceIp { source_ip: String },
}

fn main() {
    let cli = Cli::parse();
    let default_mac = "2c:f0:5d:e1:9e:d6";
    if cli.command.is_none() {
        wake_on_lan(default_mac, cli.source_ip.as_deref());
    }
}
