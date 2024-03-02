//! Macro and common structs to play with [`smallvec`].
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(missing_docs)]

#[cfg(feature = "either")]
use either::Either;

#[cfg(feature = "either")]
#[cfg_attr(docsrs, doc(cfg(feature = "either")))]
pub use either;

#[doc(hidden)]
pub mod __private {
  #[cfg(feature = "either")]
  pub use either;
  pub use smallvec;
}

/// Wraps a `SmallVec` with a newtype.
#[macro_export]
macro_rules! smallvec_wrapper {
  (
    $(#[$meta:meta])*
    $vis:vis $name:ident $(<$($generic:tt),+>)? ([$inner:ty; $inlined: expr]);
  ) => {
    $(#[$meta])*
    $vis struct $name $(< $($generic),+ >)? ($crate::__private::smallvec::SmallVec<[$inner; { $inlined }]>);

    impl $(< $($generic),+ >)? ::core::default::Default for $name $(< $($generic),+ >)? {
      fn default() -> Self {
        Self::new()
      }
    }

    impl $(< $($generic),+ >)? $name $(< $($generic),+ >)? {
      /// Creates a new instance.
      #[cfg(not(feature = "const_new"))]
      #[inline]
      pub fn new() -> Self {
        Self($crate::__private::smallvec::SmallVec::new())
      }

      /// Creates a new instance.
      #[cfg(feature = "const_new")]
      #[inline]
      pub const fn new() -> Self {
        Self($crate::__private::smallvec::SmallVec::new_const())
      }

      /// Constructs a new instance on the stack from an array without copying elements.
      #[cfg(not(feature = "const_new"))]
      #[inline]
      pub fn from_buf(buf: [$inner; { $inlined }]) -> Self {
        Self($crate::__private::smallvec::SmallVec::from_buf(buf))
      }

      /// Constructs a new instance on the stack from an array without copying elements.
      #[cfg(feature = "const_new")]
      #[cfg_attr(docsrs, doc(cfg(feature = "const_new")))]
      #[inline]
      pub const fn from_buf(value: [$inner; { $inlined }]) -> Self {
        Self($crate::__private::smallvec::SmallVec::from_const(value))
      }

      /// Constructs a new instance on the stack from an array without copying elements. Also sets the length, which must be less or equal to the size of buf.
      #[inline]
      pub fn from_buf_and_len(value: [$inner; { $inlined }], len: usize) -> Self {
        Self($crate::__private::smallvec::SmallVec::from_buf_and_len(value, len))
      }

      /// Constructs a new instance on the stack from an array without copying elements. Also sets the length. The user is responsible for ensuring that `len <= N`.
      #[cfg(feature = "const_new")]
      #[cfg_attr(docsrs, doc(cfg(feature = "const_new")))]
      #[inline]
      pub const unsafe fn from_const_with_len_unchecked(value: [$inner; { $inlined }], len: usize) -> Self {
        Self($crate::__private::smallvec::SmallVec::from_const_with_len_unchecked(value, len))
      }

      /// Creates a new instance of `ApplyBatchResponse` with the given capacity.
      pub fn with_capacity(capacity: usize) -> Self {
        Self($crate::__private::smallvec::SmallVec::with_capacity(capacity))
      }

      /// Convert self into an `Either::Left` if possible. Otherwise return `Either::Right`.
      ///
      /// This method returns `Either::Left` if the self is on the stack,
      /// or if the `Either::Right` is too long (and all the elements were spilled to the heap).
      #[cfg(feature = "either")]
      #[cfg_attr(docsrs, doc(cfg(feature = "either")))]
      pub fn into_either(self) -> $crate::__private::either::Either<[$inner; { $inlined }], $crate::__private::smallvec::SmallVec<[$inner; { $inlined }]>> {
        match self.0.into_inner() {
          ::core::result::Result::Ok(x) => $crate::__private::either::Either::Left(x),
          ::core::result::Result::Err(x) => $crate::__private::either::Either::Right(x),
        }
      }

      /// Convert [`Either`](either::Either) into self.
      #[cfg(feature = "either")]
      #[cfg_attr(docsrs, doc(cfg(feature = "either")))]
      #[inline]
      pub fn from_either(either: $crate::__private::either::Either<[$inner; { $inlined }], $crate::__private::smallvec::SmallVec<[$inner; { $inlined }]>>) -> Self {
        match either {
          $crate::__private::either::Either::Left(x) => Self($crate::__private::smallvec::SmallVec::from(x)),
          $crate::__private::either::Either::Right(x) => Self(x),
        }
      }
    }

    impl $(< $($generic),+ >)? ::core::convert::From<$inner> for $name $(< $($generic),+ >)? {
      fn from(value: $inner) -> Self {
        Self($crate::__private::smallvec::smallvec![value])
      }
    }

    impl $(< $($generic),+ >)? ::core::convert::From<::core::option::Option<$inner>> for $name $(< $($generic),+ >)? {
      fn from(value: ::core::option::Option<$inner>) -> Self {
        match value {
          ::core::option::Option::Some(value) => Self($crate::__private::smallvec::smallvec![value]),
          ::core::option::Option::None => Self($crate::__private::smallvec::SmallVec::new()),
        }
      }
    }

    impl $(< $($generic),+ >)? ::core::convert::From<[$inner; { $inlined }]> for $name $(< $($generic),+ >)? {
      fn from(value: [$inner; { $inlined }]) -> Self {
        Self(value.into())
      }
    }

    impl $(< $($generic),+ >)? ::core::convert::From<$crate::__private::smallvec::SmallVec<[$inner; { $inlined }]>> for $name $(< $($generic),+ >)? {
      fn from(value: $crate::__private::smallvec::SmallVec<[$inner; { $inlined }]>) -> Self {
        Self(value)
      }
    }

    impl $(< $($generic),+ >)? ::core::ops::Deref for $name $(< $($generic),+ >)? {
      type Target = $crate::__private::smallvec::SmallVec<[$inner; { $inlined }]>;

      fn deref(&self) -> &Self::Target {
        &self.0
      }
    }

    impl $(< $($generic),+ >)? ::core::ops::DerefMut for $name $(< $($generic),+ >)? {
      fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
      }
    }

    impl $(< $($generic),+ >)? ::core::convert::AsRef<$crate::__private::smallvec::SmallVec<[$inner; { $inlined }]>> for $name $(< $($generic),+ >)? {
      fn as_ref(&self) -> &$crate::__private::smallvec::SmallVec<[$inner; { $inlined }]> {
        &self.0
      }
    }

    impl $(< $($generic),+ >)? ::core::convert::AsMut<$crate::__private::smallvec::SmallVec<[$inner; { $inlined }]>> for $name $(< $($generic),+ >)? {
      fn as_mut(&mut self) -> &mut $crate::__private::smallvec::SmallVec<[$inner; { $inlined }]> {
        &mut self.0
      }
    }

    impl $(< $($generic),+ >)? ::core::convert::AsRef<[$inner]> for $name $(< $($generic),+ >)? {
      fn as_ref(&self) -> &[$inner] {
        &self.0
      }
    }

    impl $(< $($generic),+ >)? ::core::convert::AsMut<[$inner]> for $name $(< $($generic),+ >)? {
      fn as_mut(&mut self) -> &mut [$inner] {
        &mut self.0
      }
    }

    impl $(< $($generic),+ >)? ::core::iter::FromIterator<$inner> for $name $(< $($generic),+ >)? {
      fn from_iter<__T: ::core::iter::IntoIterator<Item = $inner>>(iter: __T) -> Self {
        Self(iter.into_iter().collect())
      }
    }

    impl $(< $($generic),+ >)? ::core::iter::IntoIterator for $name $(< $($generic),+ >)? {
      type Item = $inner;
      type IntoIter = $crate::__private::smallvec::IntoIter<[$inner; { $inlined }]>;

      fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
      }
    }
  };
}

