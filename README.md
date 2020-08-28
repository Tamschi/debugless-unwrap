# debugless-unwrap

This library provides alternatives to the standard `.unwrap`* methods on `Result` and `Option` that don't require `Debug` to be implemented on the unexpected variant:

```rust
use assert_panic::assert_panic;
use debugless_unwrap::*;

#[derive(Copy, Clone)]
struct T;

let some = Some(T);
let none = Option::<T>::None;
let ok = Result::<T, T>::Ok(T);
let err = Result::<T, T>::Err(T);

none.debugless_unwrap_none();
ok.debugless_unwrap();
err.debugless_unwrap_err();

assert_panic!(some.debugless_unwrap_none());
assert_panic!({ err.debugless_unwrap(); });
assert_panic!({ ok.debugless_unwrap_err(); });
```

[Changelog](./CHANGELOG.md)
