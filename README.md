# struct-pad

Padding types to enable memory layout optimizations.


[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/ryanavella/struct-pad/blob/master/LICENSE-MIT) [![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](https://github.com/ryanavella/struct-pad/blob/master/LICENSE-UNLICENSE) [![crates.io](https://img.shields.io/crates/v/struct-pad.svg?colorB=319e8c)](https://crates.io/crates/struct-pad) [![docs.rs](https://img.shields.io/badge/docs.rs-struct--pad-yellowgreen)](https://docs.rs/struct-pad)

## Example

```rust
struct Example {
    field1: u64,
    field2: u8,
    // Padding fields
    pad1: PadU8,
    #[cfg(target_pointer_width = "16")]
    pad2: PadU0,
    #[cfg(not(target_pointer_width = "16"))]
    pad2: PadU16,
    #[cfg(target_pointer_width = "64")]
    pad3: PadU32,
    #[cfg(not(target_pointer_width = "64"))]
    pad3: PadU0,
 }

impl Example {
    const fn new(field1: u64, field2: u8) -> Self {
        Self {
            field1,
            field2,
            pad1: Pad::VALUE,
            pad2: Pad::VALUE,
            pad3: Pad::VALUE,
        }
    }
}
```
