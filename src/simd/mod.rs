//! Traits implemented by SIMD types and non-SIMD types.

pub use self::auto_simd_impl::*;
pub use self::simd_bool::SimdBool;
pub use self::simd_complex::SimdComplexField;
// FIXME: Feature gate should be removed when std::simd makes it into stable rustc.
#[cfg(feature = "nightly")]
pub use self::simd_impl::*;
pub use self::simd_option::SimdOption;
pub use self::simd_partial_ord::SimdPartialOrd;
pub use self::simd_real::SimdRealField;
pub use self::simd_signed::SimdSigned;
pub use self::simd_value::{PrimitiveSimdValue, SimdValue};
#[cfg(feature = "wide")]
pub use self::wide_simd_impl::{
    WideBoolF32x4, WideBoolF32x8, WideBoolF64x4, WideF32x4, WideF32x8, WideF64x4,
};

mod auto_simd_impl;
#[cfg(feature = "rand")]
mod rand_impl;
mod simd_bool;
mod simd_complex;
// FIXME: Feature gate should be removed when std::simd makes it into stable rustc.
#[cfg(feature = "nightly")]
mod simd_impl;
mod simd_option;
mod simd_partial_ord;
mod simd_real;
mod simd_signed;
mod simd_value;
#[cfg(feature = "wide")]
mod wide_simd_impl;
