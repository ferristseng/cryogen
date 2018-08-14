extern crate clap;
#[macro_use]
extern crate cryogen_prelude;
extern crate pulldown_cmark;

use clap::{Arg, ArgMatches};
use cryogen_prelude::{CompileVariablePlugin, markdown::{read_header, RenderedMarkdown}};
use pulldown_cmark::{html, Options, Parser};
use std::io::Read;
use std::fs::File;

const MD_YAML_METADATA: &'static str = "markdown-yaml-metadata";
const MD_FOOTNOTES: &'static str = "markdown-footnotes";
const MD_TABLES: &'static str = "markdown-tables";

pub struct MarkdownPlugin {
    yaml_metadata: bool,
    footnotes: bool,
    tables: bool,
}

impl CompileVariablePlugin for MarkdownPlugin {
    type RenderValue = RenderedMarkdown;

    const PLUGIN_NAME: &'static str = "markdown";

    const ARG_NAME: &'static str = "markdown";

    const HELP: &'static str = "Assign variable to contents of a Markdown file";

    #[inline]
    fn additional_args() -> Vec<Arg<'static, 'static>> {
        args! {
            MD_YAML_METADATA ["Enable YAML metadata block"];
            MD_FOOTNOTES     ["Enable footnotes"];
            MD_TABLES        ["Enable tables"];
        }
    }

    #[inline]
    fn from_args<'a>(args: &'a ArgMatches<'a>) -> MarkdownPlugin {
        MarkdownPlugin {
            yaml_metadata: args.is_present(MD_YAML_METADATA),
            footnotes: args.is_present(MD_YAML_METADATA),
            tables: args.is_present(MD_TABLES),
        }
    }

    fn read_arg(&self, path: &str) -> Result<RenderedMarkdown, String> {
        let mut opts = Options::empty();

        if self.footnotes {
            opts.insert(pulldown_cmark::OPTION_ENABLE_FOOTNOTES);
        }

        if self.tables {
            opts.insert(pulldown_cmark::OPTION_ENABLE_TABLES);
        }

        let mut buffer = String::new();

        File::open(path)
            .map_err(|e| e.to_string())?
            .read_to_string(&mut buffer)
            .map_err(|e| e.to_string())?;

        let mut view = &buffer[..];
        let metadata = if self.yaml_metadata {
            match read_header(view.as_bytes()) {
                Ok((metadata, md_start)) => {
                    view = &view[md_start..];
                    metadata
                }
                Err(e) => return Err(format!("failed to parse markdown metadata: {:?}", e)),
            }
        } else {
            None
        };

        let mut html = String::new();
        let parser = Parser::new_ext(&view[..], opts);

        html::push_html(&mut html, parser);

        Ok(RenderedMarkdown::new(metadata, html.to_string()))
    }
}
