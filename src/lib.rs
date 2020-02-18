#![no_std]
#![allow(non_camel_case_types)]

//! This just provides the numeric C types, for basic FFI purposes.
//!
//! * If you're on Windows it just gives the necessary aliases.
//! * If you're on not-Windows it re-exports from `libc`.
//!
//! You might think "why not just always link to `libc`?", but on Windows that
//! means that your resulting binary is just a little less portable for no
//! reason. Either the user's machine will need to have the visual studio
//! redistributable DLL installed or you'll have to build the binary with a
//! static linked CRT configured. Either of these options are silly if all that
//! you want is some type declarations and you're not even calling any
//! functions.
//!
//! ### FAQ
//!
//! * **Question:** Lokathor, aren't you just being totally crazy?
//!   * Yes.
//! * **Question:** What do you do about `wasm32-unknown-unknown`? Unlike
//!   `-wasi` and `-emscripten`, it has no `libc` support, so having "C types"
//!   doesn't make any sense, right?
//!   * Correct, there is no `libc`. In this situation, essentially no
//!     definition is actually _wrong_, so all that matters is that _anything_
//!     is provided. For simplicity sake, you get the same definitions as you
//!     get on a `windows` target. If a `libc` is created in the future for that
//!     target, the types will be adjusted to match if necessary.

/// Does all our conditional compilation selection.
#[macro_export]
macro_rules! pick {
  // with a trailing else
  ($(if #[cfg($($test:meta),*)] {
      $($if_tokens:tt)*
    })else+ else {
      $($else_tokens:tt)*
    }) => {
    $crate::pick!{
      @__forests [ ] ;
      $( [ {$($test),*} {$($if_tokens)*} ], )*
      [ { } {$($else_tokens)*} ],
    }
  };

  // without a trailing else
  (if #[cfg($($if_meta:meta),*)] {
      $($if_tokens:tt)*
    } $(else if #[cfg($($else_meta:meta),*)] {
      $($else_tokens:tt)*
    })*) => {
    $crate::pick!{
      @__forests [ ] ;
      [ {$($if_meta),*} {$($if_tokens)*} ],
      $( [ {$($else_meta),*} {$($else_tokens)*} ], )*
    }
  };

  // private
  (@__forests [$($not:meta,)*];) => {
    /* halt expansion */
  };

  // private
  (@__forests [$($not:meta,)*]; [{$($m:meta),*} {$($tokens:tt)*}], $($rest:tt)*) => {
    // This "one weird trick" works because you can't apply a `cfg` to an
    // expression, only an item or a block, but a macro usage is an item, so
    // we're configuring the macro usage, which (if configured in) will then
    // contain a token tree that turns into either an item or an expression.
    #[cfg(all( $($m,)* not(any($($not),*)) ))]
    $crate::pick!{ @__identity $($tokens)* }

    $crate::pick!{ @__forests [ $($not,)* $($m,)* ] ; $($rest)* }
  };

  // private
  (@__identity $($tokens:tt)*) => {
    $($tokens)*
  };
}

pub use core::ffi::c_void;

pick! {
  if #[cfg(any(
    windows,
    all(
      target_arch = "wasm32",
      not(any(target_env = "wasi", target_env = "emscripten"))
    )
  ))] {
    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i32;
    pub type c_ulong = u32;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
    pub type c_float = f32;
    pub type c_double = f64;
  } else {
    pub use libc::{
      c_char,
      c_schar,
      c_uchar,
      c_short,
      c_ushort,
      c_int,
      c_uint,
      c_long,
      c_ulong,
      c_longlong,
      c_ulonglong,
      c_float,
      c_double,
    };
  }
}
