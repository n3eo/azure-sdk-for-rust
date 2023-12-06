#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-preview-2020-02")]
pub mod package_preview_2020_02;
#[cfg(all(feature = "package-preview-2020-02", not(feature = "without_tag_import")))]
pub use package_preview_2020_02::*;
#[cfg(feature = "package-2022-06-15")]
pub mod package_2022_06_15;
#[cfg(all(feature = "package-2022-06-15", not(feature = "without_tag_import")))]
pub use package_2022_06_15::*;
#[cfg(feature = "package-2022-04-01")]
pub mod package_2022_04_01;
#[cfg(all(feature = "package-2022-04-01", not(feature = "without_tag_import")))]
pub use package_2022_04_01::*;
#[cfg(feature = "package-2022-02-01")]
pub mod package_2022_02_01;
#[cfg(all(feature = "package-2022-02-01", not(feature = "without_tag_import")))]
pub use package_2022_02_01::*;
#[cfg(feature = "package-2022-01-11")]
pub mod package_2022_01_11;
#[cfg(all(feature = "package-2022-01-11", not(feature = "without_tag_import")))]
pub use package_2022_01_11::*;
