#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]

use spirv_std::glam::Vec4;
#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

#[spirv(closest_hit)]
pub fn test_closest_hit(#[spirv(object_to_world)] _object_to_world: [Vec4; 3]) {}
