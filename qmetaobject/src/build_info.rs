//! Functions for retrieving info about the version of Qt
//!
//! When [`qttypes`][] links against the Qt library, it exports metadata related to the build /
//! link process to any crate that directly depends on it. This data is not made available to
//! crates that transitively depend on `qttypes`, which applies to crates that depend on
//! `qmetaobject` (since `qttypes` is a direct dependency of `qmetaobject`).
//!
//! To fix this, `qmetaobject`'s build script reads the metadata from `qttypes`'s build script
//! and makes it available at compile-time to the library. This module provides a set of
//! functions for dependents of `qmetaobject` to access this data as well.
//!
//! [qttypes][https://docs.rs/qttypes/latest/qttypes]

pub fn qt_version() -> Result<&'static str, &'static str> {
    const ERR_MSG: &str = "A value for QT_VERSION was not set at build time";
    match option_env!("QT_VERSION") {
        Some(value) => Ok(value),
        None => Err(ERR_MSG)
    }
}

pub fn qt_library_path() -> Result<&'static str, &'static str> {
    const ERR_MSG: &str = "A value for QT_LIBRARY_PATH was not set at build time";
    match option_env!("QT_LIBRARY_PATH") {
        Some(value) => Ok(value),
        None => Err(ERR_MSG)
    }
}

pub fn qt_include_path() -> Result<&'static str, &'static str> {
    const ERR_MSG: &str = "A value for QT_INCLUDE_PATH was not set at build time";
    match option_env!("QT_INCLUDE_PATH") {
        Some(value) => Ok(value),
        None => Err(ERR_MSG)
    }
}

pub fn qt_compile_flags() -> Result<&'static str, &'static str> {
    const ERR_MSG: &str = "A value for QT_COMPILE_FLAGS was not set at build time";
    match option_env!("QT_COMPILE_FLAGS") {
        Some(value) => Ok(value),
        None => Err(ERR_MSG)
    }
}
