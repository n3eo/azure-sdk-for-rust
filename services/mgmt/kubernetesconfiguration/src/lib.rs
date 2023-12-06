#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-preview-2023-05")]
pub mod package_preview_2023_05;
#[cfg(all(feature = "package-preview-2023-05", not(feature = "without_tag_import")))]
pub use package_preview_2023_05::*;
#[cfg(feature = "package-preview-2022-04")]
pub mod package_preview_2022_04;
#[cfg(all(feature = "package-preview-2022-04", not(feature = "without_tag_import")))]
pub use package_preview_2022_04::*;
#[cfg(feature = "package-preview-2022-01-15")]
pub mod package_preview_2022_01_15;
#[cfg(all(feature = "package-preview-2022-01-15", not(feature = "without_tag_import")))]
pub use package_preview_2022_01_15::*;
#[cfg(feature = "package-preview-2022-01")]
pub mod package_preview_2022_01;
#[cfg(all(feature = "package-preview-2022-01", not(feature = "without_tag_import")))]
pub use package_preview_2022_01::*;
#[cfg(feature = "package-preview-2021-11")]
pub mod package_preview_2021_11;
#[cfg(all(feature = "package-preview-2021-11", not(feature = "without_tag_import")))]
pub use package_preview_2021_11::*;
