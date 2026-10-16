use std::process::Command;

#[test]
fn binary_prints_its_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_esm"))
        .output()
        .expect("the esm binary runs");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        cli::version_line()
    );
}
