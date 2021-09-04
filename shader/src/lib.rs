#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]

#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

use spirv_std as _;

#[derive(PartialEq, Eq)]
struct T(u32);

#[spirv(fragment)]
pub fn main(out: &mut f32) {
    if T(0) == T(0) {
        *out = 1.0;
    }
}
