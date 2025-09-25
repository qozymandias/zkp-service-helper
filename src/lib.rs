#![deny(clippy::pedantic, clippy::panic, clippy::expect_used, clippy::unwrap_used)]
#![allow(
    clippy::module_inception,
    clippy::too_many_arguments,
    clippy::used_underscore_binding,
    clippy::similar_names,
    clippy::missing_errors_doc,
    rustdoc::private_intra_doc_links
)]

//! # ZKP Service Helper
//!
//! ZKP Service Helper is a library for interacting with the API endpoints of the `ZKWasm` Playground
//! (ZKP) project. It is based on the `ZKWasm` Service Helper Typescript libray,
//! see [here](https://github.com/DelphinusLab/zkWasm-service-helper/).
//!
//! ## Examples
//!
//! ### Example Query Image (GET request)
//!
//! ```
//! let res  = ZkWasmServiceHelper::new(endpoint).query_image(md5).await?;
//! ```
//!
//! ### Example Add Image (POST request)
//!
//! ```
//! let (buffer, md5) = read_wasm(image_file_path)?;
//!
//! let res = ZkWasmServiceHelper::new(endpoint).setup_image(
//!     md5.clone(),
//!     buffer,
//!     md5.clone(),
//!     user_address.clone(),
//!     format!("ZKP CLI test image {md5}"),
//!     String::new(),
//!     22,
//!     interface::ProvePaymentSrc::Default,
//!     vec![chain_id],
//!     None,
//!     None,
//!     interface::InitialContext::Without,
//!     private_key.clone(),
//! );
//! ```
//! ## More Examples
//!
//! See [src/tests](https://github.com/qozymandias/zkp-service-helper/tree/main/src/tests) for more usage examples.

/// Contains the `ZkWasmServiceHelper` struct which has functions for running API requests.
pub mod helper;

/// Struct library containing at the types required to use ZKP API.
pub mod interface;

#[cfg(test)]
mod tests;