#[cfg(feature = "either")]
impl<T> From<Either<T, ::smallvec::SmallVec<[T; 1]>>> for OneOrMore<T> {
  #[inline]
  fn from(either: Either<T, ::smallvec::SmallVec<[T; 1]>>) -> Self {
    match either {
      Either::Left(t) => OneOrMore::from(t),
      Either::Right(vec) => OneOrMore::from(vec),
    }
  }
}

#[cfg(feature = "either")]
impl<T> From<Either<Option<T>, ::smallvec::SmallVec<[T; 1]>>> for OneOrMore<T> {
  #[inline]
  fn from(either: Either<Option<T>, ::smallvec::SmallVec<[T; 1]>>) -> Self {
    match either {
      Either::Left(t) => OneOrMore::from(t),
      Either::Right(vec) => OneOrMore::from(vec),
    }
  }
}

smallvec_wrapper!(
  /// A tiny vec which can inline 1 element on stack.
  #[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(transparent))]
  #[cfg_attr(
    feature = "rkyv",
    derive(::rkyv::Serialize, ::rkyv::Deserialize, ::rkyv::Archive)
  )]
  #[cfg_attr(feature = "rkyv", archive(compare(PartialEq), check_bytes))]
  pub OneOrMore<T>([T; 1]);
);

