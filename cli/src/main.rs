extern crate clap;
extern crate cryogen_prelude;
extern crate tera;
#[cfg(feature = "json")]
extern crate cryogen_plugin_json;
#[cfg(feature = "markdown")]
extern crate cryogen_plugin_markdown;
#[cfg(feature = "yaml")]
extern crate cryogen_plugin_yaml;

use clap::{Arg, ArgMatches, App, SubCommand};
use std::fs::File;
use std::io::{Read, Write};
use tera::{Context, Tera};

use cryogen_prelude::{CompileVariablePlugin, VarMapping};


// Opens the tera template specified in ArgMatches.
//
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


// Command to render a single output file from a tera template.
//
struct SingleCommand;

impl SingleCommand {
    fn command_name() -> &'static str {
        "single"
    }

    fn exec_plugin<'a, T: CompileVariablePlugin>(args: &ArgMatches<'a>,
                                                 template_vars: &mut Context) {
        let plugin = T::from_args(args);

        match args.values_of(T::arg_full_name()) {
            Some(mappings) => {
                for mapping in mappings.map(VarMapping::from_str_panic) {
                    match plugin.read_path(mapping.file_path()) {
                        Ok(value) => template_vars.add(mapping.var_name(), &value),
                        Err(e) => {
                            panic!(format!("failed to parse file for var ({}): {:?}",
                                           mapping.var_name(),
                                           e));
                        }
                    }
                }
            }
            None => (),
        }
    }

    fn register_plugin<T: CompileVariablePlugin>(plugins: &mut Vec<Arg<'static, 'static>>) {
        plugins.push(Arg::with_name(T::plugin_name())
            .long(T::arg_full_name())
            .help(T::arg_help())
            .takes_value(true)
            .multiple(true));
        plugins.extend(T::additional_args());
    }

    fn app<'a, 'b>() -> App<'a, 'b> {
        let mut plugins = Vec::new();

        #[cfg(feature = "json")]
        SingleCommand::register_plugin::<cryogen_plugin_json::JsonPlugin>(&mut plugins);
        #[cfg(feature = "markdown")]
        SingleCommand::register_plugin::<cryogen_plugin_markdown::MarkdownPlugin>(&mut plugins);
        #[cfg(feature = "yaml")]
        SingleCommand::register_plugin::<cryogen_plugin_yaml::YamlPlugin>(&mut plugins);

        SubCommand::with_name(SingleCommand::command_name())
            .about("Renders a single output file")
            .arg(Arg::with_name("TEMPLATE")
                .help("The tera template to render")
                .required(true)
                .index(1))
            .args(&plugins)
    }

    fn exec<'a>(args: &'a ArgMatches<'a>) {
        let (template_path, template_contents) = open_template(&args);
        let mut template_vars = Context::new();

        #[cfg(feature = "json")]
        SingleCommand::exec_plugin::<cryogen_plugin_json::JsonPlugin>(&args, &mut template_vars);
        #[cfg(feature = "markdown")]
        SingleCommand::exec_plugin::<cryogen_plugin_markdown::MarkdownPlugin>(&args,
                                                                              &mut template_vars);
        #[cfg(feature = "yaml")]
        SingleCommand::exec_plugin::<cryogen_plugin_yaml::YamlPlugin>(&args, &mut template_vars);

        match Tera::one_off(&template_contents, &template_vars, false) {
            Ok(rendered) => {
                let _ = ::std::io::stdout().write_all(rendered.as_ref());
            }
            Err(e) => {
                panic!(format!("failed one time render for template ({}): {:?}",
                               template_path,
                               e))
            }
        };
    }
}


fn main() {
    let app = App::new("Cryogen")
        .version("1.0")
        .author("Ferris T. <ferristseng@fastmail.fm>")
        .about("Render a tera template with file data")
        .subcommand(SingleCommand::app())
        .get_matches();

    match app.subcommand() {
        ("single", Some(args)) => SingleCommand::exec(args),
        _ => panic!("unexpected subcommand"),
    }
}
