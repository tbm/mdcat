// Copyright 2018-2020 Sebastian Wiesner <sebastian@swsnr.de>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![deny(warnings, missing_docs, clippy::all)]

use pretty_assertions::assert_eq;
use pulldown_cmark::{Options, Parser};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use syntect::parsing::SyntaxSet;

fn format_ansi_to_html(markdown: &str) -> String {
    let child = Command::new("ansi2html")
        .arg("--input-encoding")
        .arg("utf8")
        .arg("--output-encoding")
        .arg("utf8")
        .arg("--markup-lines")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start ansi2html");
    {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(markdown, options);
        mdcat::push_tty(
            &mdcat::Settings {
                terminal_capabilities: mdcat::TerminalCapabilities::ansi(),
                terminal_size: mdcat::TerminalSize::default(),
                resource_access: mdcat::ResourceAccess::LocalOnly,
                syntax_set: SyntaxSet::load_defaults_newlines(),
            },
            &mut child.stdin.unwrap(),
            &std::env::current_dir().expect("No working directory"),
            parser,
        )
        .expect("Formatting failed")
    }
    let mut buffer = Vec::new();
    child
        .stdout
        .unwrap()
        .read_to_end(&mut buffer)
        .expect("Failed to read");

    String::from_utf8(buffer).expect("Failed to convert from bytes")
}

fn test_directory() -> PathBuf {
    Path::new(file!())
        .parent()
        .expect("Failed to get parent directory")
        .join("formatting")
}

fn read_file(basename: &str, extension: &str) -> String {
    let mut contents = String::new();
    let path = test_directory().join(basename).with_extension(extension);
    File::open(path)
        .and_then(|mut source| source.read_to_string(&mut contents))
        .expect("Failed to read test file");
    contents
}

fn assert_formats_to_expected_html(basename: &str) {
    let markdown = read_file(basename, "md");
    let actual_html = format_ansi_to_html(&markdown);

    let target = test_directory()
        .join(basename)
        .with_extension("actual.html");
    File::create(target)
        .and_then(|mut f| f.write_all(actual_html.as_bytes()))
        .expect("Failed to write actual HTML");

    let expected_html = read_file(basename, "expected.html");
    assert_eq!(actual_html, expected_html, "Different format produced");
}

macro_rules! test_compare_html(
    ($testname:ident) => (
        #[test]
        fn $testname() {
            crate::assert_formats_to_expected_html(stringify!($testname));
        }
    )
);

mod formatting {
    mod html {
        test_compare_html!(block_quote_and_ruler);
        test_compare_html!(code_blocks);
        test_compare_html!(headers_and_paragraphs);
        test_compare_html!(inline_formatting);
        test_compare_html!(just_a_line);
        test_compare_html!(links);
        test_compare_html!(lists);
        test_compare_html!(tasklist);
    }
}
