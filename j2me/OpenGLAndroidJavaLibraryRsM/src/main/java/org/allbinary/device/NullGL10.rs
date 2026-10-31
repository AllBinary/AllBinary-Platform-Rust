// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
//  
// By agreeing to this license you and any business entity you represent are 
// legally bound to the AllBinary Open License Version 1 legal agreement. 
//  
// You may obtain the AllBinary Open License Version 1 legal agreement from 
// AllBinary or the root directory of AllBinary's AllBinary Platform repository. 
//  
// Created By: Travis Berthelot 
//   
// Generated Code Do Not Modify
// package org.allbinary.device
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
// use crate::java::nio::Buffer::Buffer;
// use crate::java::nio::FloatBuffer::FloatBuffer;
// use crate::java::nio::IntBuffer::IntBuffer;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullGL10 {
}

thread_local! { pub static NullGL10_NULL_GL10: Option<Rc<RefCell<NullGL10>>> = NullGL10::new(); }
impl GL10 for NullGL10 {
}
impl crate::javax::microedition::khronos::opengles::GL::GL for NullGL10 {
}

impl NullGL10 {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn glActiveTexture(&mut self, mut texture: i32) {
        return;
    }
    

    pub fn glAlphaFunc(&mut self, mut func: i32, mut r#ref: f32) {
        return;
    }
    

    pub fn glAlphaFuncx(&mut self, mut func: i32, mut r#ref: i32) {
    }
    

    pub fn glBindTexture(&mut self, mut target: i32, mut texture: i32) {
        return;
    }
    

    pub fn glBlendFunc(&mut self, mut sfactor: i32, mut dfactor: i32) {
        return;
    }
    

    pub fn glClear(&mut self, mut mask: i32) {
        return;
    }
    

    pub fn glClearColor(&mut self, mut red: f32, mut green: f32, mut blue: f32, mut alpha: f32) {
        return;
    }
    

    pub fn glClearColorx(&mut self, mut red: i32, mut green: i32, mut blue: i32, mut alpha: i32) {
    }
    

    pub fn glClearDepthf(&mut self, mut depth: f32) {
        return;
    }
    

    pub fn glClearDepthx(&mut self, mut depth: i32) {
    }
    

    pub fn glClearStencil(&mut self, mut s: i32) {
        return;
    }
    

    pub fn glClientActiveTexture(&mut self, mut texture: i32) {
        return;
    }
    

    pub fn glColor4f(&mut self, mut red: f32, mut green: f32, mut blue: f32, mut alpha: f32) {
        return;
    }
    

    pub fn glColor4x(&mut self, mut red: i32, mut green: i32, mut blue: i32, mut alpha: i32) {
    }
    

    pub fn glColorMask(&mut self, mut red: bool, mut green: bool, mut blue: bool, mut alpha: bool) {
        return;
    }
    

    pub fn glColorPointer(&mut self, mut size: i32, mut r#type: i32, mut stride: i32, mut pointer: Option<Rc<RefCell<Buffer>>>) {
        return;
    }
    

    pub fn glCompressedTexImage2D(&mut self, mut target: i32, mut level: i32, mut internalformat: i32, mut width: i32, mut height: i32, mut border: i32, mut imageSize: i32, mut data: Option<Rc<RefCell<Buffer>>>) {
        return;
    }
    

    pub fn glCompressedTexSubImage2D(&mut self, mut target: i32, mut level: i32, mut xoffset: i32, mut yoffset: i32, mut width: i32, mut height: i32, mut format: i32, mut imageSize: i32, mut data: Option<Rc<RefCell<Buffer>>>) {
        return;
    }
    

    pub fn glCopyTexImage2D(&mut self, mut target: i32, mut level: i32, mut internalformat: i32, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut border: i32) {
        return;
    }
    

    pub fn glCopyTexSubImage2D(&mut self, mut target: i32, mut level: i32, mut xoffset: i32, mut yoffset: i32, mut x: i32, mut y: i32, mut width: i32, mut height: i32) {
        return;
    }
    

    pub fn glCullFace(&mut self, mut mode: i32) {
        return;
    }
    

