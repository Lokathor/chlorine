#![no_std]

//! Simple conditional compilation picker library.

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
    // we're configuring the macro usage, which will then contain a token tree
    // that turns into either an item or an expression.
    #[cfg(all( $($m,)* not(any($($not),*)) ))]
    $crate::pick!{ @__identity $($tokens)* }

    $crate::pick!{ @__forests [ $($not,)* $($m,)* ] ; $($rest)* }
  };

  // private
  (@__identity $($tokens:tt)*) => {
    $($tokens)*
  };
}

/// Kiel [`pick`], sed en Esperanto. Nur por amuzo!
#[macro_export]
macro_rules! elekti {
  // kun finiĝo "alie"
  ($(se #[cfg($($test:meta),*)] {
      $($if_tokens:tt)*
    })alie+ alie {
      $($else_tokens:tt)*
    }) => {
    $crate::pick!{
      @__forests [ ] ;
      $( [ {$($test),*} {$($if_tokens)*} ], )*
      [ { } {$($else_tokens)*} ],
    }
  };

  // sen finiĝo "alie"
  (se #[cfg($($if_meta:meta),*)] {
      $($if_tokens:tt)*
    } $(alie se #[cfg($($else_meta:meta),*)] {
      $($else_tokens:tt)*
    })*) => {
    $crate::pick!{
      @__forests [ ] ;
      [ {$($if_meta),*} {$($if_tokens)*} ],
      $( [ {$($else_meta),*} {$($else_tokens)*} ], )*
    }
  };
}
