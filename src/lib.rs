#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]
//!
//! ## API
//!
//! The following functions are defined in this package:
//!
#![doc = include_str!(concat!(env!("OUT_DIR"), "/rhai-url-docs.md"))]
#![doc = include_str!("../docs/highlight.html")]

use rhai::def_package;
use rhai::plugin::*;

pub(crate) mod url;

def_package! {
    /// Package for url operations.
    pub UrlPackage(lib) {
       combine_with_exported_module!(lib, "rhai_url", url::url_module);
    }
}
