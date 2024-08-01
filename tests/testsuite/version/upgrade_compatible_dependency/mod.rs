use cargo_test_support::compare::assert_ui;
use cargo_test_support::current_dir;
use cargo_test_support::file;

use crate::git_from;
use crate::init_registry;
use crate::CargoCommand;

#[cargo_test]
fn case() {
    init_registry();
    let project = git_from(current_dir!().join("in"));
    let project_root = project.root();
    let cwd = &project_root;

    snapbox::cmd::Command::cargo_ui()
        .arg("release")
        .args([
            "version",
            "0.4.5",
            "-p",
            "cargo-list-test-fixture-dependency",
            "-x",
            "--no-confirm",
        ])
        .current_dir(cwd)
        .assert()
        .success()
        .stdout_eq(file!["stdout.term.svg"])
        .stderr_eq(file!["stderr.term.svg"]);

    assert_ui().subset_matches(current_dir!().join("out"), &project_root);
}
