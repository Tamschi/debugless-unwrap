//! This library provides alternatives to the standard `.unwrap`* methods on `Result` and `Option` that don't require `Debug` to be implemented on the unexpected variant.
//!
//! # Example
//!
//! Given
//!
//! ```
//! struct T;
//!
//! let none = Option::<T>::None;
//! let ok = Result::<T, T>::Ok(T);
//! let err = Result::<T, T>::Err(T);
//! ```
//!
//! , the following `std` methods are unavailable:
//!
//! ```compile_fail
//! # struct T;
//! # let none = Option::<T>::None;
//! # let ok = Result::<T, T>::Ok(T);
//! # let err = Result::<T, T>::Err(T);
//! #
//! none.unwrap_none(); // Some(T) isn't Debug.
//! ```
//!
//! ```compile_fail
//! # struct T;
//! # let none = Option::<T>::None;
//! # let ok = Result::<T, T>::Ok(T);
//! # let err = Result::<T, T>::Err(T);
//! #
//! ok.unwrap(); // Err(T) isn't Debug.
//! ```
//!
//! ```compile_fail
//! # struct T;
//! # let none = Option::<T>::None;
//! # let ok = Result::<T, T>::Ok(T);
//! # let err = Result::<T, T>::Err(T);
//! #
//! err.unwrap_err(); // Ok(T) isn't Debug.
//! ```
//!
//! The methods in this library can be used in this situation (e.g. when working with generics), but provide less information:
//!
//! Additionally given
//!
//! ```
//! use debugless_unwrap::*;
//! ```
//!
//! , the following work like their `debugless_`-less equivalents:
//!
//! ```
//! # use debugless_unwrap::*;
//! # struct T;
//! # let none = Option::<T>::None;
//! # let ok = Result::<T, T>::Ok(T);
//! # let err = Result::<T, T>::Err(T);
//! #
//! none.debugless_unwrap_none();
//! ```
//!
//! ```
//! # use debugless_unwrap::*;
//! # struct T;
//! # let none = Option::<T>::None;
//! # let ok = Result::<T, T>::Ok(T);
//! # let err = Result::<T, T>::Err(T);
//! #
//! ok.debugless_unwrap();
//! ```
//!
//! ```
//! # use debugless_unwrap::*;
//! # struct T;
//! # let none = Option::<T>::None;
//! # let ok = Result::<T, T>::Ok(T);
//! # let err = Result::<T, T>::Err(T);
//! #
//! err.debugless_unwrap_err();
//! ```

#![no_std]
#![warn(clippy::pedantic)]

/// Provides `.debugless_unwrap()` on `Result`.
///
/// # Example
///
/// ```
/// use assert_panic::assert_panic;
/// use debugless_unwrap::DebuglessUnwrapExt;
///
/// struct T;
///
///  let ok = Result::<T, T>::Ok(T);
///  let err = Result::<T, T>::Err(T);
///
/// ok.debugless_unwrap();
///
/// assert_panic!({ err.debugless_unwrap(); });
/// ```
pub trait DebuglessUnwrapExt {
	type Unwrapped;

	#[track_caller]
	fn debugless_unwrap(self) -> Self::Unwrapped;
}

impl<T, E> DebuglessUnwrapExt for Result<T, E> {
	type Unwrapped = T;
	fn debugless_unwrap(self) -> Self::Unwrapped {
		if let Ok(unwrapped) = self {
			unwrapped
		} else {
			panic!("Tried to debugless_unwrap Err value")
		}
	}
}

/// Provides `.debugless_unwrap_err()` on `Result`.
///
/// # Example
///
/// ```
/// use assert_panic::assert_panic;
/// use debugless_unwrap::DebuglessUnwrapErrExt;
///
/// struct T;
///
///  let ok = Result::<T, T>::Ok(T);
///  let err = Result::<T, T>::Err(T);
///
/// err.debugless_unwrap_err();
///
/// assert_panic!({ ok.debugless_unwrap_err(); });
/// ```
pub trait DebuglessUnwrapErrExt {
	type Unwrapped;

	#[track_caller]
	fn debugless_unwrap_err(self) -> Self::Unwrapped;
}

impl<T, E> DebuglessUnwrapErrExt for Result<T, E> {
	type Unwrapped = E;
	fn debugless_unwrap_err(self) -> Self::Unwrapped {
		match self {
			Ok(_) => panic!("Tried to debugless_unwrap_err Ok value"),
			Err(unwrapped) => unwrapped,
		}
	}
}

/// Provides `.debugless_unwrap_none()` on `Option`.
///
/// # Example
///
/// ```
/// use assert_panic::assert_panic;
/// use debugless_unwrap::DebuglessUnwrapNoneExt;
///
/// struct T;
///
///  let some = Some(T);
///  let none = Option::<T>::None;
///
/// none.debugless_unwrap_none();
///
/// assert_panic!(some.debugless_unwrap_none());
/// ```
pub trait DebuglessUnwrapNoneExt {
	#[track_caller]
	fn debugless_unwrap_none(self);
}

impl<T> DebuglessUnwrapNoneExt for Option<T> {
	fn debugless_unwrap_none(self) {
		assert!(self.is_none(), "Tried to debugless_unwrap_none Some value");
	}
}

pub mod prelude {
	pub use super::{DebuglessUnwrapErrExt, DebuglessUnwrapExt, DebuglessUnwrapNoneExt};
}

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
pub mod readme {}
