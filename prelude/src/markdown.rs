use serde_yaml;

/// The Markdown metadata holder is a YAML object.
///
pub type MarkdownMetadata = serde_yaml::Value;

const DIVIDER: &[u8] = &[b'-', b'-', b'-'];
const NEWLINE: &[u8] = &[b'\n'];
const NEWLINE_ALT: &[u8] = &[b'\r', b'\n'];

/// Tries to read a YAML block from the beginning of an input buffer.
///
pub fn read_header(mut buf: &[u8]) -> Result<(Option<MarkdownMetadata>, usize), String> {
    let mut total_read = 0;
    // Check to see if the buffer starts with the YAML block divider.
    //
    if !buf.starts_with(DIVIDER) {
        return Ok((None, 0));
    }

    total_read += DIVIDER.len();

    buf = &buf[DIVIDER.len()..];

    // Check to see if the DIVIDER is followed by a new line.
    //
    if buf.starts_with(NEWLINE_ALT) {
        total_read += NEWLINE_ALT.len();

        buf = &buf[NEWLINE_ALT.len()..];
    } else if buf.starts_with(NEWLINE) {
        total_read += NEWLINE.len();

        buf = &buf[NEWLINE.len()..];
    } else {
        return Ok((None, 0));
    }

    let mut index = 0;
    let mut yaml_end = None;
    for part in buf.windows(DIVIDER.len()) {
        if part == DIVIDER {
            let after_index = index + DIVIDER.len();

            // The YAML block could end at the EOF.
            //
            // Usually the YAML block will end before the
            // end of the file, and a newline will signify
            // the start of the Markdown code.
            //
            if after_index > buf.len() {
                total_read += after_index;

                yaml_end = Some(index);

                break;
            }

            if buf[after_index..].starts_with(NEWLINE) {
                total_read += after_index + NEWLINE.len();

                yaml_end = Some(index);

                break;
            }
            if buf[after_index..].starts_with(NEWLINE_ALT) {
                total_read += after_index + NEWLINE_ALT.len();

                yaml_end = Some(index);

                break;
            }
        }

        index += 1;
    }

    match yaml_end {
        Some(0) => Ok((None, total_read)),
        Some(end) => serde_yaml::from_slice(&buf[..end])
            .map(|meta| (Some(meta), total_read))
            .map_err(|e| e.to_string()),
        None => Ok((None, 0)),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_empty_prelude_recognized() {
        let yaml_block = b"---\n---\n";

        assert!(super::read_header(yaml_block).is_ok())
    }

    #[test]
    fn test_empty_string() {
        assert!(super::read_header(b"").is_ok());
    }

    #[test]
    fn test_incomplete_block() {
        assert!(super::read_header(b"---").is_ok());
    }

    #[test]
    fn test_valid_block() {
        assert!(
            super::read_header(b"---\ntitle: ABC\n---\n")
                .map(|(res, _)| res.is_some())
                .unwrap_or(false)
        );
    }
}
