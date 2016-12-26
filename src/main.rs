#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate shiplift;
extern crate clap;
extern crate hyper;

use clap::{App, AppSettings, SubCommand};
use std::process;
use std::io::Write;

mod dokku;
use dokku::Dokku;

mod config;
use config::Config;

mod plugin;
use plugin::{Plugin, Command};

fn terminate(message: String) -> ! {
    let mut stderr = std::io::stderr();
    stderr.write(message.as_bytes());
    process::exit(1);
}

fn boostrap() -> (Dokku, Config) {
    match (Dokku::new(), Config::new()) {
        (Ok(dokku), Ok(config)) => (dokku, config),
        (Err(err), _) => terminate(format!("Failed to initialize dokku env: {:#?}", err)),
        (_, Err(err)) => terminate(format!("Failed to get config: {:#?}", err))
    }
}

fn main() {
    let (dokku, config) = boostrap();
    println!("{:#?}", config);
    let plugin = Plugin::new(dokku, config);

    let matches = App::new("Dokku plugin: pg")
        .version("0.0.1")
    // We want to propogate to unreachable command so we can return dokku error code
        .setting(AppSettings::AllowExternalSubcommands)
        .subcommand(SubCommand::with_name("install"))
        .get_matches();

    match matches.subcommand() {
        ("install", Some(install_matches)) => plugin.run(Command::Install),
        ("", None) => plugin.run(Command::Empty),
        _ => plugin.run(Command::NotImplemented),
    }
}
