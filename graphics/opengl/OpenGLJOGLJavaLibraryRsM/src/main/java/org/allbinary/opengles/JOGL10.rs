//
// AllBinary Open License Version 1
// Copyright (c) 2011 AllBinary
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
// package org.allbinary.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::com::jogamp::opengl::glu::GLU::GLU;
use super::GL10::GL10;
use super::opengles::opengles;
use super::khronos::khronos;
use super::microedition::microedition;
use super::javax::javax;
use super::GL2::GL2;
use super::opengl::opengl;
use super::jogamp::jogamp;
use super::com::com;
use super::GL::GL;
use super::Buffer::Buffer;
use super::nio::nio;
use super::java::java;
use super::IntBuffer::IntBuffer;
use super::FloatBuffer::FloatBuffer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//JOGLEToKhronosGL10
pub struct JOGL10 {
    pub RED: String,
    pub GREEN: String,
    pub BLUE: String,
    pub ALPHA: String,
    pub PNAME: String,
    pub _PNAME: String,
    pub PARAM: String,
    pub PARAMS: String,
    pub LIGHT: String,
    pub FACE: String,
    pub TARGET: String,
    pub gl10: Option<Rc<RefCell<GL2>>>,
    pub glu: Option<Rc<RefCell<GLU>>>,
    pub GL_LINE: i32,
    pub GL_QUAD_STRIP: i32,
}

// impl GL10 for JOGL10 with the methods of the trait

impl JOGL10 {
    

    pub fn new_struct(mut gl: Option<Rc<RefCell<GL>>>, mut gl2: Option<Rc<RefCell<GL2>>>) -> Self {
        let mut this = Self {
            RED: String::new(),
            GREEN: String::new(),
            BLUE: String::new(),
            ALPHA: String::new(),
            PNAME: String::new(),
            _PNAME: String::new(),
            PARAM: String::new(),
            PARAMS: String::new(),
            LIGHT: String::new(),
            FACE: String::new(),
            TARGET: String::new(),
            gl10: None,
            glu: None,
            GL_LINE: 0,
            GL_QUAD_STRIP: 0,
        };
        this.RED = String::from("red: ");
        this.GREEN = String::from(" green: ");
        this.BLUE = String::from(" blue: ");
        this.ALPHA = String::from(" alpha: ");
        this.PNAME = String::from("pname: ");
        this._PNAME = String::from(" pname: ");
        this.PARAM = String::from(" param: ");
        this.PARAMS = String::from(" params: ");
        this.LIGHT = String::from("light: ");
        this.FACE = String::from("face: ");
        this.TARGET = String::from("target: ");
        this.gl10 = gl2.clone();
        this.glu = GLU::createGLU(this.gl10.clone());
        this.GL_LINE = jcall!(jcall!(jcall!(unsafe { com_jogamp.clone() }).opengl).GL2).GL_LINE.clone();
        this.GL_QUAD_STRIP = jcall!(jcall!(jcall!(unsafe { com_jogamp.clone() }).opengl).GL2).GL_QUAD_STRIP.clone();
        this
    }

