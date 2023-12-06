#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2015-01-privatepreview")]
pub mod package_2015_01_privatepreview;
#[cfg(all(feature = "package-2015-01-privatepreview", not(feature = "without_tag_import")))]
pub use package_2015_01_privatepreview::*;
#[cfg(feature = "package-2015-01-preview")]
pub mod package_2015_01_preview;
#[cfg(all(feature = "package-2015-01-preview", not(feature = "without_tag_import")))]
pub use package_2015_01_preview::*;
