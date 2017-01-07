#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

extern crate shiplift;
extern crate clap;
extern crate hyper;
extern crate libc;

use clap::{App, AppSettings, Arg, SubCommand};
use std::process;
use std::io::Write;

mod dokku;
use dokku::{Dokku, dokku_logger};

mod config;
use config::Config;

mod plugin;
use plugin::{Plugin};

mod util;

fn terminate(message: String) -> ! {
    let mut stderr = std::io::stderr();
    stderr.write(message.as_bytes()).unwrap();
    process::exit(1);
}

fn boostrap() -> (Dokku, Config) {
    match (Dokku::new(), Config::new()) {
        (Ok(dokku), Ok(config)) => (dokku, config),
        (Err(err), _) => terminate(format!("Failed to initialize dokku env: {:#?}", err)),
        (_, Err(err)) => terminate(format!("Failed to get config: {:#?}", err)),
    }
}

fn main() {

    dokku_logger::init().unwrap();

    let (dokku, config) = boostrap();
    println!("{:#?}", config);
    let plugin = Plugin::new(dokku, config);

    let matches = App::new("Dokku plugin: pg")
        .version("0.0.1")
    // We want to propogate to unreachable command so we can return dokku error code
        .setting(AppSettings::AllowExternalSubcommands)
        .subcommand(SubCommand::with_name("install"))
        .subcommand(
            SubCommand::with_name("create")
                .arg(Arg::with_name("name")
                     .help("Service name")
                     .value_name("NAME")
                     .required(true))
                .arg(Arg::with_name("image")
                     .help("Image to use")
                     .long("image")
                     .value_name("IMAGE")
                     .takes_value(true)
                     .default_value(&plugin.default_image))
                .arg(Arg::with_name("port")
                     .help("Port to expose")
                     .long("port")
                     .value_name("PORT")
                     .takes_value(true))
        )
        .get_matches();

    let res = match matches.subcommand() {
        ("install", Some(..)) => plugin.install(),
        ("create", Some(create_matches)) => plugin.create(
            create_matches.value_of("name").unwrap(),
            create_matches.value_of("image").unwrap(),
            create_matches.value_of("port")
        ),
        ("", None) => plugin.no_command(),
        _ => plugin.not_implemented(),
    };

    plugin.exit(res)
}
