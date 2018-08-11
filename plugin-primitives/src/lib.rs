extern crate clap;
extern crate cryogen_prelude;

use clap::{Arg, ArgMatches};
use std::str::FromStr;

use cryogen_prelude::CompileVariablePlugin;

pub struct StringPlugin;

impl CompileVariablePlugin for StringPlugin {
    type RenderValue = String;

    fn plugin_name() -> &'static str {
        "string"
    }

    fn arg_full_name() -> &'static str {
        "string"
    }

    fn arg_help() -> &'static str {
        "Assign variable to string value"
    }

    fn additional_args() -> Vec<Arg<'static, 'static>> {
        vec![]
    }

    fn from_args<'a>(_: &'a ArgMatches<'a>) -> StringPlugin {
        StringPlugin
    }

    fn read_arg(&self, val: &str) -> Result<Self::RenderValue, String> {
        Ok(val.to_string())
    }
}

pub struct IntPlugin;

impl CompileVariablePlugin for IntPlugin {
    type RenderValue = isize;

    fn plugin_name() -> &'static str {
        "int"
    }

    fn arg_full_name() -> &'static str {
        "int"
    }

    fn arg_help() -> &'static str {
        "Assign variable to integer value"
    }

    fn additional_args() -> Vec<Arg<'static, 'static>> {
        vec![]
    }

    fn from_args<'a>(_: &'a ArgMatches<'a>) -> IntPlugin {
        IntPlugin
    }

    fn read_arg(&self, int: &str) -> Result<Self::RenderValue, String> {
        match isize::from_str(int) {
            Ok(i) => Ok(i),
            Err(e) => Err(format!("error parsing int: {:?}", e)),
        }
    }
}

pub struct FloatPlugin;

impl CompileVariablePlugin for FloatPlugin {
    type RenderValue = f64;

    fn plugin_name() -> &'static str {
        "float"
    }

    fn arg_full_name() -> &'static str {
        "float"
    }

    fn arg_help() -> &'static str {
        "Assign variable to float value"
    }

    fn additional_args() -> Vec<Arg<'static, 'static>> {
        vec![]
    }

    fn from_args<'a>(_: &'a ArgMatches<'a>) -> FloatPlugin {
        FloatPlugin
    }

    fn read_arg(&self, float: &str) -> Result<Self::RenderValue, String> {
        match f64::from_str(float) {
            Ok(f) => Ok(f),
            Err(e) => Err(format!("error parsing float: {:?}", e)),
        }
    }
}

pub struct BooleanPlugin;

impl CompileVariablePlugin for BooleanPlugin {
    type RenderValue = bool;

    fn plugin_name() -> &'static str {
        "bool"
    }

    fn arg_full_name() -> &'static str {
        "bool"
    }

    fn arg_help() -> &'static str {
        "Assign variable to boolean value"
    }

    fn additional_args() -> Vec<Arg<'static, 'static>> {
        vec![]
    }

    fn from_args<'a>(_: &'a ArgMatches<'a>) -> BooleanPlugin {
        BooleanPlugin
    }

    fn read_arg(&self, boolean: &str) -> Result<Self::RenderValue, String> {
        match bool::from_str(boolean) {
            Ok(b) => Ok(b),
            Err(e) => Err(format!("error parsing boolean: {:?}", e)),
        }
    }
}
