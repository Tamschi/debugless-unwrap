pub trait DebuglessUnwrap {
	type Unwrapped;
	fn debugless_unwrap(self) -> Self::Unwrapped;
}

impl<T, E> DebuglessUnwrap for Result<T, E> {
	type Unwrapped = T;
	fn debugless_unwrap(self) -> Self::Unwrapped {
		match self {
			Ok(unwrapped) => unwrapped,
			Err(_) => panic!("Tried to debugless_unwrap Err value"),
		}
	}
}

impl<T> DebuglessUnwrap for Option<T> {
	type Unwrapped = T;
	fn debugless_unwrap(self) -> Self::Unwrapped {
		match self {
			Some(unwrapped) => unwrapped,
			None => panic!("Tried to debugless_unwrap None value"),
		}
	}
}

pub trait DebuglessUnwrapErr {
	type Unwrapped;
	fn debugless_unwrap_err(self) -> Self::Unwrapped;
}

impl<T, E> DebuglessUnwrapErr for Result<T, E> {
	type Unwrapped = E;
	fn debugless_unwrap_err(self) -> Self::Unwrapped {
		match self {
			Ok(_) => panic!("Tried to debugless_unwrap_err Ok value"),
			Err(unwrapped) => unwrapped,
		}
	}
}

pub trait DebuglessUnwrapNone {
	fn debugless_unwrap_none(self);
}

impl<T> DebuglessUnwrapNone for Option<T> {
	fn debugless_unwrap_none(self) {
		if self.is_some() {
			panic!("Tried to debugless_unwrap_none Some value")
		}
	}
}
