# tuple-transpose

[![Build Status](https://dev.azure.com/upsuper/upsuper/_apis/build/status/tuple-transpose?branchName=master)](https://dev.azure.com/upsuper/upsuper/_build/latest?definitionId=3&branchName=master)
[![Crates.io](https://img.shields.io/crates/v/tuple-transpose.svg)](https://crates.io/crates/tuple-transpose)

Transpose a tuple of results or options to result or option of tuple.

## Examples

```rust
// Result
assert_eq!((Ok::<_, ()>(1u32), Ok(2.0f32)).transpose(), Ok((1u32, 2.0f32)));
assert_eq!((Ok(1u32), Err::<u64, _>(2.0f32)).transpose(), Err(2.0f32));
assert_eq!((Err::<i32, _>(1u32), Ok(2.0f32)).transpose(), Err(1u32));

// Option
assert_eq!((Some(1u32), Some(2.0f32)).transpose(), Some((1u32, 2.0f32)));
assert_eq!((Some(1u32), None::<f32>).transpose(), None::<(u32, f32)>);
assert_eq!((None::<u32>, Some(2.0f32)).transpose(), None::<(u32, f32)>);
```
