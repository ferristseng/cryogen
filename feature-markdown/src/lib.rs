extern crate clap;
extern crate hoedown;
extern crate lib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use clap::{Arg, ArgMatches};
use hoedown::{Extension, Markdown};
use hoedown::renderer::Render;
use hoedown::renderer::html::{Flags, Html};
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::fs::File;

use lib::CompileVariablePlugin;


/// Value written to Tera context.
///
#[derive(Serialize)]
pub struct RenderedMarkdown {
    metadata: Option<serde_yaml::Value>,
    html: String,
}

impl RenderedMarkdown {
    fn new(html: String) -> RenderedMarkdown {
        RenderedMarkdown {
            html: html,
            metadata: None,
        }
    }

    fn with_metadata(html: String, metadata: serde_yaml::Value) -> RenderedMarkdown {
        RenderedMarkdown {
            html: html,
            metadata: Some(metadata),
        }
    }
}


fn contains_metadata_prelude(buf: &[u8]) -> bool {
    buf.len() >= 4 && &buf[0..4] != &[b'-', b'-', b'-', b'\n'] && buf.len() >= 5 &&
    &buf[0..5] != &[b'-', b'-', b'-', b'\r', b'\n']
}

fn contains_metadata_epilogue(buf: &[u8], bytes_read: usize) -> bool {
    let previous_slice_start = buf.len() - bytes_read;

    bytes_read > 3 && &buf[previous_slice_start..previous_slice_start + 3] == &[b'-', b'-', b'-'] &&
    buf[previous_slice_start + 3..].iter().all(|b| (*b as char).is_whitespace())
}

/// Read the metadata fence at the beginning of the surrounded by ---.
///
fn read_header(file: &mut File) -> Result<Vec<u8>, String> {
    let mut bytes_read = 0u64;
    let mut reader = BufReader::new(file);
    let mut data = Vec::new();

    match reader.read_until(b'\n', &mut data) {
        Err(e) => return Err(format!("failed to read file header: {:?}", e)),
        Ok(bytes_num) => bytes_read += bytes_num as u64,
    }

    if contains_metadata_prelude(&data[..]) {
        Err("expected markdown metadata".to_string())
    } else {
        // Clear the metadata start marker.
        //
        data.clear();

        loop {
            match reader.read_until(b'\n', &mut data) {
                Ok(0) => break,
                Ok(bytes_num) if contains_metadata_epilogue(&data, bytes_num) => {
                    let len = data.len();

                    data.truncate(len - bytes_num);

                    // The BufReader can read past the start of the markdown document, so we have to
                    // find the beginning.
                    //
                    return match reader.seek(SeekFrom::Start(bytes_read + bytes_num as u64)) {
                        Ok(_) => Ok(data),
                        Err(e) => Err(format!("failed to seek to start of markdown: {:?}", e)),
                    };
                }
                Ok(bytes_num) => bytes_read += bytes_num as u64,
                Err(e) => {
                    println!("{:?}", e);
                    return Err(format!("encountered io error while reading markdown metadata: \
                                        {:?}",
                                       e));
                }
            }
        }

        Err("file ended before markdown metadata ended".to_string())
    }
}


pub struct MarkdownPlugin {
    yaml_metadata: bool,
    autolink: bool,
    disable_indented_code: bool,
    fenced_code: bool,
    footnotes: bool,
    highlight: bool,
    math: bool,
    math_explicit: bool,
    no_intra_emphasis: bool,
    quote: bool,
    space_headers: bool,
    strikethrough: bool,
    superscript: bool,
    tables: bool,
    underline: bool,
}

impl CompileVariablePlugin for MarkdownPlugin {
    type RenderValue = RenderedMarkdown;

