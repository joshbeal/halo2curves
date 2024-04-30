#![cfg_attr(feature = "asm", feature(asm_const))]

mod arithmetic;

pub mod bn256;
#[cfg(not(all(target_os = "zkvm", target_vendor = "succinct")))]
pub mod grumpkin;
pub mod pairing;
pub mod pasta;
pub mod secp256k1;
pub mod serde;

#[macro_use]
mod derive;
pub use arithmetic::CurveAffineExt;
pub use pasta_curves::arithmetic::{Coordinates, CurveAffine, CurveExt};

// Re-export ff and group to simplify down stream dependencies
#[cfg(feature = "reexport")]
pub use ff;
#[cfg(not(feature = "reexport"))]
use ff;
#[cfg(feature = "reexport")]
pub use group;
#[cfg(not(feature = "reexport"))]
use group;

#[cfg(test)]
pub mod tests;

#[cfg(all(feature = "prefetch", target_arch = "x86_64"))]
#[inline(always)]
pub fn prefetch<T>(data: &[T], offset: usize) {
    use core::arch::x86_64::_mm_prefetch;
    unsafe {
        _mm_prefetch(
            data.as_ptr().add(offset) as *const i8,
            core::arch::x86_64::_MM_HINT_T0,
        );
    }
}
