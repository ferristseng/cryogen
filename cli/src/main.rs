#[macro_use]
extern crate clap;
#[cfg(feature = "github-markdown")]
extern crate cryogen_plugin_github_markdown;
#[cfg(feature = "json")]
extern crate cryogen_plugin_json;
#[cfg(feature = "markdown")]
extern crate cryogen_plugin_markdown;
extern crate cryogen_plugin_primitives;
#[cfg(feature = "yaml")]
extern crate cryogen_plugin_yaml;
extern crate cryogen_prelude;
extern crate tera;

use clap::{App, Arg, ArgMatches, SubCommand};
use cryogen_prelude::{CompileVariablePlugin, VarMapping};
use std::fs::File;
use std::io::{Read, Write};
use tera::{Context, Tera};

// Build a vector of plugins to use.
//
macro_rules! plugins {
    ( $( $(#[$feature:meta])* $plug:ty );*; ) => {
        {
            let mut plugins = Vec::new();
            $(
                $(#[$feature])*
                Self::register_plugin::<$plug>(&mut plugins);
            )*
            plugins
        }
    }
}

/// Opens the tera template specified in ArgMatches.
///
fn open_template<'a>(args: &'a ArgMatches<'a>) -> (&'a str, String) {
    let file_path = args.value_of("TEMPLATE").unwrap();

    match File::open(file_path) {
        Ok(mut file) => {
            let mut buf = String::new();
            let _ = file.read_to_string(&mut buf);
            (file_path, buf)
        }
        Err(e) => panic!(format!("failed to open template ({}): {:?}", file_path, e)),
    }
}

/// Command to render a single output file from a tera template.
///
struct SingleCommand;

impl SingleCommand {
    const COMMAND_NAME: &'static str = "single";

    fn exec_plugin<'a, T>(args: &ArgMatches<'a>, template_vars: &mut Context)
    where
        T: CompileVariablePlugin,
    {
        let plugin = T::from_args(args);

        match args.values_of(T::ARG_NAME) {
            Some(mappings) => for mapping in mappings.map(VarMapping::from_str_panic) {
                match plugin.read_arg(mapping.arg_value()) {
                    Ok(value) => template_vars.add(mapping.var_name(), &value),
                    Err(e) => {
                        panic!(format!(
                            "failed to parse file for var ({}): {:?}",
                            mapping.var_name(),
                            e
                        ));
                    }
                }
            },
            None => (),
        }
    }

    #[inline]
    fn register_plugin<T>(plugins: &mut Vec<Arg<'static, 'static>>)
    where
        T: CompileVariablePlugin,
    {
        plugins.push(
            Arg::with_name(T::PLUGIN_NAME)
                .long(T::ARG_NAME)
                .help(T::HELP)
                .takes_value(true)
                .multiple(true),
        );
        plugins.extend(T::additional_args());
    }

    fn app<'a, 'b>() -> App<'a, 'b> {
        let plugins = plugins! {
            cryogen_plugin_primitives::StringPlugin;
            cryogen_plugin_primitives::FloatPlugin;
            cryogen_plugin_primitives::IntPlugin;
            cryogen_plugin_primitives::BooleanPlugin;
            #[cfg(feature = "github-markdown")]
            cryogen_plugin_github_markdown::GithubMarkdownPlugin;
            #[cfg(feature = "json")]
            cryogen_plugin_json::JsonPlugin;
            #[cfg(feature = "markdown")]
            cryogen_plugin_markdown::MarkdownPlugin;
            #[cfg(feature = "yaml")]
            cryogen_plugin_yaml::YamlPlugin;
        };

        SubCommand::with_name(Self::COMMAND_NAME)
            .about("Renders a single output file")
            .arg(
                Arg::with_name("TEMPLATE")
                    .help("The tera template to render")
                    .required(true)
                    .index(1),
            )
            .args(&plugins)
    }

    fn exec<'a>(args: &'a ArgMatches<'a>) {
        let (template_path, template_contents) = open_template(&args);
        let mut template_vars = Context::new();

        macro_rules! exec {
            ( $( $(#[$feature:meta])* $plug:ty );*; ) => {
                $(
                    $(#[$feature])*
                    Self::exec_plugin::<$plug>(&args, &mut template_vars);
                )*
            }
        }

        exec! {
            cryogen_plugin_primitives::StringPlugin;
            cryogen_plugin_primitives::FloatPlugin;
            cryogen_plugin_primitives::IntPlugin;
            cryogen_plugin_primitives::BooleanPlugin;
            #[cfg(feature = "github-markdown")]
            cryogen_plugin_github_markdown::GithubMarkdownPlugin;
            #[cfg(feature = "json")]
            cryogen_plugin_json::JsonPlugin;
            #[cfg(feature = "markdown")]
            cryogen_plugin_markdown::MarkdownPlugin;
            #[cfg(feature = "yaml")]
            cryogen_plugin_yaml::YamlPlugin;
        }

        match Tera::one_off(&template_contents, &template_vars, false) {
            Ok(rendered) => {
                let _ = ::std::io::stdout().write_all(rendered.as_ref());
            }
            Err(e) => panic!(format!(
                "failed one time render for template ({}): {}",
                template_path,
                e.description()
            )),
        };
    }
}

fn main() {
    let app = App::new("Cryogen")
        .version(crate_version!())
        .author("Ferris T. <ferristseng@fastmail.fm>")
        .about("Render a tera template with file data")
        .subcommand(SingleCommand::app())
        .get_matches();

    match app.subcommand() {
        ("single", Some(args)) => SingleCommand::exec(args),
        _ => panic!("unexpected subcommand"),
    }
}