    pub fn new(mut gl: Option<Rc<RefCell<GL>>>, mut gl2: Option<Rc<RefCell<GL2>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gl, gl2))))
    }
    

    pub fn getJOGLGL(&mut self) -> Option<Rc<RefCell<GL2>>> {
        return self.gl10.clone();
    }
    

    pub fn glActiveTexture(&mut self, mut texture: i32) {
        jcall!(self.gl10).glActiveTexture(texture.clone());
    }
    

    pub fn glAlphaFunc(&mut self, mut func: i32, mut r#ref: f32) {
        jcall!(self.gl10).glAlphaFunc(func.clone(), r#ref.clone());
    }
    

    pub fn glBindTexture(&mut self, mut target: i32, mut texture: i32) {
        jcall!(self.gl10).glBindTexture(target.clone(), texture.clone());
    }
    

    pub fn glBlendFunc(&mut self, mut sfactor: i32, mut dfactor: i32) {
        jcall!(self.gl10).glBlendFunc(sfactor.clone(), dfactor.clone());
    }
    

    pub fn glClear(&mut self, mut mask: i32) {
        jcall!(self.gl10).glClear(mask.clone());
    }
    

    pub fn glClearColor(&mut self, mut red: f32, mut green: f32, mut blue: f32, mut alpha: f32) {
        jcall!(self.gl10).glClearColor(red.clone(), green.clone(), blue.clone(), alpha.clone());
    }
    

    pub fn glClearDepthf(&mut self, mut depth: f32) {
        jcall!(self.gl10).glClearDepthf(depth.clone());
    }
    

    pub fn glClearStencil(&mut self, mut s: i32) {
        jcall!(self.gl10).glClearStencil(s.clone());
    }
    

    pub fn glClientActiveTexture(&mut self, mut texture: i32) {
        jcall!(self.gl10).glClientActiveTexture(texture.clone());
    }
    

    pub fn glColor4f(&mut self, mut red: f32, mut green: f32, mut blue: f32, mut alpha: f32) {
        jcall!(self.gl10).glColor4f(red.clone(), green.clone(), blue.clone(), alpha.clone());
    }
    

    pub fn glColorMask(&mut self, mut red: bool, mut green: bool, mut blue: bool, mut alpha: bool) {
        jcall!(self.gl10).glColorMask(red.clone(), green.clone(), blue.clone(), alpha.clone());
    }
    

    pub fn glColorPointer(&mut self, mut size: i32, mut r#type: i32, mut stride: i32, mut pointer: Option<Rc<RefCell<Buffer>>>) {
        jcall!(self.gl10).glColorPointer(size.clone(), r#type.clone(), stride.clone(), pointer.clone());
    }
    

    pub fn glCompressedTexImage2D(&mut self, mut target: i32, mut level: i32, mut internalformat: i32, mut width: i32, mut height: i32, mut border: i32, mut imageSize: i32, mut data: Option<Rc<RefCell<Buffer>>>) {
        jcall!(self.gl10).glCompressedTexImage2D(target.clone(), level.clone(), internalformat.clone(), width.clone(), height.clone(), border.clone(), imageSize.clone(), data.clone());
    }
    

    pub fn glCompressedTexSubImage2D(&mut self, mut target: i32, mut level: i32, mut xoffset: i32, mut yoffset: i32, mut width: i32, mut height: i32, mut format: i32, mut imageSize: i32, mut data: Option<Rc<RefCell<Buffer>>>) {
        jcall!(self.gl10).glCompressedTexSubImage2D(target.clone(), level.clone(), xoffset.clone(), yoffset.clone(), width.clone(), height.clone(), format.clone(), imageSize.clone(), data.clone());
    }
    

    pub fn glCopyTexImage2D(&mut self, mut target: i32, mut level: i32, mut internalformat: i32, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut border: i32) {
        jcall!(self.gl10).glCopyTexImage2D(target.clone(), level.clone(), internalformat.clone(), x.clone(), y.clone(), width.clone(), height.clone(), border.clone());
    }
    

    pub fn glCopyTexSubImage2D(&mut self, mut target: i32, mut level: i32, mut xoffset: i32, mut yoffset: i32, mut x: i32, mut y: i32, mut width: i32, mut height: i32) {
        jcall!(self.gl10).glCopyTexSubImage2D(target.clone(), level.clone(), xoffset.clone(), yoffset.clone(), x.clone(), y.clone(), width.clone(), height.clone());
    }
    

    pub fn glCullFace(&mut self, mut mode: i32) {
        jcall!(self.gl10).glCullFace(mode.clone());
    }
    

    pub fn glDeleteTextures3(&mut self, mut n: i32, mut textures: Vec<i32>, mut offset: i32) {
        jcall!(self.gl10).glDeleteTextures(n.clone(), textures.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glDeleteTextures(&mut self, mut n: i32, mut textures: Option<Rc<RefCell<IntBuffer>>>) {
        jcall!(self.gl10).glDeleteTextures(n.clone(), textures.clone());
    }
    

    pub fn glDepthFunc(&mut self, mut func: i32) {
        jcall!(self.gl10).glDepthFunc(func.clone());
    }
    

    pub fn glDepthMask(&mut self, mut flag: bool) {
        jcall!(self.gl10).glDepthMask(flag.clone());
    }
    

    pub fn glDepthRangef(&mut self, mut zNear: f32, mut zFar: f32) {
        jcall!(self.gl10).glDepthRangef(zNear.clone(), zFar.clone());
    }
    

    pub fn glDisable(&mut self, mut cap: i32) {
        jcall!(self.gl10).glDisable(cap.clone());
    }
    

    pub fn glDisableClientState(&mut self, mut array: i32) {
        jcall!(self.gl10).glDisableClientState(array.clone());
    }
    

    pub fn glDrawArrays(&mut self, mut mode: i32, mut first: i32, mut count: i32) {
        jcall!(self.gl10).glDrawArrays(mode.clone(), first.clone(), count.clone());
    }
    

    pub fn glDrawElements(&mut self, mut mode: i32, mut count: i32, mut r#type: i32, mut indices: Option<Rc<RefCell<Buffer>>>) {
        jcall!(self.gl10).glDrawElements(mode.clone(), count.clone(), r#type.clone(), indices.clone());
    }
    

    pub fn glEnable(&mut self, mut cap: i32) {
        jcall!(self.gl10).glEnable(cap.clone());
    }
    

    pub fn glEnableClientState(&mut self, mut array: i32) {
        jcall!(self.gl10).glEnableClientState(array.clone());
    }
    

    pub fn glFinish(&mut self) {
        jcall!(self.gl10).glFinish();
    }
    

    pub fn glFlush(&mut self) {
        jcall!(self.gl10).glFlush();
    }
    

    pub fn glFogf(&mut self, mut pname: i32, mut param: f32) {
        jcall!(self.gl10).glFogf(pname.clone(), param.clone());
    }
    

    pub fn glFogfv3(&mut self, mut pname: i32, mut params: Vec<f32>, mut offset: i32) {
        jcall!(self.gl10).glFogfv(pname.clone(), params.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glFogfv(&mut self, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) {
        jcall!(self.gl10).glFogfv(pname.clone(), params.clone());
    }
    

    pub fn glFrontFace(&mut self, mut mode: i32) {
        jcall!(self.gl10).glFrontFace(mode.clone());
    }
    

    pub fn glFrustumf(&mut self, mut left: f32, mut right: f32, mut bottom: f32, mut top: f32, mut zNear: f32, mut zFar: f32) {
        jcall!(self.gl10).glFrustumf(left.clone(), right.clone(), bottom.clone(), top.clone(), zNear.clone(), zFar.clone());
    }
    

    pub fn glGenTextures3(&mut self, mut n: i32, mut textures: Vec<i32>, mut offset: i32) {
        jcall!(self.gl10).glGenTextures(n.clone(), textures.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glGenTextures(&mut self, mut n: i32, mut textures: Option<Rc<RefCell<IntBuffer>>>) {
        jcall!(self.gl10).glGenTextures(n.clone(), textures.clone());
    }
    

    pub fn glGetError(&mut self) -> i32 {
        return jcall!(self.gl10).glGetError();
    }
    

    pub fn glGetIntegerv3(&mut self, mut pname: i32, mut params: Vec<i32>, mut offset: i32) {
        jcall!(self.gl10).glGetIntegerv(pname.clone(), params.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glGetIntegerv(&mut self, mut pname: i32, mut params: Option<Rc<RefCell<IntBuffer>>>) {
        jcall!(self.gl10).glGetIntegerv(pname.clone(), params.clone());
    }
    

    pub fn glGetString(&mut self, mut name: i32) -> String {
        return jcall!(self.gl10).glGetString(name.clone());
    }
    

    pub fn glHint(&mut self, mut target: i32, mut mode: i32) {
        jcall!(self.gl10).glHint(target.clone(), mode.clone());
    }
    

    pub fn glLightModelf(&mut self, mut pname: i32, mut param: f32) {
        jcall!(self.gl10).glLightModelf(pname.clone(), param.clone());
    }
    

    pub fn glLightModelfv3(&mut self, mut pname: i32, mut params: Vec<f32>, mut offset: i32) {
        jcall!(self.gl10).glLightModelfv(pname.clone(), params.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glLightModelfv(&mut self, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) {
        jcall!(self.gl10).glLightModelfv(pname.clone(), params.clone());
    }
    

    pub fn glLightf(&mut self, mut light: i32, mut pname: i32, mut param: f32) {
        jcall!(self.gl10).glLightf(light.clone(), pname.clone(), param.clone());
    }
    

    pub fn glLightfv4(&mut self, mut light: i32, mut pname: i32, mut params: Vec<f32>, mut offset: i32) {
        jcall!(self.gl10).glLightfv(light.clone(), pname.clone(), params.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glLightfv(&mut self, mut light: i32, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) {
        jcall!(self.gl10).glLightfv(light.clone(), pname.clone(), params.clone());
    }
    

    pub fn glLineWidth(&mut self, mut width: f32) {
        jcall!(self.gl10).glLineWidth(width.clone());
    }
    

    pub fn glLoadIdentity(&mut self) {
        jcall!(self.gl10).glLoadIdentity();
    }
    

    pub fn glLoadMatrixf2(&mut self, mut m: Vec<f32>, mut offset: i32) {
        jcall!(self.gl10).glLoadMatrixf(m.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glLoadMatrixf(&mut self, mut m: Option<Rc<RefCell<FloatBuffer>>>) {
        jcall!(self.gl10).glLoadMatrixf(m.clone());
    }
    

    pub fn glLogicOp(&mut self, mut opcode: i32) {
        jcall!(self.gl10).glLogicOp(opcode.clone());
    }
    

    pub fn glMaterialf(&mut self, mut face: i32, mut pname: i32, mut param: f32) {
        jcall!(self.gl10).glMaterialf(face.clone(), pname.clone(), param.clone());
    }
    

    pub fn glMaterialfv4(&mut self, mut face: i32, mut pname: i32, mut params: Vec<f32>, mut offset: i32) {
        jcall!(self.gl10).glMaterialfv(face.clone(), pname.clone(), params.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glMaterialfv(&mut self, mut face: i32, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) {
        jcall!(self.gl10).glMaterialfv(face.clone(), pname.clone(), params.clone());
    }
    

    pub fn glMatrixMode(&mut self, mut mode: i32) {
        jcall!(self.gl10).glMatrixMode(mode.clone());
    }
    

    pub fn glMultMatrixf2(&mut self, mut m: Vec<f32>, mut offset: i32) {
        jcall!(self.gl10).glMultMatrixf(m.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glMultMatrixf(&mut self, mut m: Option<Rc<RefCell<FloatBuffer>>>) {
        jcall!(self.gl10).glMultMatrixf(m.clone());
    }
    

    pub fn glMultiTexCoord4f(&mut self, mut target: i32, mut s: f32, mut t: f32, mut r: f32, mut q: f32) {
        jcall!(self.gl10).glMultiTexCoord4f(target.clone(), s.clone(), t.clone(), r.clone(), q.clone());
    }
    

    pub fn glNormal3f(&mut self, mut nx: f32, mut ny: f32, mut nz: f32) {
        jcall!(self.gl10).glNormal3f(nx.clone(), ny.clone(), nz.clone());
    }
    

    pub fn glNormalPointer(&mut self, mut r#type: i32, mut stride: i32, mut pointer: Option<Rc<RefCell<Buffer>>>) {
        jcall!(self.gl10).glNormalPointer(r#type.clone(), stride.clone(), pointer.clone());
    }
    

    pub fn glOrthof(&mut self, mut left: f32, mut right: f32, mut bottom: f32, mut top: f32, mut zNear: f32, mut zFar: f32) {
        jcall!(self.gl10).glOrthof(left.clone(), right.clone(), bottom.clone(), top.clone(), zNear.clone(), zFar.clone());
    }
    

    pub fn glPixelStorei(&mut self, mut pname: i32, mut param: i32) {
        jcall!(self.gl10).glPixelStorei(pname.clone(), param.clone());
    }
    

    pub fn glPointSize(&mut self, mut size: f32) {
        jcall!(self.gl10).glPointSize(size.clone());
    }
    

    pub fn glPolygonOffset(&mut self, mut factor: f32, mut units: f32) {
        jcall!(self.gl10).glPolygonOffset(factor.clone(), units.clone());
    }
    

    pub fn glPopMatrix(&mut self) {
        jcall!(self.gl10).glPopMatrix();
    }
    

    pub fn glPushMatrix(&mut self) {
        jcall!(self.gl10).glPushMatrix();
    }
    

    pub fn glReadPixels(&mut self, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut format: i32, mut r#type: i32, mut pixels: Option<Rc<RefCell<Buffer>>>) {
        jcall!(self.gl10).glReadPixels(x.clone(), y.clone(), width.clone(), height.clone(), format.clone(), r#type.clone(), pixels.clone());
    }
    

    pub fn glRotatef(&mut self, mut angle: f32, mut x: f32, mut y: f32, mut z: f32) {
        jcall!(self.gl10).glRotatef(angle.clone(), x.clone(), y.clone(), z.clone());
    }
    

    pub fn glRotatex(&mut self, mut angle: i32, mut x: i32, mut y: i32, mut z: i32) {
        jcall!(self.gl10).glRotatef(angle.clone(), x.clone(), y.clone(), z.clone());
    }
    

    pub fn glSampleCoverage(&mut self, mut value: f32, mut invert: bool) {
        jcall!(self.gl10).glSampleCoverage(value.clone(), invert.clone());
    }
    

    pub fn glScalef(&mut self, mut x: f32, mut y: f32, mut z: f32) {
        jcall!(self.gl10).glScalef(x.clone(), y.clone(), z.clone());
    }
    

    pub fn glScalex(&mut self, mut x: i32, mut y: i32, mut z: i32) {
        jcall!(self.gl10).glScalef(x.clone(), y.clone(), z.clone());
    }
    

    pub fn glScissor(&mut self, mut x: i32, mut y: i32, mut width: i32, mut height: i32) {
        jcall!(self.gl10).glScissor(x.clone(), y.clone(), width.clone(), height.clone());
    }
    

    pub fn glShadeModel(&mut self, mut mode: i32) {
        jcall!(self.gl10).glShadeModel(mode.clone());
    }
    

    pub fn glStencilFunc(&mut self, mut func: i32, mut r#ref: i32, mut mask: i32) {
        jcall!(self.gl10).glStencilFunc(func.clone(), r#ref.clone(), mask.clone());
    }
    

    pub fn glStencilMask(&mut self, mut mask: i32) {
        jcall!(self.gl10).glStencilMask(mask.clone());
    }
    

    pub fn glStencilOp(&mut self, mut fail: i32, mut zfail: i32, mut zpass: i32) {
        jcall!(self.gl10).glStencilOp(fail.clone(), zfail.clone(), zpass.clone());
    }
    

    pub fn glTexCoordPointer(&mut self, mut size: i32, mut r#type: i32, mut stride: i32, mut pointer: Option<Rc<RefCell<Buffer>>>) {
        jcall!(self.gl10).glTexCoordPointer(size.clone(), r#type.clone(), stride.clone(), pointer.clone());
    }
    

    pub fn glTexEnvf(&mut self, mut target: i32, mut pname: i32, mut param: f32) {
        jcall!(self.gl10).glTexEnvf(target.clone(), pname.clone(), param.clone());
    }
    

    pub fn glTexEnvfv4(&mut self, mut target: i32, mut pname: i32, mut params: Vec<f32>, mut offset: i32) {
        jcall!(self.gl10).glTexEnvfv(target.clone(), pname.clone(), params.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glTexEnvfv(&mut self, mut target: i32, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) {
        jcall!(self.gl10).glTexEnvfv(target.clone(), pname.clone(), params.clone());
    }
    

    pub fn glTexImage2D(&mut self, mut target: i32, mut level: i32, mut internalformat: i32, mut width: i32, mut height: i32, mut border: i32, mut format: i32, mut r#type: i32, mut pixels: Option<Rc<RefCell<Buffer>>>) {
        jcall!(self.gl10).glTexImage2D(target.clone(), level.clone(), internalformat.clone(), width.clone(), height.clone(), border.clone(), format.clone(), r#type.clone(), pixels.clone());
    }
    

    pub fn glTexParameterf(&mut self, mut target: i32, mut pname: i32, mut param: f32) {
        jcall!(self.gl10).glTexParameterf(target.clone(), pname.clone(), param.clone());
    }
    

    pub fn glTexParameteri(&mut self, mut target: i32, mut pname: i32, mut param: i32) {
        jcall!(self.gl10).glTexParameteri(target.clone(), pname.clone(), param.clone());
    }
    

    pub fn glTexSubImage2D(&mut self, mut target: i32, mut level: i32, mut xoffset: i32, mut yoffset: i32, mut width: i32, mut height: i32, mut format: i32, mut r#type: i32, mut pixels: Option<Rc<RefCell<Buffer>>>) {
        jcall!(self.gl10).glTexSubImage2D(target.clone(), level.clone(), xoffset.clone(), yoffset.clone(), width.clone(), height.clone(), format.clone(), r#type.clone(), pixels.clone());
    }
    

    pub fn glTranslatef(&mut self, mut x: f32, mut y: f32, mut z: f32) {
        jcall!(self.gl10).glTranslatef(x.clone(), y.clone(), z.clone());
    }
    

    pub fn glVertexPointer(&mut self, mut size: i32, mut r#type: i32, mut stride: i32, mut pointer: Option<Rc<RefCell<Buffer>>>) {
        jcall!(self.gl10).glVertexPointer(size.clone(), r#type.clone(), stride.clone(), pointer.clone());
    }
    

    pub fn glViewport(&mut self, mut x: i32, mut y: i32, mut width: i32, mut height: i32) {
        jcall!(self.gl10).glViewport(x.clone(), y.clone(), width.clone(), height.clone());
    }
    

    pub fn glBegin(&mut self, mut i: i32) {
        jcall!(self.gl10).glBegin(i.clone());
    }
    

    pub fn glEnd(&mut self) {
        jcall!(self.gl10).glEnd();
    }
    

    pub fn glVertex3f(&mut self, mut f: f32, mut f2: f32, mut f3: f32) {
        jcall!(self.gl10).glVertex3f(f.clone(), f2.clone(), f3.clone());
    }
    

    pub fn glPolygonMode(&mut self, mut i: i32, mut i1: i32) {
        jcall!(self.gl10).glPolygonMode(i.clone(), i1.clone());
    }
}
pub trait JOGL10Dyn {
    fn as_JOGL10(&self) -> &JOGL10;
    fn as_JOGL10_mut(&mut self) -> &mut JOGL10;
    fn getJOGLGL(&mut self) -> Option<Rc<RefCell<GL2>>> { JOGL10::getJOGLGL(self.as_JOGL10_mut()) }
    fn glActiveTexture(&mut self, mut texture: i32) { JOGL10::glActiveTexture(self.as_JOGL10_mut(), texture) }
    fn glAlphaFunc(&mut self, mut func: i32, mut r#ref: f32) { JOGL10::glAlphaFunc(self.as_JOGL10_mut(), func, r#ref) }
    fn glBindTexture(&mut self, mut target: i32, mut texture: i32) { JOGL10::glBindTexture(self.as_JOGL10_mut(), target, texture) }
    fn glBlendFunc(&mut self, mut sfactor: i32, mut dfactor: i32) { JOGL10::glBlendFunc(self.as_JOGL10_mut(), sfactor, dfactor) }
    fn glClear(&mut self, mut mask: i32) { JOGL10::glClear(self.as_JOGL10_mut(), mask) }
    fn glClearColor(&mut self, mut red: f32, mut green: f32, mut blue: f32, mut alpha: f32) { JOGL10::glClearColor(self.as_JOGL10_mut(), red, green, blue, alpha) }
    fn glClearDepthf(&mut self, mut depth: f32) { JOGL10::glClearDepthf(self.as_JOGL10_mut(), depth) }
    fn glClearStencil(&mut self, mut s: i32) { JOGL10::glClearStencil(self.as_JOGL10_mut(), s) }
    fn glClientActiveTexture(&mut self, mut texture: i32) { JOGL10::glClientActiveTexture(self.as_JOGL10_mut(), texture) }
    fn glColor4f(&mut self, mut red: f32, mut green: f32, mut blue: f32, mut alpha: f32) { JOGL10::glColor4f(self.as_JOGL10_mut(), red, green, blue, alpha) }
    fn glColorMask(&mut self, mut red: bool, mut green: bool, mut blue: bool, mut alpha: bool) { JOGL10::glColorMask(self.as_JOGL10_mut(), red, green, blue, alpha) }
    fn glColorPointer(&mut self, mut size: i32, mut r#type: i32, mut stride: i32, mut pointer: Option<Rc<RefCell<Buffer>>>) { JOGL10::glColorPointer(self.as_JOGL10_mut(), size, r#type, stride, pointer) }
    fn glCompressedTexImage2D(&mut self, mut target: i32, mut level: i32, mut internalformat: i32, mut width: i32, mut height: i32, mut border: i32, mut imageSize: i32, mut data: Option<Rc<RefCell<Buffer>>>) { JOGL10::glCompressedTexImage2D(self.as_JOGL10_mut(), target, level, internalformat, width, height, border, imageSize, data) }
    fn glCompressedTexSubImage2D(&mut self, mut target: i32, mut level: i32, mut xoffset: i32, mut yoffset: i32, mut width: i32, mut height: i32, mut format: i32, mut imageSize: i32, mut data: Option<Rc<RefCell<Buffer>>>) { JOGL10::glCompressedTexSubImage2D(self.as_JOGL10_mut(), target, level, xoffset, yoffset, width, height, format, imageSize, data) }
    fn glCopyTexImage2D(&mut self, mut target: i32, mut level: i32, mut internalformat: i32, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut border: i32) { JOGL10::glCopyTexImage2D(self.as_JOGL10_mut(), target, level, internalformat, x, y, width, height, border) }
    fn glCopyTexSubImage2D(&mut self, mut target: i32, mut level: i32, mut xoffset: i32, mut yoffset: i32, mut x: i32, mut y: i32, mut width: i32, mut height: i32) { JOGL10::glCopyTexSubImage2D(self.as_JOGL10_mut(), target, level, xoffset, yoffset, x, y, width, height) }
    fn glCullFace(&mut self, mut mode: i32) { JOGL10::glCullFace(self.as_JOGL10_mut(), mode) }
    fn glDeleteTextures3(&mut self, mut n: i32, mut textures: Vec<i32>, mut offset: i32) { JOGL10::glDeleteTextures3(self.as_JOGL10_mut(), n, textures, offset) }
    fn glDeleteTextures(&mut self, mut n: i32, mut textures: Option<Rc<RefCell<IntBuffer>>>) { JOGL10::glDeleteTextures(self.as_JOGL10_mut(), n, textures) }
    fn glDepthFunc(&mut self, mut func: i32) { JOGL10::glDepthFunc(self.as_JOGL10_mut(), func) }
    fn glDepthMask(&mut self, mut flag: bool) { JOGL10::glDepthMask(self.as_JOGL10_mut(), flag) }
    fn glDepthRangef(&mut self, mut zNear: f32, mut zFar: f32) { JOGL10::glDepthRangef(self.as_JOGL10_mut(), zNear, zFar) }
    fn glDisable(&mut self, mut cap: i32) { JOGL10::glDisable(self.as_JOGL10_mut(), cap) }
    fn glDisableClientState(&mut self, mut array: i32) { JOGL10::glDisableClientState(self.as_JOGL10_mut(), array) }
    fn glDrawArrays(&mut self, mut mode: i32, mut first: i32, mut count: i32) { JOGL10::glDrawArrays(self.as_JOGL10_mut(), mode, first, count) }
    fn glDrawElements(&mut self, mut mode: i32, mut count: i32, mut r#type: i32, mut indices: Option<Rc<RefCell<Buffer>>>) { JOGL10::glDrawElements(self.as_JOGL10_mut(), mode, count, r#type, indices) }
    fn glEnable(&mut self, mut cap: i32) { JOGL10::glEnable(self.as_JOGL10_mut(), cap) }
    fn glEnableClientState(&mut self, mut array: i32) { JOGL10::glEnableClientState(self.as_JOGL10_mut(), array) }
    fn glFinish(&mut self) { JOGL10::glFinish(self.as_JOGL10_mut()) }
    fn glFlush(&mut self) { JOGL10::glFlush(self.as_JOGL10_mut()) }
    fn glFogf(&mut self, mut pname: i32, mut param: f32) { JOGL10::glFogf(self.as_JOGL10_mut(), pname, param) }
    fn glFogfv3(&mut self, mut pname: i32, mut params: Vec<f32>, mut offset: i32) { JOGL10::glFogfv3(self.as_JOGL10_mut(), pname, params, offset) }
    fn glFogfv(&mut self, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) { JOGL10::glFogfv(self.as_JOGL10_mut(), pname, params) }
    fn glFrontFace(&mut self, mut mode: i32) { JOGL10::glFrontFace(self.as_JOGL10_mut(), mode) }
    fn glFrustumf(&mut self, mut left: f32, mut right: f32, mut bottom: f32, mut top: f32, mut zNear: f32, mut zFar: f32) { JOGL10::glFrustumf(self.as_JOGL10_mut(), left, right, bottom, top, zNear, zFar) }
    fn glGenTextures3(&mut self, mut n: i32, mut textures: Vec<i32>, mut offset: i32) { JOGL10::glGenTextures3(self.as_JOGL10_mut(), n, textures, offset) }
    fn glGenTextures(&mut self, mut n: i32, mut textures: Option<Rc<RefCell<IntBuffer>>>) { JOGL10::glGenTextures(self.as_JOGL10_mut(), n, textures) }
    fn glGetError(&mut self) -> i32 { JOGL10::glGetError(self.as_JOGL10_mut()) }
    fn glGetIntegerv3(&mut self, mut pname: i32, mut params: Vec<i32>, mut offset: i32) { JOGL10::glGetIntegerv3(self.as_JOGL10_mut(), pname, params, offset) }
    fn glGetIntegerv(&mut self, mut pname: i32, mut params: Option<Rc<RefCell<IntBuffer>>>) { JOGL10::glGetIntegerv(self.as_JOGL10_mut(), pname, params) }
    fn glGetString(&mut self, mut name: i32) -> String { JOGL10::glGetString(self.as_JOGL10_mut(), name) }
    fn glHint(&mut self, mut target: i32, mut mode: i32) { JOGL10::glHint(self.as_JOGL10_mut(), target, mode) }
    fn glLightModelf(&mut self, mut pname: i32, mut param: f32) { JOGL10::glLightModelf(self.as_JOGL10_mut(), pname, param) }
    fn glLightModelfv3(&mut self, mut pname: i32, mut params: Vec<f32>, mut offset: i32) { JOGL10::glLightModelfv3(self.as_JOGL10_mut(), pname, params, offset) }
    fn glLightModelfv(&mut self, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) { JOGL10::glLightModelfv(self.as_JOGL10_mut(), pname, params) }
    fn glLightf(&mut self, mut light: i32, mut pname: i32, mut param: f32) { JOGL10::glLightf(self.as_JOGL10_mut(), light, pname, param) }
    fn glLightfv4(&mut self, mut light: i32, mut pname: i32, mut params: Vec<f32>, mut offset: i32) { JOGL10::glLightfv4(self.as_JOGL10_mut(), light, pname, params, offset) }
    fn glLightfv(&mut self, mut light: i32, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) { JOGL10::glLightfv(self.as_JOGL10_mut(), light, pname, params) }
    fn glLineWidth(&mut self, mut width: f32) { JOGL10::glLineWidth(self.as_JOGL10_mut(), width) }
    fn glLoadIdentity(&mut self) { JOGL10::glLoadIdentity(self.as_JOGL10_mut()) }
    fn glLoadMatrixf2(&mut self, mut m: Vec<f32>, mut offset: i32) { JOGL10::glLoadMatrixf2(self.as_JOGL10_mut(), m, offset) }
    fn glLoadMatrixf(&mut self, mut m: Option<Rc<RefCell<FloatBuffer>>>) { JOGL10::glLoadMatrixf(self.as_JOGL10_mut(), m) }
    fn glLogicOp(&mut self, mut opcode: i32) { JOGL10::glLogicOp(self.as_JOGL10_mut(), opcode) }
    fn glMaterialf(&mut self, mut face: i32, mut pname: i32, mut param: f32) { JOGL10::glMaterialf(self.as_JOGL10_mut(), face, pname, param) }
    fn glMaterialfv4(&mut self, mut face: i32, mut pname: i32, mut params: Vec<f32>, mut offset: i32) { JOGL10::glMaterialfv4(self.as_JOGL10_mut(), face, pname, params, offset) }
    fn glMaterialfv(&mut self, mut face: i32, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) { JOGL10::glMaterialfv(self.as_JOGL10_mut(), face, pname, params) }
    fn glMatrixMode(&mut self, mut mode: i32) { JOGL10::glMatrixMode(self.as_JOGL10_mut(), mode) }
    fn glMultMatrixf2(&mut self, mut m: Vec<f32>, mut offset: i32) { JOGL10::glMultMatrixf2(self.as_JOGL10_mut(), m, offset) }
    fn glMultMatrixf(&mut self, mut m: Option<Rc<RefCell<FloatBuffer>>>) { JOGL10::glMultMatrixf(self.as_JOGL10_mut(), m) }
    fn glMultiTexCoord4f(&mut self, mut target: i32, mut s: f32, mut t: f32, mut r: f32, mut q: f32) { JOGL10::glMultiTexCoord4f(self.as_JOGL10_mut(), target, s, t, r, q) }
    fn glNormal3f(&mut self, mut nx: f32, mut ny: f32, mut nz: f32) { JOGL10::glNormal3f(self.as_JOGL10_mut(), nx, ny, nz) }
    fn glNormalPointer(&mut self, mut r#type: i32, mut stride: i32, mut pointer: Option<Rc<RefCell<Buffer>>>) { JOGL10::glNormalPointer(self.as_JOGL10_mut(), r#type, stride, pointer) }
    fn glOrthof(&mut self, mut left: f32, mut right: f32, mut bottom: f32, mut top: f32, mut zNear: f32, mut zFar: f32) { JOGL10::glOrthof(self.as_JOGL10_mut(), left, right, bottom, top, zNear, zFar) }
    fn glPixelStorei(&mut self, mut pname: i32, mut param: i32) { JOGL10::glPixelStorei(self.as_JOGL10_mut(), pname, param) }
    fn glPointSize(&mut self, mut size: f32) { JOGL10::glPointSize(self.as_JOGL10_mut(), size) }
    fn glPolygonOffset(&mut self, mut factor: f32, mut units: f32) { JOGL10::glPolygonOffset(self.as_JOGL10_mut(), factor, units) }
    fn glPopMatrix(&mut self) { JOGL10::glPopMatrix(self.as_JOGL10_mut()) }
    fn glPushMatrix(&mut self) { JOGL10::glPushMatrix(self.as_JOGL10_mut()) }
    fn glReadPixels(&mut self, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut format: i32, mut r#type: i32, mut pixels: Option<Rc<RefCell<Buffer>>>) { JOGL10::glReadPixels(self.as_JOGL10_mut(), x, y, width, height, format, r#type, pixels) }
    fn glRotatef(&mut self, mut angle: f32, mut x: f32, mut y: f32, mut z: f32) { JOGL10::glRotatef(self.as_JOGL10_mut(), angle, x, y, z) }
    fn glRotatex(&mut self, mut angle: i32, mut x: i32, mut y: i32, mut z: i32) { JOGL10::glRotatex(self.as_JOGL10_mut(), angle, x, y, z) }
    fn glSampleCoverage(&mut self, mut value: f32, mut invert: bool) { JOGL10::glSampleCoverage(self.as_JOGL10_mut(), value, invert) }
    fn glScalef(&mut self, mut x: f32, mut y: f32, mut z: f32) { JOGL10::glScalef(self.as_JOGL10_mut(), x, y, z) }
    fn glScalex(&mut self, mut x: i32, mut y: i32, mut z: i32) { JOGL10::glScalex(self.as_JOGL10_mut(), x, y, z) }
    fn glScissor(&mut self, mut x: i32, mut y: i32, mut width: i32, mut height: i32) { JOGL10::glScissor(self.as_JOGL10_mut(), x, y, width, height) }
    fn glShadeModel(&mut self, mut mode: i32) { JOGL10::glShadeModel(self.as_JOGL10_mut(), mode) }
    fn glStencilFunc(&mut self, mut func: i32, mut r#ref: i32, mut mask: i32) { JOGL10::glStencilFunc(self.as_JOGL10_mut(), func, r#ref, mask) }
    fn glStencilMask(&mut self, mut mask: i32) { JOGL10::glStencilMask(self.as_JOGL10_mut(), mask) }
    fn glStencilOp(&mut self, mut fail: i32, mut zfail: i32, mut zpass: i32) { JOGL10::glStencilOp(self.as_JOGL10_mut(), fail, zfail, zpass) }
    fn glTexCoordPointer(&mut self, mut size: i32, mut r#type: i32, mut stride: i32, mut pointer: Option<Rc<RefCell<Buffer>>>) { JOGL10::glTexCoordPointer(self.as_JOGL10_mut(), size, r#type, stride, pointer) }
    fn glTexEnvf(&mut self, mut target: i32, mut pname: i32, mut param: f32) { JOGL10::glTexEnvf(self.as_JOGL10_mut(), target, pname, param) }
    fn glTexEnvfv4(&mut self, mut target: i32, mut pname: i32, mut params: Vec<f32>, mut offset: i32) { JOGL10::glTexEnvfv4(self.as_JOGL10_mut(), target, pname, params, offset) }
    fn glTexEnvfv(&mut self, mut target: i32, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) { JOGL10::glTexEnvfv(self.as_JOGL10_mut(), target, pname, params) }
    fn glTexImage2D(&mut self, mut target: i32, mut level: i32, mut internalformat: i32, mut width: i32, mut height: i32, mut border: i32, mut format: i32, mut r#type: i32, mut pixels: Option<Rc<RefCell<Buffer>>>) { JOGL10::glTexImage2D(self.as_JOGL10_mut(), target, level, internalformat, width, height, border, format, r#type, pixels) }
    fn glTexParameterf(&mut self, mut target: i32, mut pname: i32, mut param: f32) { JOGL10::glTexParameterf(self.as_JOGL10_mut(), target, pname, param) }
    fn glTexParameteri(&mut self, mut target: i32, mut pname: i32, mut param: i32) { JOGL10::glTexParameteri(self.as_JOGL10_mut(), target, pname, param) }
    fn glTexSubImage2D(&mut self, mut target: i32, mut level: i32, mut xoffset: i32, mut yoffset: i32, mut width: i32, mut height: i32, mut format: i32, mut r#type: i32, mut pixels: Option<Rc<RefCell<Buffer>>>) { JOGL10::glTexSubImage2D(self.as_JOGL10_mut(), target, level, xoffset, yoffset, width, height, format, r#type, pixels) }
    fn glTranslatef(&mut self, mut x: f32, mut y: f32, mut z: f32) { JOGL10::glTranslatef(self.as_JOGL10_mut(), x, y, z) }
    fn glVertexPointer(&mut self, mut size: i32, mut r#type: i32, mut stride: i32, mut pointer: Option<Rc<RefCell<Buffer>>>) { JOGL10::glVertexPointer(self.as_JOGL10_mut(), size, r#type, stride, pointer) }
    fn glViewport(&mut self, mut x: i32, mut y: i32, mut width: i32, mut height: i32) { JOGL10::glViewport(self.as_JOGL10_mut(), x, y, width, height) }
    fn glBegin(&mut self, mut i: i32) { JOGL10::glBegin(self.as_JOGL10_mut(), i) }
    fn glEnd(&mut self) { JOGL10::glEnd(self.as_JOGL10_mut()) }
    fn glVertex3f(&mut self, mut f: f32, mut f2: f32, mut f3: f32) { JOGL10::glVertex3f(self.as_JOGL10_mut(), f, f2, f3) }
    fn glPolygonMode(&mut self, mut i: i32, mut i1: i32) { JOGL10::glPolygonMode(self.as_JOGL10_mut(), i, i1) }
}
impl JOGL10Dyn for JOGL10 {
    fn as_JOGL10(&self) -> &JOGL10 { self }
    fn as_JOGL10_mut(&mut self) -> &mut JOGL10 { self }
}


