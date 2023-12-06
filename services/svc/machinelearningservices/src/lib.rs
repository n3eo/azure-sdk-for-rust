#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2019-09-preview")]
pub mod package_2019_09_preview;
#[cfg(all(feature = "package-2019-09-preview", not(feature = "without_tag_import")))]
pub use package_2019_09_preview::*;
#[cfg(feature = "package-2019-08-preview")]
pub mod package_2019_08_preview;
#[cfg(all(feature = "package-2019-08-preview", not(feature = "without_tag_import")))]
pub use package_2019_08_preview::*;
