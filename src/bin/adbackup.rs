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

fn main() {
    let matches = make_clap().get_matches();

    let (sub_name, subm) = matches.subcommand();
    let sub_fn = match sub_name {
        "devices" => print_devices,
        _ => unimplemented!(),
    };

    let verbosity = matches.occurrences_of("verbose")
        + subm.unwrap().occurrences_of("verbose");
    logging::setup_logging(verbosity)
        .expect("failed to initialize logging.");

    sub_fn();
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

fn print_devices() {
    if let Some(devices) = Device::list_devices() {
        info!("Found the following devices:");
        devices.into_iter().for_each(|device|
            info!("Id: '{}', Name: '{}'", device.id, device.name))
    } else {
        warn!("No device found. Make sure that you connect at least one device with enabled \
        debug options.");
    }
}
