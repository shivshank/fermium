#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// This is bindgen's fault and some day we can hopefully remove it:
// https://github.com/rust-lang/rust-bindgen/issues/1549
#![allow(improper_ctypes)]

// Note(Lokathor): Only one of the following `include!` directives should end up
// being used in any given build.

#[cfg(any(feature = "use_bindgen_bin", feature = "use_bindgen_lib"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(all(
  all(target_os = "windows", target_arch = "x86_64"),
  not(any(feature = "use_bindgen_bin", feature = "use_bindgen_lib"))
))]
include!("bindings_win32_x64.rs");

#[cfg(all(
  all(target_os = "linux", target_arch = "x86_64"),
  not(any(feature = "use_bindgen_bin", feature = "use_bindgen_lib"))
))]
include!("bindings_linux_x64.rs");

#[cfg(all(
  all(target_os = "linux", target_arch = "arm", target_pointer_width = "32"),
  not(any(feature = "use_bindgen_bin", feature = "use_bindgen_lib"))
))]
include!("bindings_linux_arm32.rs");

#[cfg(all(
  all(target_os = "macos", target_arch = "x86_64"),
  not(any(feature = "use_bindgen_bin", feature = "use_bindgen_lib"))
))]
include!("bindings_mac_x64.rs");
