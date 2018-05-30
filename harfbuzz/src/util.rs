// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
pub fn f64_to_fixed(f: f64) -> i32 {
    ((1i32 << 16) as f64 * f) as i32
}

pub fn fixed_to_f64(f: i32) -> f64 {
    f as f64 * 1.0f64 / ((1i32 << 16) as f64)
}
