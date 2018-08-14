extern crate clap;
extern crate cryogen_prelude;
extern crate serde_yaml;

use clap::{Arg, ArgMatches};
use cryogen_prelude::{CompileVariablePlugin, Interpretation, Source};
use std::io::Read;

pub struct YamlPlugin;

impl CompileVariablePlugin for YamlPlugin {
    type RenderValue = serde_yaml::Value;

    const PLUGIN_NAME: &'static str = "yaml";

    const ARG_NAME: &'static str = "yaml";

    const ARG_INTERPRETATION: Interpretation = Interpretation::Path;

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
    fn read<'a, R>(&self, src: Source<'a, R>) -> Result<Self::RenderValue, String>
    where
        R: Read,
    {
        serde_yaml::from_reader(src).map_err(|e| e.to_string())
    }
}
