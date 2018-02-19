use std::process::Command;

#[test]
fn test_help() {
    let empty_string = "";
    let mocked_message = "adbackup 0.1.0\nJulian Raufelder <julian.raufelder@htwg-konstanz.de>\
        :Jonas Reinwald <jonas.reinwald@htwg-konstanz.de>\nA backup tool for android using adb\n\n\
        USAGE:\n adbackup-cli [FLAGS] [SUBCOMMAND]\n\n\
        FLAGS:\n\
         -h, --help Prints help information\n\
         -V, --version Prints version information\n\
         -v  Increases logging verbosity each use for up to 3 times\n\n\
        SUBCOMMANDS:\n\
         backup Start backup of device\n\
         restore Restore android backup\n\
         devices List connected devices\n\
         pull Pull file/folder from android into current folder of your pc\n\
         push Push file/folder from the pc to a connected android device\n\
         apps List all installed apps on devices\n\
         help Prints this message or the help of the given subcommand(s)\n";

    let output = Command::new("target/debug/adbackup-cli")
        .arg("-h")
        .output().unwrap();

    let output_string = String::from_utf8(output.stdout).unwrap();
    let error_string = String::from_utf8(output.stderr).unwrap();

    assert!(output.status.success());
    assert_eq!(error_string, empty_string);
    assert_eq!(str::replace(&output_string, " ", ""), str::replace(&mocked_message, " ", ""));
}
