use std::path::Path;

use pretty_assertions::assert_eq;
use sauce::{
    shell::Zsh,
    test_utils::{mkpath, setup},
};

#[test]
fn it_works_when_no_saucefile_exists() {
    let (out, err, mut context) = setup();

    context.sauce_path = Path::new("does_not_exist.toml").to_path_buf();
    let shell_kind = Zsh {};
    context.execute(&shell_kind, false);
    assert_eq!(out.value(), "");
    assert_eq!(
        err.value(),
        format!("Sourced {}\n", context.sauce_path.to_string_lossy())
    );
}

#[test]
fn it_runs() {
    let (out, err, mut context) = setup();

    context.sauce_path = mkpath("./tests/execute_it_runs.toml");
    let shell_kind = Zsh {};
    context.execute(&shell_kind, false);
    assert_eq!(
        out.value(),
        r#"export TEST=example;

alias foo=git;

function meow {
  echo "$@"
};

"#
    );
    assert_eq!(
        err.value(),
        format!("Sourced {}\n", context.sauce_path.to_string_lossy())
    );
}

#[test]
fn it_no_ops_with_autoload_flag_when_autoload_is_disabled() {
    let (out, err, mut context) = setup();

    context.sauce_path = mkpath("./tests/execute_it_runs.toml");
    let shell_kind = Zsh {};
    context.execute(&shell_kind, true);
    assert_eq!(out.value(), "");
    assert_eq!(err.value(), "");
}

#[test]
fn it_loads_with_autoload_flag_when_autoload_is_enabled() {
    let (_, err, mut context) = setup();

    context.sauce_path = mkpath("./tests/execute_it_runs.toml");
    context.settings.autoload = Some(true);
    let shell_kind = Zsh {};
    context.execute(&shell_kind, true);
    assert_eq!(
        err.value(),
        format!("Sourced {}\n", context.sauce_path.to_string_lossy())
    );
}

#[test]
fn it_obeys_quiet() {
    let (_, err, mut context) = setup();

    context.sauce_path = mkpath("./tests/execute_it_runs.toml");
    context.settings.autoload = Some(true);
    context.output.set_quiet(true);

    let shell_kind = Zsh {};
    context.execute(&shell_kind, true);
    assert_eq!(err.value(), "");
}

#[test]
fn it_obeys_verbose() {
    let (out, err, mut context) = setup();

    context.sauce_path = mkpath("./tests/execute_it_runs.toml");
    context.output.set_quiet(true);
    context.output.set_verbose(true);

    let shell_kind = Zsh {};
    context.execute(&shell_kind, false);
    assert_eq!(out.value(), err.value());
    assert_eq!(out.value().contains("export TEST"), true);
}
