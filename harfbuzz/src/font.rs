// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]

use {Blob, Tag};
use std;
use sys;
use util;

pub struct Font {
    raw: *mut sys::hb_font_t,
}

impl Font {
    /// Construct a `Font` from a raw pointer. Takes ownership of the font.
    pub unsafe fn from_raw(raw: *mut sys::hb_font_t) -> Self {
        Font { raw }
    }

    /// Borrows a raw pointer to the font.
    pub fn as_raw(&self) -> *mut sys::hb_font_t {
        self.raw
    }

    /// Gives up ownership and returns a raw pointer to the font.
    pub fn into_raw(self) -> *mut sys::hb_font_t {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    pub fn get_table(&self, tag: Tag) -> Blob {
        unsafe {
            let face = sys::hb_font_get_face(self.raw);
            let blob = sys::hb_face_reference_table(face, tag.0);
            Blob::from_raw(blob)
        }
    }

    pub fn set_ppem(&mut self, x_ppem: u32, y_ppem: u32) {
        unsafe { sys::hb_font_set_ppem(self.raw, x_ppem, y_ppem); }
    }

    pub fn set_scale(&mut self, x_scale: f64, y_scale: f64) {
        let x_scale = util::f64_to_fixed(x_scale);
        let y_scale = util::f64_to_fixed(y_scale);
        unsafe { sys::hb_font_set_scale(self.raw, x_scale, y_scale); }
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { sys::hb_font_destroy(self.raw) }
    }
}
