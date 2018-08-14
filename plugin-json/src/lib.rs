extern crate clap;
extern crate cryogen_prelude;
extern crate serde_json;

use clap::{Arg, ArgMatches};
use cryogen_prelude::{CompileVariablePlugin, Interpretation, Source};
use std::io::Read;

pub struct JsonPlugin;

impl CompileVariablePlugin for JsonPlugin {
    type RenderValue = serde_json::value::Value;

    const PLUGIN_NAME: &'static str = "json";

    const ARG_NAME: &'static str = "json";

    const ARG_INTERPRETATION: Interpretation = Interpretation::Path;

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
    fn read<'a, R>(&self, src: Source<'a, R>) -> Result<Self::RenderValue, String>
    where
        R: Read,
    {
        serde_json::from_reader(src).map_err(|e| e.to_string())
    }
}
