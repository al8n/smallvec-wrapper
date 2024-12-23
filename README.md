<div align="center">
<h1>smallvec-wrapper</h1>
</div>
<div align="center">

Macro and common structs to play with [`smallvec`](https://crates.io/crates/smallvec).

[<img alt="github" src="https://img.shields.io/badge/github-al8n/smallvec--wrapper-8da0cb?style=for-the-badge&logo=Github" height="22">][Github-url]
[<img alt="Build" src="https://img.shields.io/github/actions/workflow/status/al8n/smallvec-wrapper/ci.yml?logo=Github-Actions&style=for-the-badge" height="22">][CI-url]

[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-smallvec--wrapper-66c2a5?style=for-the-badge&labelColor=555555&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">][doc-url]
[<img alt="crates.io" src="https://img.shields.io/crates/v/smallvec-wrapper?style=for-the-badge&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iaXNvLTg4NTktMSI/Pg0KPCEtLSBHZW5lcmF0b3I6IEFkb2JlIElsbHVzdHJhdG9yIDE5LjAuMCwgU1ZHIEV4cG9ydCBQbHVnLUluIC4gU1ZHIFZlcnNpb246IDYuMDAgQnVpbGQgMCkgIC0tPg0KPHN2ZyB2ZXJzaW9uPSIxLjEiIGlkPSJMYXllcl8xIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIiB4PSIwcHgiIHk9IjBweCINCgkgdmlld0JveD0iMCAwIDUxMiA1MTIiIHhtbDpzcGFjZT0icHJlc2VydmUiPg0KPGc+DQoJPGc+DQoJCTxwYXRoIGQ9Ik0yNTYsMEwzMS41MjgsMTEyLjIzNnYyODcuNTI4TDI1Niw1MTJsMjI0LjQ3Mi0xMTIuMjM2VjExMi4yMzZMMjU2LDB6IE0yMzQuMjc3LDQ1Mi41NjRMNzQuOTc0LDM3Mi45MTNWMTYwLjgxDQoJCQlsMTU5LjMwMyw3OS42NTFWNDUyLjU2NHogTTEwMS44MjYsMTI1LjY2MkwyNTYsNDguNTc2bDE1NC4xNzQsNzcuMDg3TDI1NiwyMDIuNzQ5TDEwMS44MjYsMTI1LjY2MnogTTQzNy4wMjYsMzcyLjkxMw0KCQkJbC0xNTkuMzAzLDc5LjY1MVYyNDAuNDYxbDE1OS4zMDMtNzkuNjUxVjM3Mi45MTN6IiBmaWxsPSIjRkZGIi8+DQoJPC9nPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPC9zdmc+DQo=" height="22">][crates-url]
[<img alt="crates.io" src="https://img.shields.io/crates/d/smallvec-wrapper?color=critical&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBzdGFuZGFsb25lPSJubyI/PjwhRE9DVFlQRSBzdmcgUFVCTElDICItLy9XM0MvL0RURCBTVkcgMS4xLy9FTiIgImh0dHA6Ly93d3cudzMub3JnL0dyYXBoaWNzL1NWRy8xLjEvRFREL3N2ZzExLmR0ZCI+PHN2ZyB0PSIxNjQ1MTE3MzMyOTU5IiBjbGFzcz0iaWNvbiIgdmlld0JveD0iMCAwIDEwMjQgMTAyNCIgdmVyc2lvbj0iMS4xIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHAtaWQ9IjM0MjEiIGRhdGEtc3BtLWFuY2hvci1pZD0iYTMxM3guNzc4MTA2OS4wLmkzIiB3aWR0aD0iNDgiIGhlaWdodD0iNDgiIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIj48ZGVmcz48c3R5bGUgdHlwZT0idGV4dC9jc3MiPjwvc3R5bGU+PC9kZWZzPjxwYXRoIGQ9Ik00NjkuMzEyIDU3MC4yNHYtMjU2aDg1LjM3NnYyNTZoMTI4TDUxMiA3NTYuMjg4IDM0MS4zMTIgNTcwLjI0aDEyOHpNMTAyNCA2NDAuMTI4QzEwMjQgNzgyLjkxMiA5MTkuODcyIDg5NiA3ODcuNjQ4IDg5NmgtNTEyQzEyMy45MDQgODk2IDAgNzYxLjYgMCA1OTcuNTA0IDAgNDUxLjk2OCA5NC42NTYgMzMxLjUyIDIyNi40MzIgMzAyLjk3NiAyODQuMTYgMTk1LjQ1NiAzOTEuODA4IDEyOCA1MTIgMTI4YzE1Mi4zMiAwIDI4Mi4xMTIgMTA4LjQxNiAzMjMuMzkyIDI2MS4xMkM5NDEuODg4IDQxMy40NCAxMDI0IDUxOS4wNCAxMDI0IDY0MC4xOTJ6IG0tMjU5LjItMjA1LjMxMmMtMjQuNDQ4LTEyOS4wMjQtMTI4Ljg5Ni0yMjIuNzItMjUyLjgtMjIyLjcyLTk3LjI4IDAtMTgzLjA0IDU3LjM0NC0yMjQuNjQgMTQ3LjQ1NmwtOS4yOCAyMC4yMjQtMjAuOTI4IDIuOTQ0Yy0xMDMuMzYgMTQuNC0xNzguMzY4IDEwNC4zMi0xNzguMzY4IDIxNC43MiAwIDExNy45NTIgODguODMyIDIxNC40IDE5Ni45MjggMjE0LjRoNTEyYzg4LjMyIDAgMTU3LjUwNC03NS4xMzYgMTU3LjUwNC0xNzEuNzEyIDAtODguMDY0LTY1LjkyLTE2NC45MjgtMTQ0Ljk2LTE3MS43NzZsLTI5LjUwNC0yLjU2LTUuODg4LTMwLjk3NnoiIGZpbGw9IiNmZmZmZmYiIHAtaWQ9IjM0MjIiIGRhdGEtc3BtLWFuY2hvci1pZD0iYTMxM3guNzc4MTA2OS4wLmkwIiBjbGFzcz0iIj48L3BhdGg+PC9zdmc+&style=for-the-badge" height="22">][crates-url]

<img alt="license" src="https://img.shields.io/badge/License-Apache%202.0/MIT-blue.svg?style=for-the-badge&fontColor=white&logoColor=f5c076&logo=data:image/svg+xml;base64,PCFET0NUWVBFIHN2ZyBQVUJMSUMgIi0vL1czQy8vRFREIFNWRyAxLjEvL0VOIiAiaHR0cDovL3d3dy53My5vcmcvR3JhcGhpY3MvU1ZHLzEuMS9EVEQvc3ZnMTEuZHRkIj4KDTwhLS0gVXBsb2FkZWQgdG86IFNWRyBSZXBvLCB3d3cuc3ZncmVwby5jb20sIFRyYW5zZm9ybWVkIGJ5OiBTVkcgUmVwbyBNaXhlciBUb29scyAtLT4KPHN2ZyBmaWxsPSIjZmZmZmZmIiBoZWlnaHQ9IjgwMHB4IiB3aWR0aD0iODAwcHgiIHZlcnNpb249IjEuMSIgaWQ9IkNhcGFfMSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiB4bWxuczp4bGluaz0iaHR0cDovL3d3dy53My5vcmcvMTk5OS94bGluayIgdmlld0JveD0iMCAwIDI3Ni43MTUgMjc2LjcxNSIgeG1sOnNwYWNlPSJwcmVzZXJ2ZSIgc3Ryb2tlPSIjZmZmZmZmIj4KDTxnIGlkPSJTVkdSZXBvX2JnQ2FycmllciIgc3Ryb2tlLXdpZHRoPSIwIi8+Cg08ZyBpZD0iU1ZHUmVwb190cmFjZXJDYXJyaWVyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiLz4KDTxnIGlkPSJTVkdSZXBvX2ljb25DYXJyaWVyIj4gPGc+IDxwYXRoIGQ9Ik0xMzguMzU3LDBDNjIuMDY2LDAsMCw2Mi4wNjYsMCwxMzguMzU3czYyLjA2NiwxMzguMzU3LDEzOC4zNTcsMTM4LjM1N3MxMzguMzU3LTYyLjA2NiwxMzguMzU3LTEzOC4zNTcgUzIxNC42NDgsMCwxMzguMzU3LDB6IE0xMzguMzU3LDI1OC43MTVDNzEuOTkyLDI1OC43MTUsMTgsMjA0LjcyMywxOCwxMzguMzU3UzcxLjk5MiwxOCwxMzguMzU3LDE4IHMxMjAuMzU3LDUzLjk5MiwxMjAuMzU3LDEyMC4zNTdTMjA0LjcyMywyNTguNzE1LDEzOC4zNTcsMjU4LjcxNXoiLz4gPHBhdGggZD0iTTE5NC43OTgsMTYwLjkwM2MtNC4xODgtMi42NzctOS43NTMtMS40NTQtMTIuNDMyLDIuNzMyYy04LjY5NCwxMy41OTMtMjMuNTAzLDIxLjcwOC0zOS42MTQsMjEuNzA4IGMtMjUuOTA4LDAtNDYuOTg1LTIxLjA3OC00Ni45ODUtNDYuOTg2czIxLjA3Ny00Ni45ODYsNDYuOTg1LTQ2Ljk4NmMxNS42MzMsMCwzMC4yLDcuNzQ3LDM4Ljk2OCwyMC43MjMgYzIuNzgyLDQuMTE3LDguMzc1LDUuMjAxLDEyLjQ5NiwyLjQxOGM0LjExOC0yLjc4Miw1LjIwMS04LjM3NywyLjQxOC0xMi40OTZjLTEyLjExOC0xNy45MzctMzIuMjYyLTI4LjY0NS01My44ODItMjguNjQ1IGMtMzUuODMzLDAtNjQuOTg1LDI5LjE1Mi02NC45ODUsNjQuOTg2czI5LjE1Miw2NC45ODYsNjQuOTg1LDY0Ljk4NmMyMi4yODEsMCw0Mi43NTktMTEuMjE4LDU0Ljc3OC0zMC4wMDkgQzIwMC4yMDgsMTY5LjE0NywxOTguOTg1LDE2My41ODIsMTk0Ljc5OCwxNjAuOTAzeiIvPiA8L2c+IDwvZz4KDTwvc3ZnPg==" height="22">

</div>

## Installation

```toml
[dependencies]
smallvec-wrapper = "0.2"
```

## Example

```rust
use smallvec_wrapper::smallvec_warpper;

smallvec_wrapper!(
  /// A vec holds the first 5 elements on the stack.
  #[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(transparent))]
  pub MyVec<T>([T; 5]);
);
```

The macro will help you generate a new type and methods.

<details>

  <summary>
    Show generated code
  </summary>

  ```rust
  /// A vec holds the first 5 elements on the stack.
  #[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(transparent))]
  pub struct MyVec<T>(::smallvec_wrapper::__private::smallvec::SmallVec<[T; { 5 }]>);

  impl<T> ::core::default::Default for MyVec<T> {
      fn default() -> Self {
          Self::new()
      }
  }

  impl<T> MyVec<T> {
      /// Creates a new instance.
      #[cfg(feature = "const_new")]
      #[inline]
      pub const fn new() -> Self {
          Self(::smallvec_wrapper::__private::smallvec::SmallVec::new_const())
      }
      /// Constructs a new instance on the stack from an array without copying elements.
      #[cfg(feature = "const_new")]
      #[inline]
      pub const fn from_buf(value: [T; { 5 }]) -> Self {
          Self(::smallvec_wrapper::__private::smallvec::SmallVec::from_const(value))
      }
      /// Constructs a new instance on the stack from an array without copying elements. Also sets the length, which must be less or equal to the size of buf.
      #[inline]
      pub fn from_buf_and_len(value: [T; { 5 }], len: usize) -> Self {
          Self(
              ::smallvec_wrapper::__private::smallvec::SmallVec::from_buf_and_len(
                  value,
                  len,
              ),
          )
      }
      /// Constructs a new instance on the stack from an array without copying elements. Also sets the length. The user is responsible for ensuring that `len <= N`.
      ///
      /// # Safety
      /// - The user is responsible for ensuring that `len <= N`.
      #[cfg(feature = "const_new")]
      #[inline]
      pub const unsafe fn from_const_with_len_unchecked(
          value: [T; { 5 }],
          len: usize,
      ) -> Self {
          Self(
              ::smallvec_wrapper::__private::smallvec::SmallVec::from_const_with_len_unchecked(
                  value,
                  len,
              ),
          )
      }
      /// Creates a new instance of `ApplyBatchResponse` with the given capacity.
      pub fn with_capacity(capacity: usize) -> Self {
          Self(::smallvec_wrapper::__private::smallvec::SmallVec::with_capacity(capacity))
      }
  }
  impl<T> ::core::convert::From<T> for MyVec<T> {
      fn from(value: T) -> Self {
          Self({
              let count = 0usize + 1usize;
              #[allow(unused_mut)]
              let mut vec = ::smallvec::SmallVec::new();
              if count <= vec.inline_size() {
                  vec.push(value);
                  vec
              } else {
                  ::smallvec::SmallVec::from_vec(
                      <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([value])),
                  )
              }
          })
      }
  }
  impl<T> ::core::convert::From<::std::vec::Vec<T>> for MyVec<T> {
      fn from(values: ::std::vec::Vec<T>) -> Self {
          Self(::core::convert::Into::into(values))
      }
  }
  impl<T> ::core::convert::From<&[T]> for MyVec<T>
  where
      T: ::core::clone::Clone,
  {
      fn from(values: &[T]) -> Self {
          Self(::core::convert::Into::into(values))
      }
  }
  impl<T> ::core::convert::From<::core::option::Option<T>> for MyVec<T> {
      fn from(value: ::core::option::Option<T>) -> Self {
          match value {
              ::core::option::Option::Some(value) => {
                  Self({
                      let count = 0usize + 1usize;
                      #[allow(unused_mut)]
                      let mut vec = ::smallvec::SmallVec::new();
                      if count <= vec.inline_size() {
                          vec.push(value);
                          vec
                      } else {
                          ::smallvec::SmallVec::from_vec(
                              <[_]>::into_vec(
                                  #[rustc_box]
                                  ::alloc::boxed::Box::new([value]),
                              ),
                          )
                      }
                  })
              }
              ::core::option::Option::None => {
                  Self(::smallvec_wrapper::__private::smallvec::SmallVec::new())
              }
          }
      }
  }
  impl<T> ::core::convert::From<[T; { 5 }]> for MyVec<T> {
      fn from(value: [T; { 5 }]) -> Self {
          Self(value.into())
      }
  }
  impl<
      T,
  > ::core::convert::From<::smallvec_wrapper::__private::smallvec::SmallVec<[T; { 5 }]>>
  for MyVec<T> {
      fn from(
          value: ::smallvec_wrapper::__private::smallvec::SmallVec<[T; { 5 }]>,
      ) -> Self {
          Self(value)
      }
  }
  impl<T> ::core::ops::Deref for MyVec<T> {
      type Target = ::smallvec_wrapper::__private::smallvec::SmallVec<[T; { 5 }]>;
      fn deref(&self) -> &Self::Target {
          &self.0
      }
  }
  impl<T> ::core::ops::DerefMut for MyVec<T> {
      fn deref_mut(&mut self) -> &mut Self::Target {
          &mut self.0
      }
  }
  impl<T> ::core::borrow::Borrow<[T]> for MyVec<T> {
      fn borrow(&self) -> &[T] {
          &self.0
      }
  }
  impl<T> ::core::borrow::BorrowMut<[T]> for MyVec<T> {
      fn borrow_mut(&mut self) -> &mut [T] {
          &mut self.0
      }
  }
  impl<
      T,
  > ::core::convert::AsRef<::smallvec_wrapper::__private::smallvec::SmallVec<[T; { 5 }]>>
  for MyVec<T> {
      fn as_ref(&self) -> &::smallvec_wrapper::__private::smallvec::SmallVec<[T; { 5 }]> {
          &self.0
      }
  }
  impl<
      T,
  > ::core::convert::AsMut<::smallvec_wrapper::__private::smallvec::SmallVec<[T; { 5 }]>>
  for MyVec<T> {
      fn as_mut(
          &mut self,
      ) -> &mut ::smallvec_wrapper::__private::smallvec::SmallVec<[T; { 5 }]> {
          &mut self.0
      }
  }
  impl<T> ::core::convert::AsRef<[T]> for MyVec<T> {
      fn as_ref(&self) -> &[T] {
          &self.0
      }
  }
  impl<T> ::core::convert::AsMut<[T]> for MyVec<T> {
      fn as_mut(&mut self) -> &mut [T] {
          &mut self.0
      }
  }
  impl<T> ::core::iter::FromIterator<T> for MyVec<T> {
      fn from_iter<__T: ::core::iter::IntoIterator<Item = T>>(iter: __T) -> Self {
          Self(iter.into_iter().collect())
      }
  }
  impl<T> ::core::iter::IntoIterator for MyVec<T> {
      type Item = T;
      type IntoIter = ::smallvec_wrapper::__private::smallvec::IntoIter<[T; { 5 }]>;
      fn into_iter(self) -> Self::IntoIter {
          self.0.into_iter()
      }
  }
  impl<'a, T> ::core::iter::IntoIterator for &'a MyVec<T> {
      type Item = &'a T;
      type IntoIter = ::core::slice::Iter<'a, T>;
      fn into_iter(self) -> Self::IntoIter {
          (&self.0).into_iter()
      }
  }
  impl<'a, T> ::core::iter::IntoIterator for &'a mut MyVec<T> {
      type Item = &'a mut T;
      type IntoIter = ::core::slice::IterMut<'a, T>;
      fn into_iter(self) -> Self::IntoIter {
          self.iter_mut()
      }
  }
  impl<T> ::core::iter::Extend<T> for MyVec<T> {
      #[inline]
      fn extend<I: ::core::iter::IntoIterator<Item = T>>(&mut self, iter: I) {
          self.0.extend(iter)
      }
  }
  ```

</details>





#### License

`smallvec-wrapper` is under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT) for details.

Copyright (c) 2024 Al Liu.

[Github-url]: https://github.com/al8n/smallvec-wrapper/
[CI-url]: https://github.com/al8n/smallvec-wrapper/actions/workflows/ci.yml
[doc-url]: https://docs.rs/smallvec-wrapper
[crates-url]: https://crates.io/crates/smallvec-wrapper
[zh-cn-url]: https://github.com/al8n/smallvec-wrapper/tree/main/README-zh_CN.md
