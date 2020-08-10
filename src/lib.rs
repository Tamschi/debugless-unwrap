pub trait OpaqueUnwrap {
    type Unwrapped;
    fn opaque_unwrap(self) -> Self::Unwrapped;
}

impl<T, E> OpaqueUnwrap for Result<T, E> {
    type Unwrapped = T;
    fn opaque_unwrap(self) -> Self::Unwrapped {
        match self {
            Ok(unwrapped) => unwrapped,
            Err(_) => panic!("Tried to opaque_unwrap Err value"),
        }
    }
}

impl<T> OpaqueUnwrap for Option<T> {
    type Unwrapped = T;
    fn opaque_unwrap(self) -> Self::Unwrapped {
        match self {
            Some(unwrapped) => unwrapped,
            None => panic!("Tried to opaque_unwrap None value"),
        }
    }
}

pub trait OpaqueUnwrapErr {
    type Unwrapped;
    fn opaque_unwrap_err(self) -> Self::Unwrapped;
}

impl<T, E> OpaqueUnwrapErr for Result<T, E> {
    type Unwrapped = E;
    fn opaque_unwrap_err(self) -> Self::Unwrapped {
        match self {
            Ok(_) => panic!("Tried to opaque_unwrap_err Ok value"),
            Err(unwrapped) => unwrapped,
        }
    }
}

pub trait OpaqueUnwrapNone {
    fn opaque_unwrap_none(self);
}

impl<T> OpaqueUnwrapNone for Option<T> {
    fn opaque_unwrap_none(self) {
        if self.is_some() {
            panic!("Tried to opaque_unwrap_err Some value")
        }
    }
}
