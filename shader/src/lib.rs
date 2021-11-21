#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]

#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

use spirv_std as _;

#[spirv(vertex)]
pub fn test_vs() {
    let mut x1 = 0.0f32;
    let mut x2 = 1.0f32;

    core::mem::swap(&mut x1, &mut x2);
}
