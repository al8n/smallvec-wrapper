//! Macro and common structs to play with [`smallvec`].
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(missing_docs)]

pub use either;
use either::Either;

#[cfg(not(feature = "std"))]
extern crate alloc as std;

#[doc(hidden)]
pub mod __private {
  pub use either;
  pub use paste;
  pub use smallvec;

  pub use std::boxed::Box;
  pub use std::{vec, vec::Vec};
}

/// Wraps a `SmallVec` with a newtype.
#[macro_export]
macro_rules! smallvec_wrapper {
  (
    $(#[$meta:meta])*
    $vis:vis $name:ident $(<$($generic:tt),+>)? ([$inner:ty; $inlined: expr]);
  ) => {
    $crate::__private::paste::paste! {
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
        ///
        /// # Safety
        /// - The user is responsible for ensuring that `len <= N`.
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

        #[cfg_attr(docsrs, doc(cfg(feature = "either")))]
        pub fn into_either(self) -> $crate::__private::either::Either<[$inner; { $inlined }], $crate::__private::smallvec::SmallVec<[$inner; { $inlined }]>> {
          match self.0.into_inner() {
            ::core::result::Result::Ok(x) => $crate::__private::either::Either::Left(x),
            ::core::result::Result::Err(x) => $crate::__private::either::Either::Right(x),
          }
        }

        /// Convert [`Either`] into self.

        #[cfg_attr(docsrs, doc(cfg(feature = "either")))]
        #[inline]
        pub fn from_either(either: $crate::__private::either::Either<[$inner; { $inlined }], $crate::__private::smallvec::SmallVec<[$inner; { $inlined }]>>) -> Self {
          match either {
            $crate::__private::either::Either::Left(x) => Self($crate::__private::smallvec::SmallVec::from(x)),
            $crate::__private::either::Either::Right(x) => Self(x),
          }
        }

        #[doc = "Convert the `" $name "` into an `A` if possible. Otherwise return `Err(Self)`."]
        ///
        #[doc = "This method returns `Err(Self)` if the `" $name "` is too short (and the `A` contains uninitialized elements),"]
        #[doc = "or if the `" $name "` is too long (and all the elements were spilled to the heap)."]
        pub fn into_inner(self) -> $crate::__private::either::Either<[$inner; { $inlined }], $crate::__private::Vec<$inner>> {
          match self.0.into_inner() {
            ::core::result::Result::Ok(x) => $crate::__private::either::Either::Left(x),
            ::core::result::Result::Err(x) => $crate::__private::either::Either::Right(x.into_vec()),
          }
        }

        /// Converts a `SmallVec` into a `Box<[T]>` without reallocating if the `SmallVec` has already spilled
        #[doc = "Converts a `" $name "` into a `Box<[T]>` without reallocating if the `" $name "` has already spilled"]
        /// onto the heap.
        ///
        /// Note that this will drop any excess capacity.
        pub fn into_boxed_slice(self) -> $crate::__private::Box<[$inner]> {
          self.0.into_boxed_slice()
        }

        #[doc = "Converts a `" $name "` into a `Vec` without reallocating if the `" $name "` has already spilled"]
        /// onto the heap.
        pub fn into_vec(self) -> $crate::__private::Vec<$inner> {
          self.0.into_vec()
        }

        #[doc = "Construct a new `" $name "` from a `Vec<A::Item>`."]
        ///
        /// Elements will be copied to the inline buffer if `vec.capacity() <= Self::inline_capacity()`.
        #[inline]
        pub fn from_vec(vec: $crate::__private::Vec<$inner>) -> Self {
          Self($crate::__private::smallvec::SmallVec::from_vec(vec))
        }
      }

      impl $(< $($generic),+ >)? ::core::convert::From<$inner> for $name $(< $($generic),+ >)? {
        fn from(value: $inner) -> Self {
          Self($crate::__private::smallvec::smallvec![value])
        }
      }

      impl $(< $($generic),+ >)? ::core::convert::From<::std::vec::Vec<$inner>> for $name $(< $($generic),+ >)? {
        fn from(values: ::std::vec::Vec<$inner>) -> Self {
          Self(::core::convert::Into::into(values))
        }
      }

      impl $(< $($generic),+ >)? ::core::convert::From<&[$inner]> for $name $(< $($generic),+ >)?
      where
        $inner: ::core::clone::Clone,
      {
        fn from(values: &[$inner]) -> Self {
          Self(::core::convert::Into::into(values))
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

      impl $(< $($generic),+ >)? ::core::borrow::Borrow<[$inner]> for $name $(< $($generic),+ >)? {
        fn borrow(&self) -> &[$inner] {
          &self.0
        }
      }

      impl $(< $($generic),+ >)? ::core::borrow::BorrowMut<[$inner]> for $name $(< $($generic),+ >)? {
        fn borrow_mut(&mut self) -> &mut [$inner] {
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

      impl<'a, $( $($generic),+ )?> ::core::iter::IntoIterator for &'a $name $(< $($generic),+ >)? {
        type Item = &'a $inner;
        type IntoIter = ::core::slice::Iter<'a, $inner>;

        fn into_iter(self) -> Self::IntoIter {
          (&self.0).into_iter()
        }
      }

      impl<'a, $( $($generic),+ )?> ::core::iter::IntoIterator for &'a mut $name $(< $($generic),+ >)? {
        type Item = &'a mut $inner;
        type IntoIter = ::core::slice::IterMut<'a, $inner>;

        fn into_iter(self) -> Self::IntoIter {
          (&mut self.0).into_iter()
        }
      }

      impl $(< $($generic),+ >)? ::core::iter::Extend<$inner> for $name $(< $($generic),+ >)? {
        #[inline]
        fn extend<I: ::core::iter::IntoIterator<Item = $inner>>(&mut self, iter: I) {
          self.0.extend(iter)
        }
      }
    }
  };
}

impl<T> From<Either<T, ::smallvec::SmallVec<[T; 1]>>> for OneOrMore<T> {
  #[inline]
  fn from(either: Either<T, ::smallvec::SmallVec<[T; 1]>>) -> Self {
    match either {
      Either::Left(t) => OneOrMore::from(t),
      Either::Right(vec) => OneOrMore::from(vec),
    }
  }
}

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
  #[cfg_attr(feature = "rkyv", rkyv(compare(PartialEq)))]
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
  #[cfg_attr(feature = "rkyv", rkyv(compare(PartialEq)))]
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
  #[cfg_attr(feature = "rkyv", rkyv(compare(PartialEq)))]
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
  #[cfg_attr(feature = "rkyv", rkyv(compare(PartialEq)))]
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
  #[cfg_attr(feature = "rkyv", rkyv(compare(PartialEq)))]
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
  #[cfg_attr(feature = "rkyv", rkyv(compare(PartialEq)))]
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
  #[cfg_attr(feature = "rkyv", rkyv(compare(PartialEq)))]
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
  #[cfg_attr(feature = "rkyv", rkyv(compare(PartialEq)))]
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
  #[cfg_attr(feature = "rkyv", rkyv(compare(PartialEq)))]
  pub XXXLargeVec<T>([T; 128]);
);
