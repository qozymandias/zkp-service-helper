#![deny(clippy::pedantic, clippy::panic, clippy::expect_used, clippy::unwrap_used)]
#![allow(
    clippy::module_inception,
    clippy::too_many_arguments,
    clippy::used_underscore_binding,
    clippy::similar_names,
    clippy::missing_errors_doc
)]

pub mod helper;
pub mod interface;

#[cfg(test)]
mod tests;
