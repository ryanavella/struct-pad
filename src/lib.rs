#![no_std]
//! Padding types to enable memory layout optimizations.
//!
//! # Example
//!
//! Basic usage:
//!
//! ```rust
//! use struct_pad::{Pad, PadU0, PadU8, PadU16, PadU32};
//!
//! struct Example {
//!     field1: u64,
//!     field2: u8,
//!     // Padding fields
//!     pad1: PadU8,
//!     #[cfg(target_pointer_width = "16")]
//!     pad2: PadU0,
//!     #[cfg(not(target_pointer_width = "16"))]
//!     pad2: PadU16,
//!     #[cfg(target_pointer_width = "64")]
//!     pad3: PadU32,
//!     #[cfg(not(target_pointer_width = "64"))]
//!     pad3: PadU0,
//! }
//!
//! impl Example {
//!     const fn new(field1: u64, field2: u8) -> Self {
//!         Self {
//!             field1,
//!             field2,
//!             pad1: Pad::VALUE,
//!             pad2: Pad::VALUE,
//!             pad3: Pad::VALUE,
//!         }
//!     }
//! }
//! ```

use core::cmp::Ordering;
use core::hash::{Hash, Hasher};

/// A padding type.
/// 
/// Types implementing `Pad` have only *one* valid bit-pattern.
///
/// This trait is provided so that downstream crates may 
/// construct pad values generically within `const fn`'s.
pub trait Pad: Copy + private::Sealed {
    /// The only valid `Pad` value.
    const VALUE: Self;
}

/// A padding type with the same layout as `()`.
///
/// Like the other padding types, `PadU0` implements the `Pad` trait.
/// However, it occupies no space in memory.
#[derive(Debug)]
pub struct PadU0(());

impl Clone for PadU0 {
    #[inline]
    #[must_use]
    fn clone(&self) -> Self {
        Self::VALUE
    }
}

impl Copy for PadU0 {}

impl Default for PadU0 {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::VALUE
    }
}

impl Eq for PadU0 {}

impl Hash for PadU0 {
    #[inline]
    fn hash<H: Hasher>(&self, _: &mut H) {}
}

