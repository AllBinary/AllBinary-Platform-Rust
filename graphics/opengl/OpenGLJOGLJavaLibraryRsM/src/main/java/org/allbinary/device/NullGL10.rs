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
    fn glActiveTexture(&mut self, mut texture: i32) { Self::glActiveTexture(self, texture) }
    fn glAlphaFunc(&mut self, mut func: i32, mut r#ref: f32) { Self::glAlphaFunc(self, func, r#ref) }
    fn glBindTexture(&mut self, mut target: i32, mut texture: i32) { Self::glBindTexture(self, target, texture) }
    fn glBlendFunc(&mut self, mut sfactor: i32, mut dfactor: i32) { Self::glBlendFunc(self, sfactor, dfactor) }
    fn glClear(&mut self, mut mask: i32) { Self::glClear(self, mask) }
    fn glClearColor(&mut self, mut red: f32, mut green: f32, mut blue: f32, mut alpha: f32) { Self::glClearColor(self, red, green, blue, alpha) }
    fn glClearDepthf(&mut self, mut depth: f32) { Self::glClearDepthf(self, depth) }
    fn glClearStencil(&mut self, mut s: i32) { Self::glClearStencil(self, s) }
    fn glClientActiveTexture(&mut self, mut texture: i32) { Self::glClientActiveTexture(self, texture) }
    fn glColor4f(&mut self, mut red: f32, mut green: f32, mut blue: f32, mut alpha: f32) { Self::glColor4f(self, red, green, blue, alpha) }
    fn glColorMask(&mut self, mut red: bool, mut green: bool, mut blue: bool, mut alpha: bool) { Self::glColorMask(self, red, green, blue, alpha) }
    fn glColorPointer(&mut self, mut size: i32, mut r#type: i32, mut stride: i32, mut pointer: Option<Rc<RefCell<Buffer>>>) { Self::glColorPointer(self, size, r#type, stride, pointer) }
    fn glCompressedTexImage2D(&mut self, mut target: i32, mut level: i32, mut internalformat: i32, mut width: i32, mut height: i32, mut border: i32, mut imageSize: i32, mut data: Option<Rc<RefCell<Buffer>>>) { Self::glCompressedTexImage2D(self, target, level, internalformat, width, height, border, imageSize, data) }
    fn glCompressedTexSubImage2D(&mut self, mut target: i32, mut level: i32, mut xoffset: i32, mut yoffset: i32, mut width: i32, mut height: i32, mut format: i32, mut imageSize: i32, mut data: Option<Rc<RefCell<Buffer>>>) { Self::glCompressedTexSubImage2D(self, target, level, xoffset, yoffset, width, height, format, imageSize, data) }
    fn glCopyTexImage2D(&mut self, mut target: i32, mut level: i32, mut internalformat: i32, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut border: i32) { Self::glCopyTexImage2D(self, target, level, internalformat, x, y, width, height, border) }
    fn glCopyTexSubImage2D(&mut self, mut target: i32, mut level: i32, mut xoffset: i32, mut yoffset: i32, mut x: i32, mut y: i32, mut width: i32, mut height: i32) { Self::glCopyTexSubImage2D(self, target, level, xoffset, yoffset, x, y, width, height) }
    fn glCullFace(&mut self, mut mode: i32) { Self::glCullFace(self, mode) }
    fn glDeleteTextures3(&mut self, mut n: i32, mut textures: Vec<i32>, mut offset: i32) { Self::glDeleteTextures3(self, n, textures, offset) }
    fn glDeleteTextures(&mut self, mut n: i32, mut textures: Option<Rc<RefCell<IntBuffer>>>) { Self::glDeleteTextures(self, n, textures) }
    fn glDepthFunc(&mut self, mut func: i32) { Self::glDepthFunc(self, func) }
    fn glDepthMask(&mut self, mut flag: bool) { Self::glDepthMask(self, flag) }
    fn glDepthRangef(&mut self, mut zNear: f32, mut zFar: f32) { Self::glDepthRangef(self, zNear, zFar) }
    fn glDisable(&mut self, mut cap: i32) { Self::glDisable(self, cap) }
    fn glDisableClientState(&mut self, mut array: i32) { Self::glDisableClientState(self, array) }
    fn glDrawArrays(&mut self, mut mode: i32, mut first: i32, mut count: i32) { Self::glDrawArrays(self, mode, first, count) }
    fn glDrawElements(&mut self, mut mode: i32, mut count: i32, mut r#type: i32, mut indices: Option<Rc<RefCell<Buffer>>>) { Self::glDrawElements(self, mode, count, r#type, indices) }
    fn glEnable(&mut self, mut cap: i32) { Self::glEnable(self, cap) }
    fn glEnableClientState(&mut self, mut array: i32) { Self::glEnableClientState(self, array) }
    fn glFinish(&mut self) { Self::glFinish(self) }
    fn glFlush(&mut self) { Self::glFlush(self) }
    fn glFogf(&mut self, mut pname: i32, mut param: f32) { Self::glFogf(self, pname, param) }
    fn glFogfv3(&mut self, mut pname: i32, mut params: Vec<f32>, mut offset: i32) { Self::glFogfv3(self, pname, params, offset) }
    fn glFogfv(&mut self, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) { Self::glFogfv(self, pname, params) }
    fn glFrontFace(&mut self, mut mode: i32) { Self::glFrontFace(self, mode) }
    fn glFrustumf(&mut self, mut left: f32, mut right: f32, mut bottom: f32, mut top: f32, mut zNear: f32, mut zFar: f32) { Self::glFrustumf(self, left, right, bottom, top, zNear, zFar) }
    fn glGenTextures3(&mut self, mut n: i32, mut textures: Vec<i32>, mut offset: i32) { Self::glGenTextures3(self, n, textures, offset) }
    fn glGenTextures(&mut self, mut n: i32, mut textures: Option<Rc<RefCell<IntBuffer>>>) { Self::glGenTextures(self, n, textures) }
    fn glGetError(&mut self) -> i32 { Self::glGetError(self) }
    fn glGetIntegerv3(&mut self, mut pname: i32, mut params: Vec<i32>, mut offset: i32) { Self::glGetIntegerv3(self, pname, params, offset) }
    fn glGetIntegerv(&mut self, mut pname: i32, mut params: Option<Rc<RefCell<IntBuffer>>>) { Self::glGetIntegerv(self, pname, params) }
    fn glGetString(&mut self, mut name: i32) -> String { Self::glGetString(self, name) }
    fn glHint(&mut self, mut target: i32, mut mode: i32) { Self::glHint(self, target, mode) }
    fn glLightModelf(&mut self, mut pname: i32, mut param: f32) { Self::glLightModelf(self, pname, param) }
    fn glLightModelfv3(&mut self, mut pname: i32, mut params: Vec<f32>, mut offset: i32) { Self::glLightModelfv3(self, pname, params, offset) }
    fn glLightModelfv(&mut self, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) { Self::glLightModelfv(self, pname, params) }
    fn glLightf(&mut self, mut light: i32, mut pname: i32, mut param: f32) { Self::glLightf(self, light, pname, param) }
    fn glLightfv4(&mut self, mut light: i32, mut pname: i32, mut params: Vec<f32>, mut offset: i32) { Self::glLightfv4(self, light, pname, params, offset) }
    fn glLightfv(&mut self, mut light: i32, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) { Self::glLightfv(self, light, pname, params) }
    fn glLineWidth(&mut self, mut width: f32) { Self::glLineWidth(self, width) }
    fn glLoadIdentity(&mut self) { Self::glLoadIdentity(self) }
    fn glLoadMatrixf2(&mut self, mut m: Vec<f32>, mut offset: i32) { Self::glLoadMatrixf2(self, m, offset) }
    fn glLoadMatrixf(&mut self, mut m: Option<Rc<RefCell<FloatBuffer>>>) { Self::glLoadMatrixf(self, m) }
    fn glLogicOp(&mut self, mut opcode: i32) { Self::glLogicOp(self, opcode) }
    fn glMaterialf(&mut self, mut face: i32, mut pname: i32, mut param: f32) { Self::glMaterialf(self, face, pname, param) }
    fn glMaterialfv4(&mut self, mut face: i32, mut pname: i32, mut params: Vec<f32>, mut offset: i32) { Self::glMaterialfv4(self, face, pname, params, offset) }
    fn glMaterialfv(&mut self, mut face: i32, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) { Self::glMaterialfv(self, face, pname, params) }
    fn glMatrixMode(&mut self, mut mode: i32) { Self::glMatrixMode(self, mode) }
    fn glMultMatrixf2(&mut self, mut m: Vec<f32>, mut offset: i32) { Self::glMultMatrixf2(self, m, offset) }
    fn glMultMatrixf(&mut self, mut m: Option<Rc<RefCell<FloatBuffer>>>) { Self::glMultMatrixf(self, m) }
    fn glMultiTexCoord4f(&mut self, mut target: i32, mut s: f32, mut t: f32, mut r: f32, mut q: f32) { Self::glMultiTexCoord4f(self, target, s, t, r, q) }
    fn glNormal3f(&mut self, mut nx: f32, mut ny: f32, mut nz: f32) { Self::glNormal3f(self, nx, ny, nz) }
    fn glNormalPointer(&mut self, mut r#type: i32, mut stride: i32, mut pointer: Option<Rc<RefCell<Buffer>>>) { Self::glNormalPointer(self, r#type, stride, pointer) }
    fn glOrthof(&mut self, mut left: f32, mut right: f32, mut bottom: f32, mut top: f32, mut zNear: f32, mut zFar: f32) { Self::glOrthof(self, left, right, bottom, top, zNear, zFar) }
    fn glPixelStorei(&mut self, mut pname: i32, mut param: i32) { Self::glPixelStorei(self, pname, param) }
    fn glPointSize(&mut self, mut size: f32) { Self::glPointSize(self, size) }
    fn glPolygonOffset(&mut self, mut factor: f32, mut units: f32) { Self::glPolygonOffset(self, factor, units) }
    fn glPopMatrix(&mut self) { Self::glPopMatrix(self) }
    fn glPushMatrix(&mut self) { Self::glPushMatrix(self) }
    fn glReadPixels(&mut self, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut format: i32, mut r#type: i32, mut pixels: Option<Rc<RefCell<Buffer>>>) { Self::glReadPixels(self, x, y, width, height, format, r#type, pixels) }
    fn glRotatef(&mut self, mut angle: f32, mut x: f32, mut y: f32, mut z: f32) { Self::glRotatef(self, angle, x, y, z) }
    fn glRotatex(&mut self, mut angle: i32, mut x: i32, mut y: i32, mut z: i32) { Self::glRotatex(self, angle, x, y, z) }
    fn glSampleCoverage(&mut self, mut value: f32, mut invert: bool) { Self::glSampleCoverage(self, value, invert) }
    fn glScalef(&mut self, mut x: f32, mut y: f32, mut z: f32) { Self::glScalef(self, x, y, z) }
    fn glScalex(&mut self, mut x: i32, mut y: i32, mut z: i32) { Self::glScalex(self, x, y, z) }
    fn glScissor(&mut self, mut x: i32, mut y: i32, mut width: i32, mut height: i32) { Self::glScissor(self, x, y, width, height) }
    fn glShadeModel(&mut self, mut mode: i32) { Self::glShadeModel(self, mode) }
    fn glStencilFunc(&mut self, mut func: i32, mut r#ref: i32, mut mask: i32) { Self::glStencilFunc(self, func, r#ref, mask) }
    fn glStencilMask(&mut self, mut mask: i32) { Self::glStencilMask(self, mask) }
    fn glStencilOp(&mut self, mut fail: i32, mut zfail: i32, mut zpass: i32) { Self::glStencilOp(self, fail, zfail, zpass) }
    fn glTexCoordPointer(&mut self, mut size: i32, mut r#type: i32, mut stride: i32, mut pointer: Option<Rc<RefCell<Buffer>>>) { Self::glTexCoordPointer(self, size, r#type, stride, pointer) }
    fn glTexEnvf(&mut self, mut target: i32, mut pname: i32, mut param: f32) { Self::glTexEnvf(self, target, pname, param) }
    fn glTexEnvfv4(&mut self, mut target: i32, mut pname: i32, mut params: Vec<f32>, mut offset: i32) { Self::glTexEnvfv4(self, target, pname, params, offset) }
    fn glTexEnvfv(&mut self, mut target: i32, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) { Self::glTexEnvfv(self, target, pname, params) }
    fn glTexImage2D(&mut self, mut target: i32, mut level: i32, mut internalformat: i32, mut width: i32, mut height: i32, mut border: i32, mut format: i32, mut r#type: i32, mut pixels: Option<Rc<RefCell<Buffer>>>) { Self::glTexImage2D(self, target, level, internalformat, width, height, border, format, r#type, pixels) }
    fn glTexParameterf(&mut self, mut target: i32, mut pname: i32, mut param: f32) { Self::glTexParameterf(self, target, pname, param) }
    fn glTexParameteri(&mut self, mut target: i32, mut pname: i32, mut param: i32) { Self::glTexParameteri(self, target, pname, param) }
    fn glTexSubImage2D(&mut self, mut target: i32, mut level: i32, mut xoffset: i32, mut yoffset: i32, mut width: i32, mut height: i32, mut format: i32, mut r#type: i32, mut pixels: Option<Rc<RefCell<Buffer>>>) { Self::glTexSubImage2D(self, target, level, xoffset, yoffset, width, height, format, r#type, pixels) }
    fn glTranslatef(&mut self, mut x: f32, mut y: f32, mut z: f32) { Self::glTranslatef(self, x, y, z) }
    fn glVertexPointer(&mut self, mut size: i32, mut r#type: i32, mut stride: i32, mut pointer: Option<Rc<RefCell<Buffer>>>) { Self::glVertexPointer(self, size, r#type, stride, pointer) }
    fn glViewport(&mut self, mut x: i32, mut y: i32, mut width: i32, mut height: i32) { Self::glViewport(self, x, y, width, height) }
    fn glBegin(&mut self, mut i: i32) { Self::glBegin(self, i) }
    fn glEnd(&mut self) { Self::glEnd(self) }
    fn glVertex3f(&mut self, mut f: f32, mut f2: f32, mut f3: f32) { Self::glVertex3f(self, f, f2, f3) }
    fn glPolygonMode(&mut self, mut i: i32, mut i1: i32) { Self::glPolygonMode(self, i, i1) }
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
    

    pub fn glClearDepthf(&mut self, mut depth: f32) {
        return;
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
    

    pub fn glFrontFace(&mut self, mut mode: i32) {
        return;
    }
    

    pub fn glFrustumf(&mut self, mut left: f32, mut right: f32, mut bottom: f32, mut top: f32, mut zNear: f32, mut zFar: f32) {
        return;
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
    

    pub fn glLineWidth(&mut self, mut width: f32) {
        return;
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
    

    pub fn glMultiTexCoord4f(&mut self, mut target: i32, mut s: f32, mut t: f32, mut r: f32, mut q: f32) {
        return;
    }
    

    pub fn glNormal3f(&mut self, mut nx: f32, mut ny: f32, mut nz: f32) {
        return;
    }
    

    pub fn glNormalPointer(&mut self, mut r#type: i32, mut stride: i32, mut pointer: Option<Rc<RefCell<Buffer>>>) {
        return;
    }
    

    pub fn glOrthof(&mut self, mut left: f32, mut right: f32, mut bottom: f32, mut top: f32, mut zNear: f32, mut zFar: f32) {
        return;
    }
    

    pub fn glPixelStorei(&mut self, mut pname: i32, mut param: i32) {
        return;
    }
    

    pub fn glPointSize(&mut self, mut size: f32) {
        return;
    }
    

    pub fn glPolygonOffset(&mut self, mut factor: f32, mut units: f32) {
        return;
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
    

    pub fn glTexImage2D(&mut self, mut target: i32, mut level: i32, mut internalformat: i32, mut width: i32, mut height: i32, mut border: i32, mut format: i32, mut r#type: i32, mut pixels: Option<Rc<RefCell<Buffer>>>) {
        return;
    }
    

    pub fn glTexParameterf(&mut self, mut target: i32, mut pname: i32, mut param: f32) {
        return;
    }
    

    pub fn glTexParameteri(&mut self, mut target: i32, mut pname: i32, mut param: i32) {
        return;
    }
    

    pub fn glTexSubImage2D(&mut self, mut target: i32, mut level: i32, mut xoffset: i32, mut yoffset: i32, mut width: i32, mut height: i32, mut format: i32, mut r#type: i32, mut pixels: Option<Rc<RefCell<Buffer>>>) {
        return;
    }
    

    pub fn glTranslatef(&mut self, mut x: f32, mut y: f32, mut z: f32) {
        return;
    }
    

    pub fn glVertexPointer(&mut self, mut size: i32, mut r#type: i32, mut stride: i32, mut pointer: Option<Rc<RefCell<Buffer>>>) {
        return;
    }
    

    pub fn glViewport(&mut self, mut x: i32, mut y: i32, mut width: i32, mut height: i32) {
        return;
    }
    

    pub fn glBegin(&mut self, mut i: i32) {
        return;
    }
    

    pub fn glEnd(&mut self) {
        return;
    }
    

    pub fn glVertex3f(&mut self, mut f: f32, mut f2: f32, mut f3: f32) {
        return;
    }
    

    pub fn glPolygonMode(&mut self, mut i: i32, mut i1: i32) {
        return;
    }
}

