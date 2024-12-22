[![Build Status (nightly)](https://github.com/sigurd4/option_trait/workflows/Build-nightly/badge.svg)](https://github.com/sigurd4/option_trait/actions/workflows/build-nightly.yml)
[![Build Status (nightly, all features)](https://github.com/sigurd4/option_trait/workflows/Build-nightly-all-features/badge.svg)](https://github.com/sigurd4/option_trait/actions/workflows/build-nightly-all-features.yml)

[![Build Status (stable)](https://github.com/sigurd4/option_trait/workflows/Build-stable/badge.svg)](https://github.com/sigurd4/option_trait/actions/workflows/build-stable.yml)
[![Build Status (stable, all features)](https://github.com/sigurd4/option_trait/workflows/Build-stable-all-features/badge.svg)](https://github.com/sigurd4/option_trait/actions/workflows/build-stable-all-features.yml)

[![Test Status](https://github.com/sigurd4/option_trait/workflows/Test/badge.svg)](https://github.com/sigurd4/option_trait/actions/workflows/test.yml)
[![Lint Status](https://github.com/sigurd4/option_trait/workflows/Lint/badge.svg)](https://github.com/sigurd4/option_trait/actions/workflows/lint.yml)

[![Latest Version](https://img.shields.io/crates/v/option_trait.svg)](https://crates.io/crates/option_trait)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://img.shields.io/docsrs/option_trait)](https://docs.rs/option_trait)
[![Coverage Status](https://img.shields.io/codecov/c/github/sigurd4/option_trait)](https://app.codecov.io/github/sigurd4/option_trait)

# option_trait

Provides the `Optional` trait for `Option`s, as well as compile-time managed `Option` alternatives, all generalized under the trait `Maybe`.

`Maybe<T>` is implemented for:
- `Option<T>`
    - Run-time managed
    - Also implements `Optional` and `PureMaybe`
- `T` and `()`
    - Compile-time managed
    - Also implements `PureStaticMaybe`, `PureMaybe` and `StaticMaybe`
- `[T; 1]` and `[T; 0]`
    - Compile-time managed
    - Can be managed using constant expressions, but with some difficulty
    - Also implements `StaticMaybe`
- `OptCell<T, _>` (`feature = "opt_cell"`)
    - Compile-time managed
    - Can be more easily managed using boolean constant expressions
    - Has const methods
    - Also implements `StaticMaybe`

## Examples

This is how i like to handle optional function arguments with maximum flexibility.

```rust
use option_trait::*;

fn f<O>(required: i32, optional: O)
where
    O: Maybe<i32>
{
    if O::IS_MAYBE_SOME
    {
        // This part of the code will be disabled at compile-time if the maybe cannot possibly contain a value.
    }

    // Do whatever
}

f(1, 2);
f(1, ());
f(1, Some(2));
f(1, None);
f(1, [2]);
f(1, [] as [i32; 0]);
f(1, OptCell::some(2));
f(1, OptCell::none());
```