use std::process::Command;

#[test]
fn test_help() {
    let empty_string = "";
    let mocked_message = "adbackup 0.1.0\nJulian Raufelder \
        <julian.raufelder@htwg-konstanz.de:Jonas Reinwald \
        <jonas.reinwald@htwg-konstanz.de>\nA backup tool for android using adb\
        \n\nUSAGE:\n    adbackup\n\nFLAGS:\n    -h, --help       Prints help information\
        \n    -V, --version    Prints version information\n";

    let output = Command::new("target/debug/adbackup")
        .arg("-h")
        .output().unwrap();

    let output_string = String::from_utf8(output.stdout).unwrap();
    let error_string = String::from_utf8(output.stderr).unwrap();

    assert!(output.status.success());
    assert_eq!(error_string, empty_string);
    assert_eq!(output_string, mocked_message);
}
