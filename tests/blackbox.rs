use std::process::Command;

#[test]
fn test_help() {
    let empty_string = "";
    let mocked_message = "adbackup 0.1.0\nJulian Raufelder <julian.raufelder@htwg-konstanz.de\
    >:Jonas Reinwald <jonas.reinwald@htwg-konstanz.de>\nA backup tool for android using adb\n\nUSAG\
    E:\n    adbackup-cli [FLAGS] [SUBCOMMAND]\n\nFLAGS:\n    -h, --help       Prints help informati\
    on\n    -V, --version    Prints version information\n    -v               Increases logging ver\
    bosity each use for up to 3 times\n\nSUBCOMMANDS:\n    backup     Start backup of device\n    d\
    evices    List connected devices\n    pull       Pull file/folder from device\n    push       P\
    ush file/folder to device\n    apps       List all installed apps on devices\n    help       Pr\
    ints this message or the help of the given subcommand(s)\n";

    let output = Command::new("target/debug/adbackup-cli")
        .arg("-h")
        .output().unwrap();

    let output_string = String::from_utf8(output.stdout).unwrap();
    let error_string = String::from_utf8(output.stderr).unwrap();

    assert!(output.status.success());
    assert_eq!(error_string, empty_string);
    assert_eq!(output_string, mocked_message);
}
