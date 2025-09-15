# debugless-unwrap

This library provides alternatives to the standard `.unwrap`* methods on `Result` and `Option` that don't require `Debug` to be implemented on the unexpected variant.

## Installation

```cmd
cargo add debugless-unwrap
```

## Example

```rust
use assert_panic::assert_panic;
use debugless_unwrap::prelude::*;

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

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`debugless-unwrap` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

* The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
* The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).
