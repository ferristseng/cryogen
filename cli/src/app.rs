use clap::{Arg, ArgMatches};
use cryogen_prelude::CompileVariablePlugin;
use std::{fs::File, io::Read};

// Build a vector of plugins to use.
//
#[macro_export]
macro_rules! plugins {
    ( $( $(#[$feature:meta])* $plug:ty );*; ) => {
        {
            let mut plugins = Vec::new();
            $(
                $(#[$feature])*
                ::app::register_plugin::<$plug>(&mut plugins);
            )*
            plugins
        }
    }
}

/// Opens the tera template specified in ArgMatches.
///
pub fn open_template<'a>(args: &'a ArgMatches<'a>) -> (&'a str, String) {
    let file_path = args.value_of("TEMPLATE").unwrap();

    match File::open(file_path) {
        Ok(mut file) => {
            let mut buf = String::new();
            let _ = file.read_to_string(&mut buf);
            (file_path, buf)
        }
        Err(e) => panic!(format!("failed to open template ({}): {:?}", file_path, e)),
    }
}

/// Registers a plugin.
///
#[inline]
pub fn register_plugin<T>(plugins: &mut Vec<Arg<'static, 'static>>)
where
    T: CompileVariablePlugin,
{
    plugins.push(
        Arg::with_name(T::PLUGIN_NAME)
            .long(T::ARG_NAME)
            .help(T::HELP)
            .takes_value(true)
            .multiple(true),
    );
    plugins.extend(T::additional_args());
}
