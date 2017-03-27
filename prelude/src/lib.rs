extern crate clap;
extern crate serde;

use serde::Serialize;

use clap::{Arg, ArgMatches};


/// Defines a mapping between a template variable, and the file to
/// populate it.
///
#[derive(Debug)]
pub struct VarMapping<'a> {
    var_name: &'a str,
    arg_value: &'a str,
}

impl<'a> VarMapping<'a> {
    /// Builds a VarMapping from a String
    ///
    pub fn from_str(s: &'a str) -> Result<VarMapping<'a>, &'static str> {
        let mut splits = s.splitn(2, ":");
        let var_name = splits.next().unwrap();

        match splits.next() {
            Some(arg_value) => {
                Ok(VarMapping {
                       var_name: var_name,
                       arg_value: arg_value,
                   })
            }
            None => Err("Expected a ':' in var mapping string"),
        }
    }

    /// Builds a VarMapping from a String, and panics if it fails.
    ///
    #[inline]
    pub fn from_str_panic(s: &'a str) -> VarMapping<'a> {
        match VarMapping::from_str(s) {
            Ok(s) => s,
            Err(e) => panic!(e),
        }
    }

    #[inline]
    pub fn arg_value(&self) -> &'a str {
        self.arg_value
    }

    #[inline]
    pub fn var_name(&self) -> &'a str {
        self.var_name
    }
}


/// A plugin that can assign a value to a variable in the Tera context from a
/// file.
///
pub trait CompileVariablePlugin {
    /// The serializable value to add to the Tera context.
    ///
    type RenderValue: Serialize;

    /// The name of the plugin.
    ///
    fn plugin_name() -> &'static str;

    /// The name of the argument that provides the variable name to file mapping.
    ///
    fn arg_full_name() -> &'static str;

    /// The help string to display.
    ///
    fn arg_help() -> &'static str;

    /// Optional arguments to supply. These should be prefixed to avoid namespace clashes.
    ///
    fn additional_args() -> Vec<Arg<'static, 'static>>;

    /// Constructor for building the plugin from the supplied command line arguments.
    ///
    fn from_args<'a>(args: &'a ArgMatches<'a>) -> Self;

    /// Reads the argument and returns a RenderValue instance.
    ///
    fn read_arg(&self, arg: &str) -> Result<Self::RenderValue, String>;
}
