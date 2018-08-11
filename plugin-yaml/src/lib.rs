extern crate clap;
extern crate cryogen_prelude;
extern crate serde_yaml;

use clap::{Arg, ArgMatches};
use cryogen_prelude::CompileVariablePlugin;
use std::fs::File;

pub struct YamlPlugin;

impl CompileVariablePlugin for YamlPlugin {
    type RenderValue = serde_yaml::Value;

    const PLUGIN_NAME: &'static str = "yaml";

    const ARG_NAME: &'static str = "yaml";

    const HELP: &'static str = "Assign variable to contents of YAML file";

    #[inline]
    fn additional_args() -> Vec<Arg<'static, 'static>> {
        vec![]
    }

    #[inline]
    fn from_args<'a>(_: &'a ArgMatches<'a>) -> YamlPlugin {
        YamlPlugin
    }

    #[inline]
    fn read_arg(&self, path: &str) -> Result<Self::RenderValue, String> {
        File::open(path)
            .map_err(|e| e.to_string())
            .and_then(|f| serde_yaml::from_reader(f).map_err(|e| e.to_string()))
    }
}
