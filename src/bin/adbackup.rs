#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;

use clap::{Arg, App, AppSettings, SubCommand};

extern crate adbackup;

use adbackup::devices::Device;
use adbackup::logging;

extern crate failure;

use failure::Error;

fn main() {
    let matches = make_clap().get_matches();

    let (sub_name, subm) = matches.subcommand();
    let sub_fn = match sub_name {
        "devices" => print_devices,
        _ => unimplemented!(),
    };

    let mut verbosity = matches.occurrences_of("verbose");
    if let Some(subm) = subm {
        verbosity += subm.occurrences_of("verbose");
    }

    logging::setup_logging(verbosity)
        .expect("failed to initialize logging.");

    let result = sub_fn();
    if let Some(error) = result.err() {
        error!("adbackup finished with error: {}", error.to_string());
    }
}

fn make_clap<'a, 'b>() -> clap::App<'a, 'b> {
    App::new("adbackup")
        .about("A backup tool for android using adb")
        .author(crate_authors!())
        .version(adbackup::version())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .global(true)
            .help("Increases logging verbosity each use for up to 3 times"))
        .subcommand(SubCommand::with_name("devices")
            .about("List connected devices")
            .help("List all android devices connected to your pc with enabled debug mode."))
}

fn print_devices() -> Result<(), Error> {
    let devices = Device::list_devices()?;

    if devices.len() > 0 {
        info!("Found the following devices:");
        devices.into_iter().for_each(|device|
            info!("Id: '{}', Name: '{}'", device.id, device.name))
    } else {
        warn!("No device found. Make sure that you connect at least one device with enabled \
        debug options.");
    }

    Ok(())
}
