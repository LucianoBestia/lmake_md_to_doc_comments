// region: lmake_md_to_doc_comments include README.md A //!
//! # lmake_md_to_doc_comments  
//!
//! ***version: 0.5.4  date: 2020-07-15 authors: Luciano Bestia***  
//! **Includes segments of md files into rs files as doc comments.**
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-188-green.svg)](https://github.com/LucianoBestia/lmake_md_to_doc_comments/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-118-blue.svg)](https://github.com/LucianoBestia/lmake_md_to_doc_comments/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-22-purple.svg)](https://github.com/LucianoBestia/lmake_md_to_doc_comments/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/lmake_md_to_doc_comments/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/lmake_md_to_doc_comments/)
//!
//! Includes segments of md files into rs files as doc comments.  
//! From this doc comments `cargo doc` will generated the documentation and auto-completion.  
//! We don't want to manually copy this segments. We want them to be automatically in sync.  
//! We will just run this binary before every `cargo doc` with a script/make.  
//! The `lmake_md_to_doc_comments` binary must be executed in the project root folder where is the cargo.toml file.  
//! It does not work in workspace folder, but every single member project must call it separately.  
//! First it searches all the rs files in src, tests and examples folders.  
//! If they contain the markers, than finds the md file and the named segment and include it as doc comments into the rs file.
//! The markers are always in pairs: start and end. So exactly the content in between is changed.
//! The markers are always comments, so they don't change the code.
//!
//! ## markers
//!
//! In the rs file write these markers:  
//!
//! ```rust
//! 1. // region: lmake_md_to_doc_comments include "filename.md" //! A  
//! 2. // endregion: lmake_md_to_doc_comments include "filename.md" //! A  
//! ```
//!
//! In the md file put markers to mark the segment:  
//!
//! ```markdown
//! 1. [comment]: # (lmake_md_to_doc_comments segment start A)  
//! 2. [comment]: # (lmake_md_to_doc_comments segment end A)  
//! ```
//!
//! The marker must be exclusively in one line. No other text in the same line.  
//! lmake_md_to_doc_comments will delete the old lines between the markers.  
//! It will find the md file and read the content between the markers.  
//! Before each line it will add the doc comment symbol as is defined in the marker.  
//! Finally it will include the new lines as doc comments in the rs file.  
//!
//! ## Tasks in Makefile.toml or script
//!
//! I use `cargo make` to script the repetitive commands sequences.  
//! <https://github.com/sagiegurari/cargo-make>  
//! I copy to doc folder, because this is the GitHub standard.  
//! In `Makefile.toml` add a task like this:  
//!
//! ```toml
//! [tasks.doc]
//! description = "include md to rs, create docs, copy to docs folder"
//! clear = true
//! script = [
//!     "lmake_md_to_doc_comments",
//!     "cargo doc --no-deps --document-private-items",
//!     "\\rsync -avz --delete-after target/doc/*  docs/",
//! ]
//! ```
//!
// endregion: lmake_md_to_doc_comments include README.md A //!

use glob::glob;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use unwrap::unwrap;

#[derive(Debug)]
struct RsMarker {
    pub md_filename: String,
    pub marker_name: String,
    pub comment_symbol: String,
    pub pos_start: usize,
    pub pos_end: usize,
}

#[derive(Debug)]
struct MdSegment {
    pub md_filename: String,
    pub marker_name: String,
    pub pos_start: usize,
    pub pos_end: usize,
    pub text: String,
}

/// find rs files with markers and
/// include md segments
pub fn include_md_segments_to_doc_comments() {
    let mut cache_md_segments = vec![];
    for rs_filename in rs_files().iter() {
        let mut rs_text_content = unwrap!(fs::read_to_string(rs_filename));
        let markers = rs_file_markers(&rs_text_content);
        if !markers.is_empty() {
            for marker in markers.iter().rev() {
                let segment_text = get_md_segments_using_cache(
                    &mut cache_md_segments,
                    &marker.md_filename,
                    &marker.marker_name,
                    &marker.comment_symbol,
                );
                rs_text_content.replace_range(marker.pos_start..marker.pos_end, &segment_text);
            }
            println!("write file: {}", rs_filename);
            unwrap!(fs::write(rs_filename, rs_text_content));
        }
    }
}

/// All rs files in src, tests and examples folders.
/// The current dir must be the project root where the cargo.toml is.
/// In case of workspace, all the members projects must be processed separately.
fn rs_files() -> Vec<String> {
    let mut rs_files = vec![];
    // in Unix shell ** means recursive match through all the subdirectories
    for filename_result in unwrap!(glob("src/**/*.rs")) {
        let filename_pathbuff = unwrap!(filename_result);
        let rs_filename = unwrap!(filename_pathbuff.to_str()).to_string();
        rs_files.push(rs_filename);
    }
    for filename_result in unwrap!(glob("tests/**/*.rs")) {
        let filename_pathbuff = unwrap!(filename_result);
        let rs_filename = unwrap!(filename_pathbuff.to_str()).to_string();
        rs_files.push(rs_filename);
    }
    for filename_result in unwrap!(glob("examples/**/*.rs")) {
        let filename_pathbuff = unwrap!(filename_result);
        let rs_filename = unwrap!(filename_pathbuff.to_str()).to_string();
        rs_files.push(rs_filename);
    }
    //return
    rs_files
}

lazy_static! {
    static ref REGEX_RS_START: Regex =
        Regex::new(r#"(?m)^ *?// region: lmake_md_to_doc_comments include (.*?) (.*?) (.*?)$"#)
            .unwrap();
}
lazy_static! {
    static ref REGEX_RS_END: Regex =
        Regex::new(r#"(?m)^ *?// endregion: lmake_md_to_doc_comments include (.*?) (.*?) (.*?)$"#)
            .unwrap();
}
/// markers in rs files
fn rs_file_markers(rs_text_content: &str) -> Vec<RsMarker> {
    let mut markers = vec![];
    for cap in REGEX_RS_START.captures_iter(rs_text_content) {
        markers.push(RsMarker {
            md_filename: cap[1].to_string(),
            marker_name: cap[2].to_string(),
            comment_symbol: cap[3].to_string(),
            pos_start: unwrap!(cap.get(0)).end() + 1,
            pos_end: 0,
        });
    }
    for cap in REGEX_RS_END.captures_iter(rs_text_content) {
        let marker = unwrap!(markers
            .iter_mut()
            .find(|m| m.md_filename == cap[1] && m.marker_name == cap[2]));
        marker.pos_end = unwrap!(cap.get(0)).start();
    }
    // return
    markers
}

lazy_static! {
    static ref REGEX_MD_START: Regex =
        Regex::new(r#"(?m)^\[comment\]: # \(lmake_md_to_doc_comments segment start (.*?)\)$"#)
            .unwrap();
}
lazy_static! {
    static ref REGEX_MD_END: Regex =
        Regex::new(r#"(?m)^\[comment\]: # \(lmake_md_to_doc_comments segment end (.*?)\)$"#)
            .unwrap();
}

/// The first time it is called 
/// reads the file and extracts all the segments 
/// into a cache vector.
/// Subsequent calls read from the cache.
fn get_md_segments_using_cache(
    cache: &mut Vec<MdSegment>,
    md_filename: &str,
    marker_name: &str,
    comment_symbol: &str,
) -> String {
    // check the cache
    if let Some(_seg) = cache.iter().find(|m| m.md_filename == md_filename) {
        let segment = unwrap!(cache
            .iter()
            .find(|m| m.md_filename == md_filename && m.marker_name == marker_name));
        return segment.text.to_string();
    } else {
        // process the file
        println!("read file: {}", md_filename);
        let md_text_content = unwrap!(fs::read_to_string(md_filename));
        for cap in REGEX_MD_START.captures_iter(&md_text_content) {
            cache.push(MdSegment {
                md_filename: md_filename.to_owned(),
                marker_name: cap[1].to_owned(),
                pos_start: unwrap!(cap.get(0)).end() + 1,
                pos_end: 0,
                text: String::new(),
            });
        }
        for cap in REGEX_MD_END.captures_iter(&md_text_content) {
            let segment = unwrap!(cache
                .iter_mut()
                .find(|m| m.md_filename == md_filename && m.marker_name == cap[1]));
            segment.pos_end = unwrap!(cap.get(0)).start();
            // the segment begins with a comment, so don't include the next empty row
            let mut last_line_was_comment = true;
            for line in md_text_content[segment.pos_start..segment.pos_end].lines() {
                if line.starts_with("[comment]: # (") {
                    // don't include md comments
                    last_line_was_comment = true;
                } else if last_line_was_comment == true && line.is_empty() {
                    // don't include empty line after md comments
                    last_line_was_comment = false;
                } else {
                    last_line_was_comment = false;
                    segment.text.push_str(comment_symbol);
                    if !line.is_empty() {
                        segment.text.push_str(" ");
                    }
                    segment.text.push_str(line);
                    segment.text.push('\n');
                }
            }
        }
        let segment = unwrap!(cache
            .iter()
            .find(|m| m.md_filename == md_filename && m.marker_name == marker_name));
        return segment.text.to_string();
    }
}
