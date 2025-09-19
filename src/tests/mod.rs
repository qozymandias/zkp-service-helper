#![allow(clippy::expect_used)]

use super::helper::ZkWasmServiceHelper;

mod archive;
mod queries;
mod tasks;
mod util;

static CONFIG: once_cell::sync::Lazy<util::TestConfig> = once_cell::sync::Lazy::new(util::TestConfig::init);
static ZKH: once_cell::sync::Lazy<ZkWasmServiceHelper> =
    once_cell::sync::Lazy::new(|| ZkWasmServiceHelper::new(CONFIG.details.server_url.clone()));

#[macro_export]
macro_rules! run_test {
        ($f:expr $(, $arg:expr)* $(,)?) => {
            {
                let fut = $f(&ZKH $(, $arg)*);
                util::check_and_print(fut.await)
            }
        };
    }

#[macro_export]
macro_rules! run_archive_test {
        ($f:expr $(, $arg:expr)* $(,)?) => {
            {
                let zkh = $crate::helper::ZkWasmServiceHelper::new(CONFIG.archive.server_url.clone());
                let fut = $f(&zkh $(, $arg)*);
                util::check_and_print(fut.await)
            }
        };
    }