    fn plugin_name() -> &'static str {
        "markdown"
    }

    fn arg_full_name() -> &'static str {
        "markdown"
    }

    fn arg_help() -> &'static str {
        "Assign variable to contents of a Markdown file"
    }

    fn additional_args() -> Vec<Arg<'static, 'static>> {
        vec![Arg::with_name("markdown-yaml-metadata").long("markdown-yaml-metadata"),
             Arg::with_name("markdown-autolink").long("markdown-autolink"),
             Arg::with_name("markdown-disable-indented-code")
                 .long("markdown-disable-indented-code"),
             Arg::with_name("markdown-fenced-code").long("markdown-fenced-code"),
             Arg::with_name("markdown-footnotes").long("markdown-footnotes"),
             Arg::with_name("markdown-highlight").long("markdown-highlight"),
             Arg::with_name("markdown-math").long("markdown-math"),
             Arg::with_name("markdown-math-explicit").long("markdown-math-explicit"),
             Arg::with_name("markdown-no-intra-emphasis").long("markdown-no-intra-emphasis"),
             Arg::with_name("markdown-quote").long("markdown-quote"),
             Arg::with_name("markdown-space-headers").long("markdown-space-headers"),
             Arg::with_name("markdown-strikethrough").long("markdown-strikethrough"),
             Arg::with_name("markdown-superscript").long("markdown-superscript"),
             Arg::with_name("markdown-tables").long("markdown-tables"),
             Arg::with_name("markdown-underline").long("markdown-underline"),
             Arg::with_name("markdown-sane-defaults")
                 .long("markdown-sane-defaults")
                 .conflicts_with_all(&["markdown-yaml-metadata",
                                       "markdown-fenced-code",
                                       "markdown-footnotes",
                                       "markdown-no-intra-emphasis",
                                       "markdown-strikethrough"])]
    }

    fn from_args<'a>(args: &'a ArgMatches<'a>) -> MarkdownPlugin {
        MarkdownPlugin {
            yaml_metadata: args.is_present("markdown-yaml-metadata") ||
                           args.is_present("markdown-sane-defaults"),
            autolink: args.is_present("markdown-autolink"),
            disable_indented_code: args.is_present("markdown-disable-indented-code"),
            fenced_code: args.is_present("markdown-fenced-code") ||
                         args.is_present("markdown-sane-defaults"),
            footnotes: args.is_present("markdown-footnotes") ||
                       args.is_present("markdown-sane-defaults"),
            highlight: args.is_present("markdown-highlight"),
            math: args.is_present("markdown-math"),
            math_explicit: args.is_present("markdown-math-explicit"),
            no_intra_emphasis: args.is_present("markdown-no-intra-emphasis") ||
                               args.is_present("markdown-sane-defaults"),
            quote: args.is_present("markdown-quote"),
            space_headers: args.is_present("markdown-space-headers"),
            strikethrough: args.is_present("markdown-strikethrough") ||
                           args.is_present("markdown-sane-defaults"),
            superscript: args.is_present("markdown-superscript"),
            tables: args.is_present("markdown-tables"),
            underline: args.is_present("markdown-underline"),
        }
    }

    fn read_file(&self, mut file: File) -> Result<RenderedMarkdown, String> {
        let mut extensions = Extension::empty();

        if self.autolink {
            extensions.insert(hoedown::AUTOLINK);
        }

        if self.disable_indented_code {
            extensions.insert(hoedown::DISABLE_INDENTED_CODE);
        }

        if self.fenced_code {
            extensions.insert(hoedown::FENCED_CODE);
        }

        if self.footnotes {
            extensions.insert(hoedown::FOOTNOTES);
        }

        if self.highlight {
            extensions.insert(hoedown::HIGHLIGHT);
        }

        if self.math {
            extensions.insert(hoedown::MATH);
        }

        if self.math_explicit {
            extensions.insert(hoedown::MATH_EXPLICIT);
        }

        if self.no_intra_emphasis {
            extensions.insert(hoedown::NO_INTRA_EMPHASIS);
        }

        if self.quote {
            extensions.insert(hoedown::QUOTE);
        }

        if self.space_headers {
            extensions.insert(hoedown::SPACE_HEADERS);
        }

        if self.strikethrough {
            extensions.insert(hoedown::STRIKETHROUGH);
        }

        if self.superscript {
            extensions.insert(hoedown::SUPERSCRIPT);
        }

        if self.tables {
            extensions.insert(hoedown::TABLES);
        }

        if self.underline {
            extensions.insert(hoedown::UNDERLINE);
        }

        let metadata = if self.yaml_metadata {
            let metadata_bytes = try!(read_header(&mut file));

            match serde_yaml::from_slice(&metadata_bytes) {
                Ok(metadata) => Some(metadata),
                Err(e) => return Err(format!("failed to parse markdown metadata: {:?}", e)),
            }
        } else {
            None
        };

        let markdown = Markdown::read_from(file).extensions(extensions);
        let mut html = Html::new(Flags::empty(), 0);
        let output = html.render(&markdown);

        match output.to_str() {
            Ok(html) => {
                Ok(metadata.map_or_else(|| RenderedMarkdown::new(html.to_string()), |metadata| {
                    RenderedMarkdown::with_metadata(html.to_string(), metadata)
                }))
            }
            Err(e) => Err(format!("error generating html from markdown: {:?}", e)),
        }
    }
}
