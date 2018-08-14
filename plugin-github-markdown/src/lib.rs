extern crate clap;
extern crate comrak;
#[macro_use]
extern crate cryogen_prelude;

use clap::{Arg, ArgMatches};
use comrak::{markdown_to_html, ComrakOptions};
use cryogen_prelude::{CompileVariablePlugin, Interpretation, Source,
                      markdown::{read_header, RenderedMarkdown}};
use std::io::Read;

const GFM_YAML_METADATA: &'static str = "gfm-yaml-metadata";
const GFM_HARDBREAKS: &'static str = "gfm-hardbreaks";
const GFM_SMART_PUNCTUATION: &'static str = "gfm-smart-puncutation";
const GFM_PRE_LANG: &'static str = "gfm-pre-lang";
const GFM_SAFE: &'static str = "gfm-safe";
const GFM_STRIKETHROUGH: &'static str = "gfm-strikethrough";
const GFM_TAG_FILTER: &'static str = "gfm-tag-filter";
const GFM_TABLES: &'static str = "gfm-tables";
const GFM_AUTOLINK: &'static str = "gfm-autolink";
const GFM_TASKLISTS: &'static str = "gfm-tasklists";
const GFM_SUPERSCRIPT: &'static str = "gfm-superscript";
const GFM_FOOTNOTES: &'static str = "gfm-footnotes";

pub struct GithubMarkdownPlugin {
    yaml_metadata: bool,
    hardbreaks: bool,
    smart_punctuation: bool,
    github_pre_lang: bool,
    safe: bool,
    strikethrough: bool,
    tag_filter: bool,
    tables: bool,
    autolink: bool,
    tasklists: bool,
    superscript: bool,
    footnotes: bool,
}

impl CompileVariablePlugin for GithubMarkdownPlugin {
    type RenderValue = RenderedMarkdown;

    const PLUGIN_NAME: &'static str = "gfm";

    const ARG_NAME: &'static str = "gfm";

    const ARG_INTERPRETATION: Interpretation = Interpretation::Path;

    const HELP: &'static str = "Assign variable to contents of a GitHub-Flavored Markdown file";

    #[inline]
    fn additional_args() -> Vec<Arg<'static, 'static>> {
        args! {
            GFM_YAML_METADATA       ["Enable YAML metadata block"];
            GFM_HARDBREAKS          ["Convert soft breaks to hard breaks in output"];
            GFM_SMART_PUNCTUATION   ["Convert punctuation to unicode equivalents"];
            GFM_PRE_LANG            ["Enable GitHub pre blocks"];
            GFM_SAFE                ["Disable rendering raw HTML and dangerous links"];
            GFM_STRIKETHROUGH       ["Enable strikethrough syntax"];
            GFM_TAG_FILTER          ["Disable some raw HTML tags"];
            GFM_TABLES              ["Enable tables"];
            GFM_AUTOLINK            ["Enable autolinks"];
            GFM_TASKLISTS           ["Enable task lists"];
            GFM_SUPERSCRIPT         ["Enable superscript syntax"];
            GFM_FOOTNOTES           ["Enable footnotes"];
        }
    }

    #[inline]
    fn from_args<'a>(args: &'a ArgMatches<'a>) -> GithubMarkdownPlugin {
        GithubMarkdownPlugin {
            yaml_metadata: args.is_present(GFM_YAML_METADATA),
            hardbreaks: args.is_present(GFM_HARDBREAKS),
            smart_punctuation: args.is_present(GFM_SMART_PUNCTUATION),
            github_pre_lang: args.is_present(GFM_PRE_LANG),
            safe: args.is_present(GFM_SAFE),
            strikethrough: args.is_present(GFM_STRIKETHROUGH),
            tag_filter: args.is_present(GFM_TAG_FILTER),
            tables: args.is_present(GFM_TABLES),
            autolink: args.is_present(GFM_AUTOLINK),
            tasklists: args.is_present(GFM_TASKLISTS),
            superscript: args.is_present(GFM_SUPERSCRIPT),
            footnotes: args.is_present(GFM_FOOTNOTES),
        }
    }

    fn read<'a, R>(&self, src: Source<'a, R>) -> Result<RenderedMarkdown, String>
    where
        R: Read,
    {
        let opts = ComrakOptions {
            hardbreaks: self.hardbreaks,
            smart: self.smart_punctuation,
            github_pre_lang: self.github_pre_lang,
            safe: self.safe,
            ext_strikethrough: self.strikethrough,
            ext_tagfilter: self.tag_filter,
            ext_table: self.tables,
            ext_autolink: self.autolink,
            ext_tasklist: self.tasklists,
            ext_superscript: self.superscript,
            ext_footnotes: self.footnotes,
            ..ComrakOptions::default()
        };

        let data = src.consume()?;
        let mut view = &data[..];
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

        let html = markdown_to_html(&view, &opts);

        Ok(RenderedMarkdown::new(metadata, html.to_string()))
    }
}
