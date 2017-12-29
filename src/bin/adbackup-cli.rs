#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

extern crate adbackup;

extern crate failure;

use failure::Error;

fn main() {
    let matches = make_clap().get_matches();

    let (sub_name, subm) = matches.subcommand();
    let sub_fn = match sub_name {
        "devices" => print_devices,
        "backup" => backup,
        "apps" => apps,
        _ => unimplemented!(),
    };

    let mut verbosity = matches.occurrences_of("verbose");
    if let Some(subm) = subm {
        verbosity += subm.occurrences_of("verbose");
    }

    adbackup::setup_logging(verbosity);

    let result = sub_fn(&matches, subm);
    if let Some(error) = result.err() {
        error!("adbackup finished with error: {}", error.to_string());
    }
}

fn make_clap<'a, 'b>() -> clap::App<'a, 'b> {
    fn device_arg<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("device")
            .help("Id of device if more than one connected")
            .long("device")
            .short("d")
            .takes_value(true)
            .value_name("ID")
    };

    App::new("adbackup")
        .about("A backup tool for android using adb")
        .author(crate_authors!())
        .version(adbackup::version())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .global(true)
                .help("Increases logging verbosity each use for up to 3 times"),
        )
        .subcommand(
            SubCommand::with_name("backup")
                .display_order(1)
                .about("Start backup of device")
                .arg(device_arg())
                .arg(
                    Arg::with_name("applications")
                        .help("Include the apk's into the backup")
                        .long("applications")
                        .short("applications"),
                )
                .arg(
                    Arg::with_name("shared")
                        .help("Include the shared storage into the backup")
                        .long("shared")
                        .short("s"),
                )
                .arg(
                    Arg::with_name("system")
                        .help("Include the system apps storage into the backup")
                        .long("system")
                        .short("S"),
                )
                .arg(
                    Arg::with_name("only_specified")
                        .help("Include only the specified apps into the backup")
                        .long("specified")
                        .short("o")
                        .takes_value(true)
                        .multiple(true)
                        .value_name("APP"),
                ),
        )
        .subcommand(
            SubCommand::with_name("devices")
                .display_order(2)
                .about("List connected devices")
                .arg(device_arg())
                .help("List all android devices connected to your pc with enabled debug mode."),
        )
        .subcommand(
            SubCommand::with_name("apps")
                .display_order(3)
                .about("List all installed apps on devices")
                .arg(device_arg()),
        )
}

fn print_devices(_: &ArgMatches, _: Option<&ArgMatches>) -> Result<(), Error> {
    let devices = adbackup::get_printable_device_list()?;
    info!("{}", devices);

    Ok(())
}

fn backup(matches: &ArgMatches, subm: Option<&ArgMatches>) -> Result<(), Error> {
    let device_id = param_from_match("device", matches, subm);
    let apk = param_from_match("applications", matches, subm);
    let shared = param_from_match("shared", matches, subm);
    let system = param_from_match("system", matches, subm);
    let only_specified = param_from_match("only_specified", matches, subm);

    let backup = adbackup::backup(device_id, apk, shared, system, only_specified)?;
    info!("{}", backup);

    Ok(())
}

fn apps(matches: &ArgMatches, subm: Option<&ArgMatches>) -> Result<(), Error> {
    let device_id = param_from_match("device", matches, subm);

    let apps = adbackup::get_printable_app_list(device_id)?;
    info!("{}", apps);

    Ok(())
}

fn param_from_match(
    param: &str,
    matches: &ArgMatches,
    subm: Option<&ArgMatches>,
) -> Option<String> {
    if let Some(subm) = subm {
        if let Some(param) = subm.value_of(param) {
            return Some(param.to_string());
        }
    }

    if let Some(param) = matches.value_of(param) {
        return Some(param.to_string());
    }

    None
}
