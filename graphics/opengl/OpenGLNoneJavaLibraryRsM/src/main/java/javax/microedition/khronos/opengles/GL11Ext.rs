// Generated Code Do Not Modify
// package javax.microedition.khronos.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::nio::Buffer::Buffer;
// use crate::java::nio::FloatBuffer::FloatBuffer;
// use crate::java::nio::IntBuffer::IntBuffer;
// use crate::java::nio::ShortBuffer::ShortBuffer;
use super::GL::GL;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait GL11Ext: GL {
    

    fn glTexParameterfv(&mut self, target: i32, pname: i32, param: Vec<f32>, offset: i32);
    

    fn glCurrentPaletteMatrixOES(&mut self, matrixpaletteindex: i32);
    

    fn glDrawTexfOES(&mut self, x: f32, y: f32, z: f32, width: f32, height: f32);
    

    fn glDrawTexfvOES2(&mut self, coords: Vec<f32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glDrawTexfvOES(&mut self, coords: Option<Rc<RefCell<FloatBuffer>>>);
    

    fn glDrawTexiOES(&mut self, x: i32, y: i32, z: i32, width: i32, height: i32);
    

    fn glDrawTexivOES2(&mut self, coords: Vec<i32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glDrawTexivOES(&mut self, coords: Option<Rc<RefCell<IntBuffer>>>);
    

    fn glDrawTexsOES(&mut self, x: i16, y: i16, z: i16, width: i16, height: i16);
    

    fn glDrawTexsvOES2(&mut self, coords: Vec<i16>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glDrawTexsvOES(&mut self, coords: Option<Rc<RefCell<ShortBuffer>>>);
    

    fn glDrawTexxOES(&mut self, x: i32, y: i32, z: i32, width: i32, height: i32);
    

    fn glDrawTexxvOES2(&mut self, coords: Vec<i32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glDrawTexxvOES(&mut self, coords: Option<Rc<RefCell<IntBuffer>>>);
    

    fn glEnable(&mut self, cap: i32);
    

    fn glEnableClientState(&mut self, array: i32);
    

    fn glLoadPaletteFromModelViewMatrixOES(&mut self);
    

    fn glMatrixIndexPointerOES(&mut self, size: i32, r#type: i32, stride: i32, pointer: Option<Rc<RefCell<Buffer>>>);
    

    // Java overloaded method: Rust needs a different name
    fn glMatrixIndexPointerOES(&mut self, size: i32, r#type: i32, stride: i32, offset: i32);
    

    fn glWeightPointerOES(&mut self, size: i32, r#type: i32, stride: i32, pointer: Option<Rc<RefCell<Buffer>>>);
    

    // Java overloaded method: Rust needs a different name
    fn glWeightPointerOES(&mut self, size: i32, r#type: i32, stride: i32, offset: i32);
}

thread_local! { pub static GL11Ext_GL_MATRIX_INDEX_ARRAY_BUFFER_BINDING_OES: i32 = 35742; }
thread_local! { pub static GL11Ext_GL_MATRIX_INDEX_ARRAY_OES: i32 = 34884; }
thread_local! { pub static GL11Ext_GL_MATRIX_INDEX_ARRAY_POINTER_OES: i32 = 34889; }
thread_local! { pub static GL11Ext_GL_MATRIX_INDEX_ARRAY_SIZE_OES: i32 = 34886; }
thread_local! { pub static GL11Ext_GL_MATRIX_INDEX_ARRAY_STRIDE_OES: i32 = 34888; }
thread_local! { pub static GL11Ext_GL_MATRIX_INDEX_ARRAY_TYPE_OES: i32 = 34887; }
thread_local! { pub static GL11Ext_GL_MATRIX_PALETTE_OES: i32 = 34880; }
thread_local! { pub static GL11Ext_GL_MAX_PALETTE_MATRICES_OES: i32 = 34882; }
thread_local! { pub static GL11Ext_GL_MAX_VERTEX_UNITS_OES: i32 = 34468; }
thread_local! { pub static GL11Ext_GL_TEXTURE_CROP_RECT_OES: i32 = 35741; }
thread_local! { pub static GL11Ext_GL_WEIGHT_ARRAY_BUFFER_BINDING_OES: i32 = 34974; }
thread_local! { pub static GL11Ext_GL_WEIGHT_ARRAY_OES: i32 = 34477; }
thread_local! { pub static GL11Ext_GL_WEIGHT_ARRAY_POINTER_OES: i32 = 34476; }
thread_local! { pub static GL11Ext_GL_WEIGHT_ARRAY_SIZE_OES: i32 = 34475; }
thread_local! { pub static GL11Ext_GL_WEIGHT_ARRAY_STRIDE_OES: i32 = 34474; }
thread_local! { pub static GL11Ext_GL_WEIGHT_ARRAY_TYPE_OES: i32 = 34473; }

