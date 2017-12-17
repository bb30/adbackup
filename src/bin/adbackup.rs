#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#![recursion_limit = "1024"]

#[macro_use]
extern crate clap;

use clap::{App, AppSettings};

extern crate adbackup;

fn main() {
    let _ = make_clap().get_matches();
}

fn make_clap<'a, 'b>() -> clap::App<'a, 'b> {
    App::new("adbackup")
        .about("A backup tool for android using adb")
        .author(crate_authors!())
        .version(adbackup::version())
        .setting(AppSettings::SubcommandRequiredElseHelp)
}