smallvec_wrapper!(
  /// A tiny vec which can inline 2 elements on stack.
  #[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(transparent))]
  #[cfg_attr(
    feature = "rkyv",
    derive(::rkyv::Serialize, ::rkyv::Deserialize, ::rkyv::Archive)
  )]
  #[cfg_attr(feature = "rkyv", archive(compare(PartialEq), check_bytes))]
  pub TinyVec<T>([T; 2]);
);

smallvec_wrapper!(
  /// A vec which can inline 3 elements on stack.
  #[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(transparent))]
  #[cfg_attr(
    feature = "rkyv",
    derive(::rkyv::Serialize, ::rkyv::Deserialize, ::rkyv::Archive)
  )]
  #[cfg_attr(feature = "rkyv", archive(compare(PartialEq), check_bytes))]
  pub TriVec<T>([T; 3]);
);

smallvec_wrapper!(
  /// A small vec which can inline 4 elements on stack.
  #[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(transparent))]
  #[cfg_attr(
    feature = "rkyv",
    derive(::rkyv::Serialize, ::rkyv::Deserialize, ::rkyv::Archive)
  )]
  #[cfg_attr(feature = "rkyv", archive(compare(PartialEq), check_bytes))]
  pub SmallVec<T>([T; 4]);
);

smallvec_wrapper!(
  /// A medium vec which can inline 8 elements on stack.
  #[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(transparent))]
  #[cfg_attr(
    feature = "rkyv",
    derive(::rkyv::Serialize, ::rkyv::Deserialize, ::rkyv::Archive)
  )]
  #[cfg_attr(feature = "rkyv", archive(compare(PartialEq), check_bytes))]
  pub MediumVec<T>([T; 8]);
);

smallvec_wrapper!(
  /// A big vec which can inline 16 elements on stack.
  #[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(transparent))]
  #[cfg_attr(
    feature = "rkyv",
    derive(::rkyv::Serialize, ::rkyv::Deserialize, ::rkyv::Archive)
  )]
  #[cfg_attr(feature = "rkyv", archive(compare(PartialEq), check_bytes))]
  pub LargeVec<T>([T; 16]);
);

smallvec_wrapper!(
  /// A xlarge vec which can inline 32 elements on stack.
  #[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(transparent))]
  #[cfg_attr(
    feature = "rkyv",
    derive(::rkyv::Serialize, ::rkyv::Deserialize, ::rkyv::Archive)
  )]
  #[cfg_attr(feature = "rkyv", archive(compare(PartialEq), check_bytes))]
  pub XLargeVec<T>([T; 32]);
);

smallvec_wrapper!(
  /// A xxlarge vec which can inline 64 elements on stack.
  #[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(transparent))]
  #[cfg_attr(
    feature = "rkyv",
    derive(::rkyv::Serialize, ::rkyv::Deserialize, ::rkyv::Archive)
  )]
  #[cfg_attr(feature = "rkyv", archive(compare(PartialEq), check_bytes))]
  pub XXLargeVec<T>([T; 64]);
);

smallvec_wrapper!(
  /// A xxxlarge vec which can inline 128 elements on stack.
  #[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(transparent))]
  #[cfg_attr(
    feature = "rkyv",
    derive(::rkyv::Serialize, ::rkyv::Deserialize, ::rkyv::Archive)
  )]
  #[cfg_attr(feature = "rkyv", archive(compare(PartialEq), check_bytes))]
  pub XXXLargeVec<T>([T; 128]);
);
