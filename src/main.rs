use clap::{Parser, Subcommand};
use wake_on_lan::{alias, wol};

#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Can be a mac address (eg 2c:f0:5d:e1:9e:d6) or an alias
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
        match cli.device {
            Some(mac) => wol::wake_on_lan(&mac, cli.source_ip.as_deref()),
            _ => match alias::get_alias("default_alias") {
                Some(mac) => wol::wake_on_lan(&mac, cli.source_ip.as_deref()),
                _ => wol::wake_on_lan(default_mac, cli.source_ip.as_deref()),
            },
        };
    }
}
