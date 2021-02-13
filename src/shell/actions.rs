use std::path::Path;

use crate::{
    colors::{BLUE, RED, YELLOW},
    filter::{parse_match_option, FilterOptions},
    output::{ErrorCode, Output},
    saucefile::Saucefile,
    settings::Settings,
    shell::{utilities::get_binary, Shell},
    target::Target,
};

pub fn edit(output: &mut Output, shell: &dyn Shell, path: &Path) {
    let path = path.to_string_lossy();
    output.notify(&[BLUE.paint("Opening "), YELLOW.paint(path.as_ref())]);

    if let Some(result) = shell.edit(std::env::var_os("EDITOR"), &path) {
        output.output(result);
    } else {
        output.notify(&[RED.paint("set $EDITOR to enable this command")]);
    }
}

pub fn init(output: &mut Output, shell: &dyn Shell, autoload_hook: bool) {
    let binary = get_binary();
    let result = shell.init(&binary, autoload_hook);
    output.output(result);
}

pub fn execute_shell_command(output: &mut Output, shell: &dyn Shell, command: &str) {
    let result = subprocess::Exec::cmd(shell.name())
        .arg("-i")
        .arg("-c")
        .arg(format!("{}; {}", clap::crate_name!(), command))
        .stdout(subprocess::Redirection::Merge)
        .join();

    if let Err(error) = result {
        output.notify(&[RED.bold().paint(error.to_string())]);
    }
}

pub fn create_saucefile(output: &mut Output, sauce_path: &Path) {
    let parent = sauce_path.parent().unwrap();
    if std::fs::create_dir_all(parent).is_err() {
        output.notify_error(
            ErrorCode::WriteError,
            &[
                RED.paint("Couldn't create "),
                YELLOW.paint(parent.to_string_lossy()),
            ],
        );
        return;
    }

    if sauce_path.is_file() {
        output.notify_error(
            ErrorCode::WriteError,
            &[
                RED.bold().paint("File already exists at "),
                YELLOW.paint(sauce_path.to_string_lossy()),
            ],
        );
    } else if std::fs::File::create(&sauce_path).is_err() {
        output.notify_error(
            ErrorCode::WriteError,
            &[
                RED.bold().paint("Couldn't create"),
                YELLOW.paint(sauce_path.to_string_lossy()),
            ],
        );
    } else {
        output.notify(&[
            BLUE.bold().paint("Created"),
            YELLOW.paint(sauce_path.to_string_lossy()),
        ]);
    }
}

pub fn clear(
    output: &mut Output,
    shell: &dyn Shell,
    saucefile: &Saucefile,
    global_settings: &Settings,
    filter_options: &FilterOptions,
) {
    let local_settings = saucefile.settings();
    let settings = local_settings.resolve_precedence(global_settings);
    let filter_exclusions = settings
        .clear_ignore
        .iter()
        .flat_map(|i| parse_match_option(Some(i)))
        .collect::<Vec<_>>();

    let filter_options = FilterOptions {
        filter_exclusions: filter_exclusions.as_slice(),
        ..filter_options.clone()
    };

    output.output(render_items(saucefile.vars(&filter_options), |k, _| {
        shell.unset_var(k)
    }));
    output.output(render_items(saucefile.aliases(&filter_options), |k, _| {
        shell.unset_alias(k)
    }));
    output.output(render_items(
        saucefile.functions(&filter_options),
        |k, _| shell.unset_function(k),
    ));
    output.notify(&[BLUE.bold().paint("Cleared your sauce")]);
}

pub fn show(
    output: &mut Output,
    filter_options: &FilterOptions,
    target: Target,
    saucefile: &Saucefile,
) {
    let header = match target {
        Target::EnvVar => &["Variable", "Value"],
        Target::Alias => &["Alias", "Value"],
        Target::Function => &["Function", "Body"],
    };

    let pairs = match target {
        Target::EnvVar => saucefile.vars(filter_options),
        Target::Alias => saucefile.aliases(filter_options),
        Target::Function => saucefile.functions(filter_options),
    };
    let preset = match target {
        Target::EnvVar => None,
        Target::Alias => None,
        Target::Function => Some("││──╞═╪╡│ │││┬┴┌┐└┘"),
    };

    let cells = pairs
        .iter()
        .map(|(k, v)| vec![<&str>::clone(k), v])
        .collect::<Vec<_>>();
    let table = output.format_table(header, cells, preset);

    output.notify_str(&table);
}

pub fn execute(
    output: &mut Output,
    shell: &dyn Shell,
    saucefile: &Saucefile,
    global_settings: &Settings,
    filter_options: &FilterOptions,
    autoload_flag: bool,
) {
    // The `autoload_flag` indicates that the "context" of the execution is happening during
    // an autoload, i.e. `cd`. It's the precondition for whether we need to check the settings to
    // see whether we **actually** should perform the autoload, or exit early.
    if autoload_flag
        && !saucefile
            .settings()
            .resolve_precedence(&global_settings)
            .autoload
    {
        return;
    }

    output.output(render_items(saucefile.vars(&filter_options), |k, v| {
        shell.set_var(k, v)
    }));
    output.output(render_items(saucefile.aliases(&filter_options), |k, v| {
        shell.set_alias(k, v)
    }));
    output.output(render_items(
        saucefile.functions(&filter_options),
        |k, v| shell.set_function(k, v),
    ));

    output.notify(&[
        BLUE.bold().paint("Sourced "),
        YELLOW.paint(saucefile.path.to_string_lossy()),
    ]);
}

