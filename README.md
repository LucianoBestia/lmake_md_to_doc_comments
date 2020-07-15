[comment]: # (lmake_md_to_doc_comments segment start A)

# lmake_md_to_doc_comments  

[comment]: # (lmake_readme cargo.toml data start)
***version: 0.5.5  date: 2020-07-15 authors: Luciano Bestia***  
**Includes segments of md files into rs files as doc comments.**

[comment]: # (lmake_readme cargo.toml data end)

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-188-green.svg)](https://github.com/LucianoBestia/lmake_md_to_doc_comments/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-143-blue.svg)](https://github.com/LucianoBestia/lmake_md_to_doc_comments/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-22-purple.svg)](https://github.com/LucianoBestia/lmake_md_to_doc_comments/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/lmake_md_to_doc_comments/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/lmake_md_to_doc_comments/)

[comment]: # (lmake_lines_of_code end)
[![crates.io](https://meritbadge.herokuapp.com/lmake_md_to_doc_comments)](https://crates.io/crates/lmake_md_to_doc_comments) [![Documentation](https://docs.rs/lmake_md_to_doc_comments/badge.svg)](https://docs.rs/lmake_md_to_doc_comments/) [![crev reviews](
https://web.crev.dev/rust-reviews/badge/crev_count/lmake_md_to_doc_comments.svg
)](https://web.crev.dev/rust-reviews/crate/lmake_md_to_doc_comments/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/lmake_md_to_doc_comments/) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/reader_for_microxml/blob/master/LICENSE)

Includes segments of md files into rs files as doc comments.  
From this doc comments `cargo doc` will generated the documentation and auto-completion.  
We don't want to manually copy this segments. We want them to be automatically in sync.  
We will just run this binary before every `cargo doc` with a script/make.  
The `lmake_md_to_doc_comments` binary must be executed in the project root folder where is the cargo.toml file.  
It does not work in workspace folder, but every single member project must call it separately.  
First it searches all the rs files in src, tests and examples folders.  
If they contain the markers, than finds the md file and the named segment and include it as doc comments into the rs file.  
The markers are always in pairs: start and end. So exactly the content in between is changed.
The markers are always comments, so they don't change the code.  
It works only for files with LF line delimiter. No CR and no CRLF.  

## markers

In the rs file write these markers:  

```rust
1. // region: lmake_md_to_doc_comments include "filename.md" //! A  
2. // endregion: lmake_md_to_doc_comments include "filename.md" //! A  
```

In the md file put markers to mark the segment:  

```markdown
1. [comment]: # (lmake_md_to_doc_comments segment start A)  
2. [comment]: # (lmake_md_to_doc_comments segment end A)  
```

The marker must be exclusively in one line. No other text in the same line.  
lmake_md_to_doc_comments will delete the old lines between the markers.  
It will find the md file and read the content between the markers.  
Before each line it will add the doc comment symbol as is defined in the marker.  
Finally it will include the new lines as doc comments in the rs file.  

## Tasks in Makefile.toml or script

I use `cargo make` to script the repetitive commands sequences.  
<https://github.com/sagiegurari/cargo-make>  
I copy to doc folder, because this is the GitHub standard.  
In `Makefile.toml` add a task like this:  

```toml
[tasks.doc]
description = "include md to rs, create docs, copy to docs folder"
clear = true
script = [
    "lmake_md_to_doc_comments",
    "cargo doc --no-deps --document-private-items",
    "\\rsync -avz --delete-after target/doc/*  docs/",
]
```

[comment]: # (lmake_md_to_doc_comments segment end A)

[comment]: # (lmake_md_to_doc_comments segment start B)

## install

Install from crates.io:  
`cargo install lmake_md_to_doc_comments`  
Then you can use it in every rust project folder.  
No arguments needed to execute the util.  

[comment]: # (lmake_md_to_doc_comments segment end B)

## Development

Documentation:  
<https://lucianobestia.github.io/lmake_md_to_doc_comments/>  
List of prepared make tasks for development: build, run, doc, publish,...  
`clear; cargo make`  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  
