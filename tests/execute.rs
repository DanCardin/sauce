use std::path::Path;

use pretty_assertions::assert_eq;
use sauce::{
    settings::Settings,
    shell::Zsh,
    test_utils::{mkpath, setup},
    Context,
};

fn corpus() -> corpus::Corpus {
    let current_dir = std::env::current_dir().unwrap();
    corpus::builder()
        .relative_to(&current_dir)
        .with_root(current_dir)
        .with_extension("toml")
        .build()
        .unwrap()
}

#[test]
fn it_works_when_no_saucefile_exists() {
    let (out, err, mut output) = setup();

    let mut context =
        Context::default().with_sauce_path(Path::new("does_not_exist.toml").to_path_buf());

    let shell_kind = Zsh {};
    context.execute(&shell_kind, false, &mut output);
    assert_eq!(out.value(), "");
    assert_eq!(err.value(), format!("No saucefiles exist\n"));
}

#[test]
fn it_runs() {
    let (out, err, mut output) = setup();

    let mut context = Context::default()
        .with_corpus(corpus())
        .at_path("./tests/execute_it_runs");
    let shell_kind = Zsh {};
    context.execute(&shell_kind, false, &mut output);

    assert_eq!(
        err.value()
            .starts_with("Sauced tests/execute_it_runs.toml from"),
        true
    );

    assert_eq!(
        out.value(),
        r#"export TEST=example;

alias foo=git;

function meow {
  echo "$@"
};

"#
    );
}

#[test]
fn it_no_ops_with_autoload_flag_when_autoload_is_disabled() {
    let (out, err, mut output) = setup();

    let mut context = Context::default().with_sauce_path(mkpath("./tests/execute_it_runs.toml"));
    let shell_kind = Zsh {};
    context.execute(&shell_kind, true, &mut output);
    assert_eq!(out.value(), "");
    assert_eq!(err.value(), "");
}

#[test]
fn it_loads_with_autoload_flag_when_autoload_is_enabled() {
    let (_, err, mut output) = setup();

    let mut context = Context::default()
        .with_corpus(corpus())
        .with_sauce_path(mkpath("./tests/execute_it_runs.toml"))
        .with_settings(Settings {
            autoload: Some(true),
            ..Default::default()
        });

    let shell_kind = Zsh {};
    context.execute(&shell_kind, true, &mut output);
    assert_eq!(
        err.value()
            .starts_with("Sauced tests/execute_it_runs.toml from"),
        true
    );
}

#[test]
fn it_obeys_quiet() {
    let (_, err, mut output) = setup();

    let mut context = Context::default().with_sauce_path(mkpath("./tests/execute_it_runs.toml"));
    output.set_quiet(true);

    let shell_kind = Zsh {};
    context.execute(&shell_kind, true, &mut output);
    assert_eq!(err.value(), "");
}

#[test]
fn it_obeys_verbose() {
    let (out, err, mut output) = setup();

    let mut context = Context::default().with_sauce_path(mkpath("./tests/execute_it_runs.toml"));
    output.set_quiet(true);
    output.set_verbose(true);

    let shell_kind = Zsh {};
    context.execute(&shell_kind, false, &mut output);
    assert_eq!(out.value(), err.value());
    assert_eq!(out.value().contains("export TEST"), true);
}

#[test]
fn it_creates_correct_file_during_new_command() {
    let (_, err, mut output) = setup();
    output.set_show(true);

    let context = Context::default()
        .at_path(Path::new("./foo/bar/baz"))
        .with_corpus(corpus());

    context.create_saucefile(&mut output);

    let corpus = corpus();
    let root = corpus.root_location.to_string_lossy();
    let expected_result = format!("Created {root}/foo/bar/baz.toml\n");

    assert_eq!(err.value(), expected_result);
}

#[test]
fn it_moves_correct_file_during_mv_command() {
    let (_, err, mut output) = setup();
    output.set_show(true);

    let current_dir = std::env::current_dir().unwrap();
    let corp = corpus::builder()
        .relative_to(&current_dir)
        .with_root("/.local/share")
        .with_extension("toml")
        .build()
        .unwrap();

    let context = Context::default().with_corpus(corp);

    context.move_saucefile(&mut output, Path::new("./src"), true);

    let expected_result = format!("Moved /.local/share.toml to /.local/share/src.toml\n");

    assert_eq!(err.value(), expected_result);
}
