use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_dnd_stats"))
}

#[test]
fn list_outputs_all_methods() {
    let output = bin().arg("list").output().expect("failed to run");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();

    let expected = [
        "stdarr",
        "3d6",
        "4d6",
        "roll3_reroll_under8",
        "roll3_reroll_1s",
        "roll3_1s_are_6s",
        "roll18",
        "roll24",
        "3up3down",
        "6x6gridMax",
        "6x6gridTotal",
        "6x6grid4d6",
    ];

    assert_eq!(lines, expected, "list output doesn't match METHOD_NAMES");
}

#[test]
fn once_succeeds_for_each_method() {
    let methods = [
        "stdarr",
        "3d6",
        "4d6",
        "roll3_reroll_under8",
        "roll3_reroll_1s",
        "roll3_1s_are_6s",
        "roll18",
        "roll24",
        "3up3down",
        "6x6gridMax",
        "6x6gridTotal",
        "6x6grid4d6",
    ];

    for method in methods {
        let output = bin()
            .args(["once", method])
            .output()
            .unwrap_or_else(|e| panic!("failed to run 'once {method}': {e}"));
        assert!(
            output.status.success(),
            "'once {method}' exited with status {}",
            output.status
        );
        assert!(
            !output.stdout.is_empty(),
            "'once {method}' produced no output"
        );
    }
}

#[test]
fn once_6x6grid_total_succeeds() {
    let output = bin().args(["once", "6x6gridTotal"]).output().expect("failed to run");
    assert!(output.status.success());
    assert!(!output.stdout.is_empty());
}

#[test]
fn once_6x6grid_4d6_succeeds() {
    let output = bin().args(["once", "6x6grid4d6"]).output().expect("failed to run");
    assert!(output.status.success());
    assert!(!output.stdout.is_empty());
}

#[test]
fn stats_succeeds() {
    let output = bin()
        .args(["stats", "--iters", "100"])
        .output()
        .expect("failed to run");
    assert!(output.status.success(), "stats exited with status {}", output.status);
    assert!(!output.stdout.is_empty(), "stats produced no output");
}