    pub fn glDeleteTextures3(&mut self, mut n: i32, mut textures: Vec<i32>, mut offset: i32) {
        return;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glDeleteTextures(&mut self, mut n: i32, mut textures: Option<Rc<RefCell<IntBuffer>>>) {
        return;
    }
    

    pub fn glDepthFunc(&mut self, mut func: i32) {
        return;
    }
    

    pub fn glDepthMask(&mut self, mut flag: bool) {
        return;
    }
    

    pub fn glDepthRangef(&mut self, mut zNear: f32, mut zFar: f32) {
        return;
    }
    

    pub fn glDepthRangex(&mut self, mut zNear: i32, mut zFar: i32) {
    }
    

    pub fn glDisable(&mut self, mut cap: i32) {
        return;
    }
    

    pub fn glDisableClientState(&mut self, mut array: i32) {
        return;
    }
    

    pub fn glDrawArrays(&mut self, mut mode: i32, mut first: i32, mut count: i32) {
        return;
    }
    

    pub fn glDrawElements(&mut self, mut mode: i32, mut count: i32, mut r#type: i32, mut indices: Option<Rc<RefCell<Buffer>>>) {
        return;
    }
    

    pub fn glEnable(&mut self, mut cap: i32) {
        return;
    }
    

    pub fn glEnableClientState(&mut self, mut array: i32) {
        return;
    }
    

    pub fn glFinish(&mut self) {
        return;
    }
    

    pub fn glFlush(&mut self) {
        return;
    }
    

    pub fn glFogf(&mut self, mut pname: i32, mut param: f32) {
        return;
    }
    

    pub fn glFogfv3(&mut self, mut pname: i32, mut params: Vec<f32>, mut offset: i32) {
        return;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glFogfv(&mut self, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) {
        return;
    }
    

    pub fn glFogx(&mut self, mut pname: i32, mut param: i32) {
    }
    

    pub fn glFogxv3(&mut self, mut pname: i32, mut params: Vec<i32>, mut offset: i32) {
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glFogxv(&mut self, mut pname: i32, mut params: Option<Rc<RefCell<IntBuffer>>>) {
    }
    

    pub fn glFrontFace(&mut self, mut mode: i32) {
        return;
    }
    

    pub fn glFrustumf(&mut self, mut left: f32, mut right: f32, mut bottom: f32, mut top: f32, mut zNear: f32, mut zFar: f32) {
        return;
    }
    

    pub fn glFrustumx(&mut self, mut left: i32, mut right: i32, mut bottom: i32, mut top: i32, mut zNear: i32, mut zFar: i32) {
    }
    

    pub fn glGenTextures3(&mut self, mut n: i32, mut textures: Vec<i32>, mut offset: i32) {
        return;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glGenTextures(&mut self, mut n: i32, mut textures: Option<Rc<RefCell<IntBuffer>>>) {
        return;
    }
    

    pub fn glGetError(&mut self) -> i32 {
        return 0;
    }
    

    pub fn glGetIntegerv3(&mut self, mut pname: i32, mut params: Vec<i32>, mut offset: i32) {
        return;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glGetIntegerv(&mut self, mut pname: i32, mut params: Option<Rc<RefCell<IntBuffer>>>) {
        return;
    }
    

    pub fn glGetString(&mut self, mut name: i32) -> String {
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
    

    pub fn glHint(&mut self, mut target: i32, mut mode: i32) {
        return;
    }
    

    pub fn glLightModelf(&mut self, mut pname: i32, mut param: f32) {
        return;
    }
    

    pub fn glLightModelfv3(&mut self, mut pname: i32, mut params: Vec<f32>, mut offset: i32) {
        return;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glLightModelfv(&mut self, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) {
        return;
    }
    

    pub fn glLightModelx(&mut self, mut pname: i32, mut param: i32) {
    }
    

    pub fn glLightModelxv3(&mut self, mut pname: i32, mut params: Vec<i32>, mut offset: i32) {
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glLightModelxv(&mut self, mut pname: i32, mut params: Option<Rc<RefCell<IntBuffer>>>) {
    }
    

    pub fn glLightf(&mut self, mut light: i32, mut pname: i32, mut param: f32) {
        return;
    }
    

    pub fn glLightfv4(&mut self, mut light: i32, mut pname: i32, mut params: Vec<f32>, mut offset: i32) {
        return;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glLightfv(&mut self, mut light: i32, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) {
        return;
    }
    

    pub fn glLightx(&mut self, mut light: i32, mut pname: i32, mut param: i32) {
    }
    

    pub fn glLightxv4(&mut self, mut light: i32, mut pname: i32, mut params: Vec<i32>, mut offset: i32) {
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glLightxv(&mut self, mut light: i32, mut pname: i32, mut params: Option<Rc<RefCell<IntBuffer>>>) {
    }
    

    pub fn glLineWidth(&mut self, mut width: f32) {
        return;
    }
    

    pub fn glLineWidthx(&mut self, mut width: i32) {
    }
    

    pub fn glLoadIdentity(&mut self) {
        return;
    }
    

    pub fn glLoadMatrixf2(&mut self, mut m: Vec<f32>, mut offset: i32) {
        return;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glLoadMatrixf(&mut self, mut m: Option<Rc<RefCell<FloatBuffer>>>) {
        return;
    }
    

    pub fn glLoadMatrixx2(&mut self, mut m: Vec<i32>, mut offset: i32) {
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glLoadMatrixx(&mut self, mut m: Option<Rc<RefCell<IntBuffer>>>) {
    }
    

    pub fn glLogicOp(&mut self, mut opcode: i32) {
        return;
    }
    

    pub fn glMaterialf(&mut self, mut face: i32, mut pname: i32, mut param: f32) {
        return;
    }
    

    pub fn glMaterialfv4(&mut self, mut face: i32, mut pname: i32, mut params: Vec<f32>, mut offset: i32) {
        return;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glMaterialfv(&mut self, mut face: i32, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) {
        return;
    }
    

    pub fn glMaterialx(&mut self, mut face: i32, mut pname: i32, mut param: i32) {
    }
    

    pub fn glMaterialxv4(&mut self, mut face: i32, mut pname: i32, mut params: Vec<i32>, mut offset: i32) {
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glMaterialxv(&mut self, mut face: i32, mut pname: i32, mut params: Option<Rc<RefCell<IntBuffer>>>) {
    }
    

    pub fn glMatrixMode(&mut self, mut mode: i32) {
        return;
    }
    

    pub fn glMultMatrixf2(&mut self, mut m: Vec<f32>, mut offset: i32) {
        return;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glMultMatrixf(&mut self, mut m: Option<Rc<RefCell<FloatBuffer>>>) {
        return;
    }
    

    pub fn glMultMatrixx2(&mut self, mut m: Vec<i32>, mut offset: i32) {
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glMultMatrixx(&mut self, mut m: Option<Rc<RefCell<IntBuffer>>>) {
    }
    

    pub fn glMultiTexCoord4f(&mut self, mut target: i32, mut s: f32, mut t: f32, mut r: f32, mut q: f32) {
        return;
    }
    

    pub fn glMultiTexCoord4x(&mut self, mut target: i32, mut s: i32, mut t: i32, mut r: i32, mut q: i32) {
    }
    

    pub fn glNormal3f(&mut self, mut nx: f32, mut ny: f32, mut nz: f32) {
        return;
    }
    

    pub fn glNormal3x(&mut self, mut nx: i32, mut ny: i32, mut nz: i32) {
    }
    

    pub fn glNormalPointer(&mut self, mut r#type: i32, mut stride: i32, mut pointer: Option<Rc<RefCell<Buffer>>>) {
        return;
    }
    

    pub fn glOrthof(&mut self, mut left: f32, mut right: f32, mut bottom: f32, mut top: f32, mut zNear: f32, mut zFar: f32) {
        return;
    }
    

    pub fn glOrthox(&mut self, mut left: i32, mut right: i32, mut bottom: i32, mut top: i32, mut zNear: i32, mut zFar: i32) {
    }
    

    pub fn glPixelStorei(&mut self, mut pname: i32, mut param: i32) {
        return;
    }
    

    pub fn glPointSize(&mut self, mut size: f32) {
        return;
    }
    

    pub fn glPointSizex(&mut self, mut size: i32) {
    }
    

    pub fn glPolygonOffset(&mut self, mut factor: f32, mut units: f32) {
        return;
    }
    

    pub fn glPolygonOffsetx(&mut self, mut factor: i32, mut units: i32) {
    }
    

    pub fn glPopMatrix(&mut self) {
        return;
    }
    

    pub fn glPushMatrix(&mut self) {
        return;
    }
    

    pub fn glReadPixels(&mut self, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut format: i32, mut r#type: i32, mut pixels: Option<Rc<RefCell<Buffer>>>) {
        return;
    }
    

    pub fn glRotatef(&mut self, mut angle: f32, mut x: f32, mut y: f32, mut z: f32) {
        return;
    }
    

    pub fn glRotatex(&mut self, mut angle: i32, mut x: i32, mut y: i32, mut z: i32) {
        return;
    }
    

    pub fn glSampleCoverage(&mut self, mut value: f32, mut invert: bool) {
        return;
    }
    

    pub fn glSampleCoveragex(&mut self, mut value: i32, mut invert: bool) {
    }
    

    pub fn glScalef(&mut self, mut x: f32, mut y: f32, mut z: f32) {
        return;
    }
    

    pub fn glScalex(&mut self, mut x: i32, mut y: i32, mut z: i32) {
        return;
    }
    

    pub fn glScissor(&mut self, mut x: i32, mut y: i32, mut width: i32, mut height: i32) {
        return;
    }
    

    pub fn glShadeModel(&mut self, mut mode: i32) {
        return;
    }
    

    pub fn glStencilFunc(&mut self, mut func: i32, mut r#ref: i32, mut mask: i32) {
        return;
    }
    

    pub fn glStencilMask(&mut self, mut mask: i32) {
        return;
    }
    

    pub fn glStencilOp(&mut self, mut fail: i32, mut zfail: i32, mut zpass: i32) {
        return;
    }
    

    pub fn glTexCoordPointer(&mut self, mut size: i32, mut r#type: i32, mut stride: i32, mut pointer: Option<Rc<RefCell<Buffer>>>) {
        return;
    }
    

    pub fn glTexEnvf(&mut self, mut target: i32, mut pname: i32, mut param: f32) {
        return;
    }
    

    pub fn glTexEnvfv4(&mut self, mut target: i32, mut pname: i32, mut params: Vec<f32>, mut offset: i32) {
        return;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glTexEnvfv(&mut self, mut target: i32, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) {
        return;
    }
    

    pub fn glTexEnvx(&mut self, mut target: i32, mut pname: i32, mut param: i32) {
    }
    

    pub fn glTexEnvxv4(&mut self, mut target: i32, mut pname: i32, mut params: Vec<i32>, mut offset: i32) {
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glTexEnvxv(&mut self, mut target: i32, mut pname: i32, mut params: Option<Rc<RefCell<IntBuffer>>>) {
    }
    

    pub fn glTexImage2D(&mut self, mut target: i32, mut level: i32, mut internalformat: i32, mut width: i32, mut height: i32, mut border: i32, mut format: i32, mut r#type: i32, mut pixels: Option<Rc<RefCell<Buffer>>>) {
        return;
    }
    

    pub fn glTexParameterf(&mut self, mut target: i32, mut pname: i32, mut param: f32) {
        return;
    }
    

    pub fn glTexParameterx(&mut self, mut target: i32, mut pname: i32, mut param: i32) {
    }
    

    pub fn glTexSubImage2D(&mut self, mut target: i32, mut level: i32, mut xoffset: i32, mut yoffset: i32, mut width: i32, mut height: i32, mut format: i32, mut r#type: i32, mut pixels: Option<Rc<RefCell<Buffer>>>) {
        return;
    }
    

    pub fn glTranslatef(&mut self, mut x: f32, mut y: f32, mut z: f32) {
        return;
    }
    

    pub fn glTranslatex(&mut self, mut x: i32, mut y: i32, mut z: i32) {
    }
    

    pub fn glVertexPointer(&mut self, mut size: i32, mut r#type: i32, mut stride: i32, mut pointer: Option<Rc<RefCell<Buffer>>>) {
        return;
    }
    

    pub fn glViewport(&mut self, mut x: i32, mut y: i32, mut width: i32, mut height: i32) {
        return;
    }
}