impl Ord for PadU0 {
    #[inline]
    #[must_use]
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl Pad for PadU0 {
    const VALUE: Self = Self(());
}

impl PartialEq for PadU0 {
    #[inline]
    #[must_use]
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl PartialOrd for PadU0 {
    #[inline]
    #[must_use]
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

/// A padding type with the same layout as `u8`.
///
/// `PadU8` is implemented as a wrapper around a single-variant enum
/// with an all-zeros bit-pattern.
#[derive(Debug)]
#[repr(transparent)]
pub struct PadU8(PadU8Inner);

impl Clone for PadU8 {
    #[inline]
    #[must_use]
    fn clone(&self) -> Self {
        Self::VALUE
    }
}

impl Copy for PadU8 {}

impl Default for PadU8 {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::VALUE
    }
}

impl Eq for PadU8 {}

impl Hash for PadU8 {
    #[inline]
    fn hash<H: Hasher>(&self, _: &mut H) {}
}

impl Ord for PadU8 {
    #[inline]
    #[must_use]
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl Pad for PadU8 {
    const VALUE: Self = Self(PadU8Inner::VALUE);
}

impl PartialEq for PadU8 {
    #[inline]
    #[must_use]
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl PartialOrd for PadU8 {
    #[inline]
    #[must_use]
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

#[derive(Debug)]
#[repr(u8)]
enum PadU8Inner {
    VALUE = 0,
}

impl Clone for PadU8Inner {
    #[inline]
    #[must_use]
    fn clone(&self) -> Self {
        Self::VALUE
    }
}

impl Copy for PadU8Inner {}

/// A padding type with the same layout as `u16`.
///
/// `PadU16` is implemented as a wrapper around a single-variant enum
/// with an all-zeros bit-pattern.
#[derive(Debug)]
#[repr(transparent)]
pub struct PadU16(PadU16Inner);

impl Clone for PadU16 {
    #[inline]
    #[must_use]
    fn clone(&self) -> Self {
        Self::VALUE
    }
}

impl Copy for PadU16 {}

impl Default for PadU16 {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::VALUE
    }
}

impl Eq for PadU16 {}

impl Hash for PadU16 {
    #[inline]
    fn hash<H: Hasher>(&self, _: &mut H) {}
}

impl Ord for PadU16 {
    #[inline]
    #[must_use]
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl Pad for PadU16 {
    const VALUE: Self = Self(PadU16Inner::VALUE);
}

impl PartialEq for PadU16 {
    #[inline]
    #[must_use]
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl PartialOrd for PadU16 {
    #[inline]
    #[must_use]
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

#[derive(Debug)]
#[repr(u16)]
enum PadU16Inner {
    VALUE = 0,
}

impl Clone for PadU16Inner {
    #[inline]
    #[must_use]
    fn clone(&self) -> Self {
        Self::VALUE
    }
}

impl Copy for PadU16Inner {}

/// A padding type with the same layout as `u32`.
///
/// `PadU32` is implemented as a wrapper around a single-variant enum
/// with an all-zeros bit-pattern.
#[derive(Debug)]
#[repr(transparent)]
pub struct PadU32(PadU32Inner);

impl Clone for PadU32 {
    #[inline]
    #[must_use]
    fn clone(&self) -> Self {
        Self::VALUE
    }
}

impl Copy for PadU32 {}

impl Default for PadU32 {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::VALUE
    }
}

impl Eq for PadU32 {}

impl Hash for PadU32 {
    #[inline]
    fn hash<H: Hasher>(&self, _: &mut H) {}
}

impl Ord for PadU32 {
    #[inline]
    #[must_use]
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl Pad for PadU32 {
    const VALUE: Self = Self(PadU32Inner::VALUE);
}

impl PartialEq for PadU32 {
    #[inline]
    #[must_use]
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl PartialOrd for PadU32 {
    #[inline]
    #[must_use]
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

#[derive(Debug)]
#[repr(u32)]
enum PadU32Inner {
    VALUE = 0,
}

impl Clone for PadU32Inner {
    #[inline]
    #[must_use]
    fn clone(&self) -> Self {
        Self::VALUE
    }
}

impl Copy for PadU32Inner {}

/// A padding type with the same layout as `u64`.
///
/// `PadU64` is implemented as a wrapper around a single-variant enum
/// with an all-zeros bit-pattern.
#[derive(Debug)]
#[repr(transparent)]
pub struct PadU64(PadU64Inner);

impl Clone for PadU64 {
    #[inline]
    #[must_use]
    fn clone(&self) -> Self {
        Self::VALUE
    }
}

impl Copy for PadU64 {}

impl Default for PadU64 {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::VALUE
    }
}

impl Eq for PadU64 {}

impl Hash for PadU64 {
    #[inline]
    fn hash<H: Hasher>(&self, _: &mut H) {}
}

impl Ord for PadU64 {
    #[inline]
    #[must_use]
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl Pad for PadU64 {
    const VALUE: Self = Self(PadU64Inner::VALUE);
}

impl PartialEq for PadU64 {
    #[inline]
    #[must_use]
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl PartialOrd for PadU64 {
    #[inline]
    #[must_use]
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

#[derive(Debug)]
#[repr(u64)]
enum PadU64Inner {
    VALUE = 0,
}

impl Clone for PadU64Inner {
    #[inline]
    #[must_use]
    fn clone(&self) -> Self {
        Self::VALUE
    }
}

impl Copy for PadU64Inner {}

/// A padding type with the same layout as `usize`.
///
/// `PadUsize` is a type alias to whichever padding type is
/// the same size as `usize`.
#[cfg(target_pointer_width = "16")]
pub type PadUsize = PadU16;
/// A padding type with the same layout as `usize`.
///
/// `PadUsize` is a type alias to whichever padding type is
/// the same size as `usize`.
#[cfg(target_pointer_width = "32")]
pub type PadUsize = PadU32;
/// A padding type with the same layout as `usize`.
///
/// `PadUsize` is a type alias to whichever padding type is
/// the same size as `usize`.
#[cfg(target_pointer_width = "64")]
pub type PadUsize = PadU64;

mod private {
    pub use super::*;
    pub trait Sealed {}
    impl Sealed for PadU0 {}
    impl Sealed for PadU8 {}
    impl Sealed for PadU16 {}
    impl Sealed for PadU32 {}
    impl Sealed for PadU64 {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, size_of};

    #[test]
    fn align() {
        assert_eq!(align_of::<PadU0>(), align_of::<()>());
        assert_eq!(align_of::<PadU8>(), align_of::<u8>());
        assert_eq!(align_of::<PadU16>(), align_of::<u16>());
        assert_eq!(align_of::<PadU32>(), align_of::<u32>());
        assert_eq!(align_of::<PadU64>(), align_of::<u64>());
        assert_eq!(align_of::<PadUsize>(), align_of::<usize>());
    }

    #[test]
    fn align_option() {
        assert_eq!(align_of::<Option<PadU0>>(), align_of::<Option<()>>());
        assert_eq!(align_of::<Option<PadU8>>(), align_of::<u8>());
        assert_eq!(align_of::<Option<PadU16>>(), align_of::<u16>());
        assert_eq!(align_of::<Option<PadU32>>(), align_of::<u32>());
        assert_eq!(align_of::<Option<PadU64>>(), align_of::<u64>());
        assert_eq!(align_of::<Option<PadUsize>>(), align_of::<usize>());
    }

    #[test]
    fn size() {
        assert_eq!(size_of::<PadU0>(), size_of::<()>());
        assert_eq!(size_of::<PadU8>(), size_of::<u8>());
        assert_eq!(size_of::<PadU16>(), size_of::<u16>());
        assert_eq!(size_of::<PadU32>(), size_of::<u32>());
        assert_eq!(size_of::<PadU64>(), size_of::<u64>());
        assert_eq!(size_of::<PadUsize>(), size_of::<usize>());
    }

    #[test]
    fn size_option() {
        assert_eq!(size_of::<Option<PadU0>>(), size_of::<Option<()>>());
        assert_eq!(size_of::<Option<PadU8>>(), size_of::<u8>());
        assert_eq!(size_of::<Option<PadU16>>(), size_of::<u16>());
        assert_eq!(size_of::<Option<PadU32>>(), size_of::<u32>());
        assert_eq!(size_of::<Option<PadU64>>(), size_of::<u64>());
        assert_eq!(size_of::<Option<PadUsize>>(), size_of::<usize>());
    }

    #[test]
    fn bit_pattern() {
        assert_eq!(PadU8::VALUE.0 as u8, 0);
        assert_eq!(PadU16::VALUE.0 as u16, 0);
        assert_eq!(PadU32::VALUE.0 as u32, 0);
        assert_eq!(PadU64::VALUE.0 as u64, 0);
        assert_eq!(PadUsize::VALUE.0 as usize, 0);
    }

    #[test]
    fn bit_pattern_default() {
        assert_eq!(PadU8::default().0 as u8, 0);
        assert_eq!(PadU16::default().0 as u16, 0);
        assert_eq!(PadU32::default().0 as u32, 0);
        assert_eq!(PadU64::default().0 as u64, 0);
        assert_eq!(PadUsize::default().0 as usize, 0);
    }
}
