#![cfg(windows)]
#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    clippy::upper_case_acronyms,
    clippy::missing_safety_doc
)]
#![cfg_attr(not(test), no_std)]

#[cfg(target_arch = "x86_64")]
mod bindings_x86_64;
#[cfg(target_arch = "x86_64")]
pub use bindings_x86_64::*;

#[cfg(target_arch = "x86")]
mod bindings_i686;
#[cfg(target_arch = "x86")]
pub use bindings_i686::*;

#[cfg(target_arch = "aarch64")]
mod bindings_aarch64;
#[cfg(target_arch = "aarch64")]
pub use bindings_aarch64::*;
