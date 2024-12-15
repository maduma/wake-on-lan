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
    CreateAlias { alias: String, mac: String },
    RemoveAlias { alias: String },
    SetDefaultMac { mac: String },
    // ListAliases,
    // TODO SetDefaultAlias { alias: String },
    SetDefaultSourceIp { source_ip: String },
}

fn get_default_mac() -> String {
    match alias::get_alias("default_mac") {
        Some(default_mac) => default_mac,
        None => {
            println!("Please, set default mac!");
            std::process::exit(0);
        }
    }
}

fn get_mac(device: &Option<String>) -> String {
    match device {
        Some(mac) => mac.to_string(),
        None => get_default_mac(),
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        None => {
            let mac = get_mac(&cli.device);
            wol::wake_on_lan(&mac, cli.source_ip.as_deref());
        }
        Some(Commands::CreateAlias { alias, mac }) => alias::create_alias(&alias, &mac),
        Some(Commands::RemoveAlias { alias }) => alias::remove_alias(&alias),
        Some(Commands::SetDefaultMac { mac }) => alias::create_alias("default_mac", &mac),
        Some(Commands::SetDefaultSourceIp { source_ip }) => {
            alias::create_alias("default_source_ip", &source_ip)
        }
    }
}
