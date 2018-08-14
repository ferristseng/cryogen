extern crate clap;
extern crate cryogen_prelude;

use clap::{Arg, ArgMatches};
use cryogen_prelude::{CompileVariablePlugin, Interpretation, Source};
use std::{io::Read, str::FromStr};

/// Creates a plugin to input a assign a primitive type to a variable.
///
macro_rules! primitive_plugin {
    (
        PluginType = $plugin:ident;
        RenderValue = $render:ty;
        PluginName = $name:expr;
        ArgName = $arg:expr;
        Help = $help:expr;
        ($bind:ident) => $eval:expr;
    ) => {
        pub struct $plugin;

        impl CompileVariablePlugin for $plugin {
            type RenderValue = $render;

            const PLUGIN_NAME: &'static str = $name;

            const ARG_NAME: &'static str = $arg;

            const ARG_INTERPRETATION: Interpretation = Interpretation::Raw;

            const HELP: &'static str = $help;

            #[inline]
            fn additional_args() -> Vec<Arg<'static, 'static>> {
                vec![]
            }

            #[inline]
            fn from_args<'a>(_: &'a ArgMatches<'a>) -> Self {
                $plugin
            }

            #[inline]
            fn read<'a, R>(&self, src: Source<'a, R>) -> Result<Self::RenderValue, String>
            where
                R: Read
            {
                let $bind = src.consume()?;
                let $bind = &$bind[..];
                $eval
            }
        }
    }
}

primitive_plugin! {
    PluginType = StringPlugin;
    RenderValue = String;
    PluginName = "string";
    ArgName = "string";
    Help = "Assign variable to string value";
    (val) => Ok(val.to_string());
}

primitive_plugin! {
    PluginType = IntPlugin;
    RenderValue = isize;
    PluginName = "int";
    ArgName = "int";
    Help = "Assign variable to integer value";
    (val) => isize::from_str(val).map_err(|e| format!("error parsing int: {:?}", e));
}

primitive_plugin! {
    PluginType = FloatPlugin;
    RenderValue = f64;
    PluginName = "float";
    ArgName = "float";
    Help = "Assign variable to float value";
    (val) => f64::from_str(val).map_err(|e| format!("error parsing float: {:?}", e));
}

primitive_plugin! {
    PluginType = BooleanPlugin;
    RenderValue = bool;
    PluginName = "bool";
    ArgName = "bool";
    Help = "Assign variable to boolean value";
    (val) => bool::from_str(val).map_err(|e| format!("error parsing boolean: {:?}", e));
}