fn render_items<F>(items: Vec<(&str, String)>, mut format_row: F) -> String
where
    F: FnMut(&str, &str) -> String,
{
    items
        .iter()
        .map(|(k, v)| format_row(k, v))
        .map(|mut v| {
            v += ";\n";
            v
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::toml::{ensure_section, value_from_string};

    use crate::test_utils::{setup, TestShell};
    use indoc::indoc;

    mod edit {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_respects_editor_env_var_zsh() {
            std::env::set_var("EDITOR", "edit");

            let (out, err, mut output) = setup();

            let shell = TestShell {};
            edit(&mut output, &shell, &Path::new("foo/bar"));

            assert_eq!(out.value(), "edit 'foo/bar'\n");
            assert_eq!(err.value(), "Opening foo/bar\n");
        }
    }

    mod init {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_defaults() {
            let (out, err, mut output) = setup();
            let shell = TestShell {};
            init(&mut output, &shell, false);

            assert_eq!(out.value(), "sauce\n");
            assert_eq!(err.value(), "");
        }

        #[test]
        fn it_emits_autoload() {
            let (out, err, mut output) = setup();
            let shell = TestShell {};

            init(&mut output, &shell, true);

            assert_eq!(out.value(), "sauce --autoload\n");
            assert_eq!(err.value(), "");
        }
    }

    mod clear {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_clears() {
            let shell = TestShell {};
            let (out, err, mut output) = setup();
            let mut saucefile = Saucefile::default();

            let section = ensure_section(&mut saucefile.document, "environment");
            section["var"] = value_from_string("varvalue");

            let section = ensure_section(&mut saucefile.document, "alias");
            section["alias"] = value_from_string("aliasvalue");

            let section = ensure_section(&mut saucefile.document, "function");
            section["fn"] = value_from_string("fnvalue");

            clear(
                &mut output,
                &shell,
                &mut saucefile,
                &Settings::default(),
                &FilterOptions::default(),
            );

            assert_eq!(out.value(), "unset var;\n\nunalias alias;\n\nunset fn;\n\n");
            assert_eq!(err.value(), "Cleared your sauce\n");
        }
    }

    mod show {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_shows_env_vars() {
            let (out, err, mut output) = setup();
            let mut saucefile = Saucefile::default();

            let section = ensure_section(&mut saucefile.document, "environment");
            section["var"] = value_from_string("varvalue");

            show(
                &mut output,
                &FilterOptions::default(),
                Target::EnvVar,
                &mut saucefile,
            );

            assert_eq!(out.value(), "");
            assert_eq!(
                err.value(),
                indoc!(
                    "
                    ┌──────────┬──────────┐
                    │ Variable │ Value    │
                    ╞══════════╪══════════╡
                    │ var      │ varvalue │
                    └──────────┴──────────┘
                    "
                )
            );
        }

        #[test]
        fn it_shows_aliases() {
            let (out, err, mut output) = setup();
            let mut saucefile = Saucefile::default();

            let section = ensure_section(&mut saucefile.document, "alias");
            section["alias"] = value_from_string("aliasvalue");

            show(
                &mut output,
                &FilterOptions::default(),
                Target::Alias,
                &saucefile,
            );

            assert_eq!(out.value(), "");
            assert_eq!(
                err.value(),
                indoc!(
                    "
                    ┌───────┬────────────┐
                    │ Alias │ Value      │
                    ╞═══════╪════════════╡
                    │ alias │ aliasvalue │
                    └───────┴────────────┘
                    "
                )
            );
        }

        #[test]
        fn it_shows_functions() {
            let (out, err, mut output) = setup();
            let mut saucefile = Saucefile::default();

            let section = ensure_section(&mut saucefile.document, "function");
            section["function"] = value_from_string("git add\ngit commit");

            show(
                &mut output,
                &FilterOptions::default(),
                Target::Function,
                &saucefile,
            );

            assert_eq!(out.value(), "");
            assert_eq!(
                err.value(),
                indoc!(
                    "
                    ┌──────────┬────────────┐
                    │ Function │ Body       │
                    ╞══════════╪════════════╡
                    │ function │ git add    │
                    │          │ git commit │
                    └──────────┴────────────┘
                    "
                )
            );
        }
    }

    mod execute {
        use super::super::*;
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_executes() {
            let shell = TestShell {};
            let (out, err, mut output) = setup();
            let mut saucefile = Saucefile::default();

            let section = ensure_section(&mut saucefile.document, "environment");
            section["var"] = value_from_string("varvalue");

            let section = ensure_section(&mut saucefile.document, "alias");
            section["alias"] = value_from_string("aliasvalue");

            let section = ensure_section(&mut saucefile.document, "function");
            section["fn"] = value_from_string("fnvalue");

            execute(
                &mut output,
                &shell,
                &saucefile,
                &Settings::default(),
                &FilterOptions::default(),
                false,
            );

            assert_eq!(
                out.value(),
                "export var=varvalue;\n\nalias alias=aliasvalue;\n\nfunction fn=fnvalue;\n\n"
            );
            assert_eq!(err.value(), "Sourced \n");
        }

        #[test]
        fn it_doesnt_execute_with_autoload_flag_and_its_disabled() {
            let shell = TestShell {};
            let (out, err, mut output) = setup();

            execute(
                &mut output,
                &shell,
                &Saucefile::default(),
                &Settings::default(),
                &FilterOptions::default(),
                true,
            );

            assert_eq!(out.value(), "");
            assert_eq!(err.value(), "");
        }
    }
}
