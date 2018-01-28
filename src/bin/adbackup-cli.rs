#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

extern crate adbackup;

extern crate failure;

use failure::{Error, err_msg};

fn main() {
    let matches = make_clap().get_matches();

    let (sub_name, subm) = matches.subcommand();

    let mut verbosity = matches.occurrences_of("verbose");
    if let Some(subm) = subm {
        verbosity += subm.occurrences_of("verbose");
    }

    adbackup::setup_logging(verbosity);

    let result = match sub_name {
        "backup" => backup(&matches, subm),
        "restore" => restore(&matches, subm),
        "devices" => print_devices(),
        "push" => push(&matches, subm),
        "pull" => pull(&matches, subm),
        "apps" => apps(&matches, subm),
        _ => unimplemented!(),
    };

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
                        .help("Include only the specified app into the backup")
                        .long("specified")
                        .short("o")
                        .takes_value(true)
                        .multiple(true)
                        .value_name("APP"),
                ),
        )
        .subcommand(
            SubCommand::with_name("restore")
                .display_order(2)
                .about("Restore android backup")
                .arg(device_arg()),
        )
        .subcommand(
            SubCommand::with_name("devices")
                .display_order(3)
                .about("List connected devices")
                .arg(device_arg())
                .help("List all android devices connected to your pc with enabled debug mode."),
        )
        .subcommand(
            SubCommand::with_name("pull")
                .display_order(4)
                .about("Pull file/folder from android into current folder of your pc")
                .arg(device_arg())
                .arg(
                    Arg::with_name("source")
                        .help("Source file/folder on the android device")
                        .required(true),
                )
        )
        .subcommand(
            SubCommand::with_name("push")
                .display_order(5)
                .about("Push file/folder from the pc to a connected android device")
                .arg(device_arg())
                .arg(
                    Arg::with_name("source")
                        .help("Source file/folder on your pc which should be pushed to android")
                        .required(true),
                )
                .arg(
                    Arg::with_name("target")
                        .help("Target folder on the android device")
                        .required(true),
                )
        )
        .subcommand(
            SubCommand::with_name("apps")
                .display_order(6)
                .about("List all installed apps on devices")
                .arg(device_arg()),
        )
}

fn print_devices() -> Result<(), Error> {
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


fn pull(matches: &ArgMatches, subm: Option<&ArgMatches>) -> Result<(), Error> {
    let device_id = param_from_match("device", matches, subm);
    let target = param_from_match("source", matches, subm);

    if let Some(target) = target {
        let result = adbackup::pull(device_id, target)?;
        info!("{}", result);

        return Ok(());
    }

    Err(err_msg("Target not specified")) // is not possible from cmd because it is required
}


fn push(matches: &ArgMatches, subm: Option<&ArgMatches>) -> Result<(), Error> {
    let device_id = param_from_match("device", matches, subm);
    let source = param_from_match("source", matches, subm);
    let target = param_from_match("target", matches, subm);

    if let Some(source) = source {
        if let Some(target) = target {
            let result = adbackup::push(device_id, source, target)?;
            info!("{}", result);

            return Ok(());
        }
    }

    Err(err_msg("Source or target not specified")) // is not possible from cmd because it is required
}

fn restore(matches: &ArgMatches, subm: Option<&ArgMatches>) -> Result<(), Error> {
    let device_id = param_from_match("device", matches, subm);

    let result = adbackup::restore(device_id)?;
    info!("{}", result);

    return Ok(());
}

fn param_from_match<'a>(
    param: &'a str,
    matches: &'a ArgMatches,
    subm: Option<&'a ArgMatches>,
) -> Option<&'a str> {
    if let Some(subm) = subm {
        if let Some(param) = subm.value_of(param) {
            return Some(param);
        }
    }

    if let Some(param) = matches.value_of(param) {
        return Some(param);
    }

    None
}
