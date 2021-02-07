use assert_cmd::Command;
use indoc::indoc;

#[test]
fn it_emits_shell_init_content() {
    let mut cmd = Command::cargo_bin("sauce").unwrap();
    let assert = cmd.args(&["--shell", "bash", "shell", "init"]).assert();
    assert
        .success()
        .stdout(predicates::str::contains("sauce --shell bash"));
}

#[test]
fn it_runs_sauce() {
    let mut cmd = Command::cargo_bin("sauce").unwrap();
    let assert = cmd.args(&["--shell", "bash"]).assert();
    assert.success();
}

#[test]
fn it_runs_sauce_in_show_mode() {
    let mut cmd = Command::cargo_bin("sauce").unwrap();
    let assert = cmd.args(&["--shell", "bash", "--show"]).assert();
    assert.success();
}

#[test]
fn it_runs_sauce_show_env() {
    let mut cmd = Command::cargo_bin("sauce").unwrap();
    let assert = cmd
        .args(&[
            "--shell=bash",
            "--file=tests/execute_it_runs.toml",
            "show",
            "env",
        ])
        .assert();
    assert.success().stderr(indoc!(
        "
        ┌──────────┬─────────┐
        │ Variable │ Value   │
        ╞══════════╪═════════╡
        │ TEST     │ example │
        └──────────┴─────────┘
        "
    ));
}

#[test]
fn it_runs_shell_exec() {
    let mut cmd = Command::cargo_bin("sauce").unwrap();
    let assert = cmd
        .args(&["--shell", "bash", "shell", "exec", "'echo foo'"])
        .assert();
    assert.success().stderr(predicates::str::contains("foo"));
}
