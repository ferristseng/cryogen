use app::open_template;
use clap::{App, Arg, ArgMatches, SubCommand};
use cryogen_prelude::{CompileVariablePlugin, Interpretation, Source, VarMapping};
use std::{fs::File, io::{stdout, Write}};
use tera::{Context, Tera};

/// Executes a plugin.
///
fn exec_plugin<'a, T>(args: &ArgMatches<'a>, template_vars: &mut Context) -> Result<(), String>
where
    T: CompileVariablePlugin,
{
    let plugin = T::from_args(args);

    let mappings = args.values_of(T::ARG_NAME).into_iter().flat_map(|a| a);
    for mapping in mappings.map(VarMapping::from_str) {
        let mapping = mapping?;
        let src = match T::ARG_INTERPRETATION {
            Interpretation::Raw => Source::Raw(mapping.arg_value(), 0),
            Interpretation::Path => {
                let file = File::open(mapping.arg_value()).map_err(|e| e.to_string())?;

                Source::File(file)
            }
        };

        template_vars.add(mapping.var_name(), &plugin.read(src)?);
    }

    Ok(())
}

/// Command to render a single output file from a tera template.
///
pub struct Command;

impl Command {
    const COMMAND_NAME: &'static str = "single";

    pub fn app<'a, 'b>() -> App<'a, 'b> {
        let plugins = plugins! {
            ::cryogen_plugin_primitives::StringPlugin;
            ::cryogen_plugin_primitives::FloatPlugin;
            ::cryogen_plugin_primitives::IntPlugin;
            ::cryogen_plugin_primitives::BooleanPlugin;
            #[cfg(feature = "github-markdown")]
            ::cryogen_plugin_github_markdown::GithubMarkdownPlugin;
            #[cfg(feature = "json")]
            ::cryogen_plugin_json::JsonPlugin;
            #[cfg(feature = "markdown")]
            ::cryogen_plugin_markdown::MarkdownPlugin;
            #[cfg(feature = "yaml")]
            ::cryogen_plugin_yaml::YamlPlugin;
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

    pub fn exec<'a>(args: &'a ArgMatches<'a>) -> Result<(), String> {
        let (template_path, template_contents) = open_template(&args);
        let mut template_vars = Context::new();

        macro_rules! exec {
            ( $( $(#[$feature:meta])* $plug:ty );*; ) => {
                $(
                    $(#[$feature])*
                    exec_plugin::<$plug>(&args, &mut template_vars)?;
                )*
            }
        }

        exec! {
            ::cryogen_plugin_primitives::StringPlugin;
            ::cryogen_plugin_primitives::FloatPlugin;
            ::cryogen_plugin_primitives::IntPlugin;
            ::cryogen_plugin_primitives::BooleanPlugin;
            #[cfg(feature = "github-markdown")]
            ::cryogen_plugin_github_markdown::GithubMarkdownPlugin;
            #[cfg(feature = "json")]
            ::cryogen_plugin_json::JsonPlugin;
            #[cfg(feature = "markdown")]
            ::cryogen_plugin_markdown::MarkdownPlugin;
            #[cfg(feature = "yaml")]
            ::cryogen_plugin_yaml::YamlPlugin;
        }

        Tera::one_off(&template_contents, &template_vars, false)
            .map_err(|e| {
                format!(
                    "failed one time render for template ({}): {}",
                    template_path,
                    e.description()
                )
            })
            .and_then(|rendered| {
                stdout()
                    .write_all(rendered.as_ref())
                    .map_err(|e| e.to_string())
            })
    }
}
