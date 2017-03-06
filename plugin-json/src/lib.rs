extern crate clap;
extern crate cryogen_prelude;
extern crate serde_json;

use clap::{Arg, ArgMatches};
use std::fs::File;

use cryogen_prelude::CompileVariablePlugin;


pub struct JsonPlugin;

impl CompileVariablePlugin for JsonPlugin {
    type RenderValue = serde_json::value::Value;

    fn plugin_name() -> &'static str {
        "json"
    }

    fn arg_full_name() -> &'static str {
        "json"
    }

    fn arg_help() -> &'static str {
        "Assign variable to contents of JSON file"
    }


    fn additional_args() -> Vec<Arg<'static, 'static>> {
        vec![]
    }

    fn from_args<'a>(_: &'a ArgMatches<'a>) -> JsonPlugin {
        JsonPlugin
    }

    fn read_file(&self, mut file: File) -> Result<Self::RenderValue, String> {
        match serde_json::from_reader(&mut file) {
            Ok(obj) => Ok(obj),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
