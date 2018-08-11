extern crate clap;
extern crate cryogen_prelude;
extern crate serde_json;

use clap::{Arg, ArgMatches};
use cryogen_prelude::CompileVariablePlugin;
use std::fs::File;

pub struct JsonPlugin;

impl CompileVariablePlugin for JsonPlugin {
    type RenderValue = serde_json::value::Value;

    const PLUGIN_NAME: &'static str = "json";

    const ARG_NAME: &'static str = "json";

    const HELP: &'static str = "Assign variable to contents of JSON file";

    #[inline]
    fn additional_args() -> Vec<Arg<'static, 'static>> {
        vec![]
    }

    #[inline]
    fn from_args<'a>(_: &'a ArgMatches<'a>) -> JsonPlugin {
        JsonPlugin
    }

    #[inline]
    fn read_arg(&self, path: &str) -> Result<Self::RenderValue, String> {
        File::open(path)
            .map_err(|e| e.to_string())
            .and_then(|f| serde_json::from_reader(f).map_err(|e| e.to_string()))
    }
}
