// region: lmake_md_to_doc_comments include README.md A //!
//! # lmake_md_to_doc_comments  
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

// region: lmake_md_to_doc_comments include README.md B //!
// endregion: lmake_md_to_doc_comments include README.md B //!

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Red, Yellow};
//use ansi_term::Style;
use clap::App;
use std::env;

#[allow(clippy::print_stdout, clippy::integer_arithmetic)]
/// The program starts here. No arguments.
fn main() {
    // this function is different for Windows and for Linux.
    // Look at the code of this function (2 variations).
    enable_ansi_support();

    // define the CLI input line parameters using the clap library
    let _matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    println!("---- {} start ----", Green.paint(env!("CARGO_PKG_NAME")));
    lmake_md_to_doc_comments::include_md_segments_to_doc_comments();
    println!("---- {} end ----", Green.paint(env!("CARGO_PKG_NAME")));
}

// region: different function code for Linux and Windows
#[cfg(target_family = "windows")]
/// only on windows "enable ansi support" must be called
pub fn enable_ansi_support() {
    let _enabled = ansi_term::enable_ansi_support();
}

#[cfg(target_family = "unix")]
//on Linux "enable ansi support" must not be called
pub fn enable_ansi_support() {
    // do nothing
}
// endregion
