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

#[macro_use]
mod app;
mod single;

fn main() -> Result<(), String> {
    let app = clap::App::new("Cryogen")
        .version(crate_version!())
        .author("Ferris T. <ferristseng@fastmail.fm>")
        .about("Render a tera template with file data")
        .subcommand(single::Command::app())
        .get_matches();

    match app.subcommand() {
        ("single", Some(args)) => single::Command::exec(args),
        (cmd, _) => Err(format!("unexpected subcommand ({})", cmd)),
    }
}
