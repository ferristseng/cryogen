extern crate clap;
extern crate cryogen_prelude;
extern crate serde_yaml;

use clap::{Arg, ArgMatches};
use std::fs::File;

use cryogen_prelude::CompileVariablePlugin;


pub struct YamlPlugin;

impl CompileVariablePlugin for YamlPlugin {
    type RenderValue = serde_yaml::Value;

    fn plugin_name() -> &'static str {
        "yaml"
    }

    fn arg_full_name() -> &'static str {
        "yaml"
    }

    fn arg_help() -> &'static str {
        "Assign variable to contents of YAML file"
    }


    fn additional_args() -> Vec<Arg<'static, 'static>> {
        vec![]
    }

    fn from_args<'a>(_: &'a ArgMatches<'a>) -> YamlPlugin {
        YamlPlugin
    }

    fn read_file(&self, mut file: File) -> Result<Self::RenderValue, String> {
        match serde_yaml::from_reader(&mut file) {
            Ok(obj) => Ok(obj),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
