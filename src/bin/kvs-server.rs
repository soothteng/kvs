use kvs::{KvStore, Result};
use log::{error, info, LevelFilter};
use std::env;
use structopt::clap::{App, AppSettings, Arg};

fn main() -> Result<()> {
    let matches = App::new("kvs-server")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("A key-value store server")
        .setting(AppSettings::DisableHelpSubcommand)
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .takes_value(true)
                .value_name("IP-PORT")
                .default_value("127.0.0.1:4000")
                .help("the server address"),
        )
        .arg(
            Arg::with_name("engine")
                .long("engine")
                .takes_value(true)
                .value_name("ENGINE-NAME")
                .possible_values(&["kvs", "sled"])
                .help("the server address"),
        )
        .get_matches();

    let addr = matches.value_of("addr").expect("wtf");
    let engine = matches.value_of("engine").expect("wtf");

    info!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    info!("Storage engine: {}", engine);
    info!("Listening on {}", addr);
    Ok(())
}
