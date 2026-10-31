// Generated Code Do Not Modify
// package org.platform.opengl
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::nio::ByteBuffer::ByteBuffer;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
use crate::org::allbinary::graphics::OpenGLBitmap::OpenGLBitmap;
use crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLGLUUtils {
}

thread_local! { pub static OpenGLGLUUtils_instance: Option<Rc<RefCell<OpenGLGLUUtils>>> = OpenGLGLUUtils::new(); }

impl OpenGLGLUUtils {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLGLUUtils>>> {
        return OpenGLGLUUtils_instance.with(Clone::clone).clone();
    }
    

    pub fn makeByteBuffer(&mut self, mut bmp: Option<Rc<RefCell<OpenGLBitmap>>>, mut flip: bool) -> Option<Rc<RefCell<ByteBuffer>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn texImage2D(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut glVersion: String, mut target: i32, mut level: i32, mut generateMipMap: bool, mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>>, mut border: i32, mut flip: bool) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn texImage2D(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut glVersion: String, mut target: i32, mut level: i32, mut generateMipMap: bool, mut bitmap: Option<Rc<RefCell<OpenGLBitmap>>>, mut border: i32, mut flip: bool) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}

