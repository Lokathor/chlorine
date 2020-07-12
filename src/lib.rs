#![no_std]
#![allow(non_camel_case_types)]

//! This just provides the numeric C types, for basic FFI purposes.
//!
//! It's mostly for when you want to support `no_std` without also depending on
//! `libc` or `winapi` (both of which add a few second to the clean build time).

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

pick! {
  if #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))] {
    // pass
  } else {
    compile_error!("This crate is probably wrong because it assumed your target didn't exist. Please file a PR with any updates it needs.")
  }
}

pub use core::ffi::c_void;

pick! {
  if #[cfg(any(
    all(
      target_os = "linux",
      any(
        target_arch = "aarch64",
        target_arch = "arm",
        target_arch = "hexagon",
        target_arch = "powerpc",
        target_arch = "powerpc64",
        target_arch = "s390x",
        target_arch = "riscv64"
      )
    ),
    all(
      target_os = "android",
      any(target_arch = "aarch64", target_arch = "arm")
    ),
    all(target_os = "l4re", target_arch = "x86_64"),
    all(
      target_os = "freebsd",
      any(
        target_arch = "aarch64",
        target_arch = "arm",
        target_arch = "powerpc",
        target_arch = "powerpc64"
      )
    ),
    all(
      target_os = "netbsd",
      any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc")
    ),
    all(target_os = "openbsd", target_arch = "aarch64"),
    all(
      target_os = "vxworks",
      any(
        target_arch = "aarch64",
        target_arch = "arm",
        target_arch = "powerpc64",
        target_arch = "powerpc"
      )
    ),
    all(target_os = "fuchsia", target_arch = "aarch64")
  ))] {
    pub type c_char = u8;
  } else {
    pub type c_char = i8;
  }
}

pub type c_schar = i8;

pub type c_uchar = u8;

pub type c_short = i16;

pub type c_ushort = u16;

pub type c_int = i32;

pub type c_uint = u32;

pick! {
  if #[cfg(any(windows, target_pointer_width = "32"))] {
    pub type c_long = i32;
    pub type c_ulong = u32;
  } else {
    pub type c_long = i64;
    pub type c_ulong = u64;
  }
}

pub type c_longlong = i64;

pub type c_ulonglong = u64;

pub type c_float = f32;

pub type c_double = f64;
