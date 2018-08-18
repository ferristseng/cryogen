extern crate clap;
extern crate serde;
#[cfg(feature = "markdown")]
extern crate serde_yaml;
#[cfg(feature = "markdown")]
#[macro_use]
extern crate serde_derive;

use clap::{Arg, ArgMatches};
use serde::Serialize;
use std::{cmp, borrow::Cow, io::{self, Read}};

#[cfg(feature = "markdown")]
pub mod markdown;

/// Macro to define very simple lists of clap arguments.
///
#[macro_export]
macro_rules! args {
    ( $($name: ident [$help: expr]);*; ) => {
        vec![
            $(
                Arg::with_name($name).long($name).help($help),
            )*
        ]
    };
}

/// Defines a mapping between a template variable, and the file to
/// populate it.
///
#[derive(Debug)]
pub struct VarMapping<'a> {
    var_name: &'a str,
    arg_value: &'a str,
}

impl<'a> VarMapping<'a> {
    /// Unpacks a mapped variable string `<var_name>:<arg_value>`.
    ///
    pub fn from_str(s: &'a str) -> Result<VarMapping<'a>, String> {
        let mut splits = s.splitn(2, ":");
        let var_name = if let Some(var_name) = splits.next() {
            var_name
        } else {
            return Err(format!("Expected a variable name to bind to in ({})", s));
        };
        let arg_value = if let Some(arg_value) = splits.next() {
            arg_value
        } else {
            return Err(format!(
                "Expected a value to bind to ({}) in ({})",
                var_name, s
            ));
        };

        Ok(VarMapping {
            var_name,
            arg_value,
        })
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

/// How to interpret the value of an argument.
///
pub enum Interpretation {
    Raw,
    Path,
}

/// A source can either be treated like a holder of a String value,
/// or a readable stream.
///
pub enum Source<'a, R>
where
    R: Read,
{
    /// A raw string file.
    ///
    Raw(&'a str, usize),

    /// A local or remote file.
    ///
    File(R),
}

impl<'a, R> Source<'a, R>
where
    R: Read,
{
    /// Consumes the source, and reads the entire value into a string.
    ///
    pub fn consume(self) -> Result<Cow<'a, str>, String> {
        match self {
            Source::Raw(raw, _) => Ok(Cow::Borrowed(raw)),
            Source::File(mut reader) => {
                let mut buf = String::new();

                reader.read_to_string(&mut buf).map_err(|e| e.to_string())?;

                Ok(Cow::Owned(buf))
            }
        }
    }
}

impl<'a, R> Read for Source<'a, R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        match self {
            // For raw strings, the raw bytes can just be copied
            // to the buffer. An index value is tracked to only copy
            // unseen bytes.
            &mut Source::Raw(raw, ref mut index) => {
                let current = *index;
                let slice = &raw.as_bytes()[current..];
                let copy_num = cmp::min(buf.len(), slice.len());

                &buf[..copy_num].copy_from_slice(&slice[..copy_num]);
                *index = current + copy_num;

                Ok(copy_num)
            }
            // For wrappers around file handlers, the data can just
            // be read directly from the buffer.
            &mut Source::File(ref mut reader) => reader.read(buf),
        }
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
    const PLUGIN_NAME: &'static str;

    /// The name of the argument that provides the variable name to file mapping.
    ///
    const ARG_NAME: &'static str;

    /// How to interpret a supplied argument.
    ///
    const ARG_INTERPRETATION: Interpretation;

    /// The help string to display.
    ///
    const HELP: &'static str;

    /// Optional arguments to supply. These should be prefixed to avoid namespace clashes.
    ///
    fn additional_args() -> Vec<Arg<'static, 'static>>;

    /// Constructor for building the plugin from the supplied command line arguments.
    ///
    fn from_args<'a>(args: &'a ArgMatches<'a>) -> Self;

    /// Reads the source data, and parses it into a value that can be rendered.
    ///
    fn read<'a, R>(&self, src: Source<'a, R>) -> Result<Self::RenderValue, String>
    where
        R: Read;
}
