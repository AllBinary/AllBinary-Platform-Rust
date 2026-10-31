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
use crate::com::jogamp::opengl::GL::GL;
use crate::com::jogamp::opengl::GL2::GL2;
use super::JOGL10::JOGL10;
use super::GL11::GL11;
use super::opengles::opengles;
use super::khronos::khronos;
use super::microedition::microedition;
use super::javax::javax;
use super::opengl::opengl;
use super::jogamp::jogamp;
use super::com::com;
use super::Buffer::Buffer;
use super::nio::nio;
use super::java::java;
use super::FloatBuffer::FloatBuffer;
use super::IntBuffer::IntBuffer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//JOGLEToKhronosGL11
pub struct JOGL11 {
    pub base: JOGL10,
    pub gl11: Option<Rc<RefCell<GL2>>>,
}

// impl GL11 for JOGL11 with the methods of the trait

impl JOGL11 {
    

    pub fn new_struct1(mut gl: Option<Rc<RefCell<GL>>>) -> Self {
        let mut this = Self::new_struct2(gl.clone(), (gl).clone() /* as GL2 */);
        // this(...) is in the struct literal above
        this
    }

    pub fn new1(mut gl: Option<Rc<RefCell<GL>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(gl))))
    }
    

    pub fn new_struct2(mut gl: Option<Rc<RefCell<GL>>>, mut gl2: Option<Rc<RefCell<GL2>>>) -> Self {
        let mut this = Self {
            base: JOGL10::new_struct(gl.clone(), gl2.clone()),
            gl11: None,
        };
        // super(...) is in the struct literal above
        this.gl11 = this.getJOGLGL();
        this
    }

    pub fn new2(mut gl: Option<Rc<RefCell<GL>>>, mut gl2: Option<Rc<RefCell<GL2>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(gl, gl2))))
    }
    

    pub fn glBindBuffer(&mut self, mut target: i32, mut buffer: i32) {
        jcall!(self.gl11).glBindBuffer(target.clone(), buffer.clone());
    }
    

    pub fn glBufferData(&mut self, mut target: i32, mut size: i32, mut data: Option<Rc<RefCell<Buffer>>>, mut usage: i32) {
        jcall!(self.gl11).glBufferData(target.clone(), size.clone(), data.clone(), usage.clone());
    }
    

    pub fn glBufferSubData(&mut self, mut target: i32, mut offset: i32, mut size: i32, mut data: Option<Rc<RefCell<Buffer>>>) {
        jcall!(self.gl11).glBufferSubData(target.clone(), offset.clone(), size.clone(), data.clone());
    }
    

    pub fn glClipPlanef3(&mut self, mut plane: i32, mut equation: Vec<f32>, mut offset: i32) {
        jcall!(self.gl11).glClipPlanef(plane.clone(), equation.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glClipPlanef(&mut self, mut plane: i32, mut equation: Option<Rc<RefCell<FloatBuffer>>>) {
        jcall!(self.gl11).glClipPlanef(plane.clone(), equation.clone());
    }
    

    pub fn glColor4ub(&mut self, mut red: i8, mut green: i8, mut blue: i8, mut alpha: i8) {
        jcall!(self.gl11).glColor4ub(red.clone(), green.clone(), blue.clone(), alpha.clone());
    }
    

    pub fn glColorPointer(&mut self, mut size: i32, mut r#type: i32, mut stride: i32, mut offset: i32) {
        jcall!(self.gl11).glColorPointer(size.clone(), r#type.clone(), stride.clone(), offset.clone());
    }
    

    pub fn glDeleteBuffers3(&mut self, mut n: i32, mut buffers: Vec<i32>, mut offset: i32) {
        jcall!(self.gl11).glDeleteBuffers(n.clone(), buffers.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glDeleteBuffers(&mut self, mut n: i32, mut buffers: Option<Rc<RefCell<IntBuffer>>>) {
        jcall!(self.gl11).glDeleteBuffers(n.clone(), buffers.clone());
    }
    

    pub fn glDrawElements(&mut self, mut mode: i32, mut count: i32, mut r#type: i32, mut offset: i32) {
        jcall!(self.gl11).glDrawElements(mode.clone(), count.clone(), r#type.clone(), offset.clone());
    }
    

    pub fn glGenBuffers3(&mut self, mut n: i32, mut buffers: Vec<i32>, mut offset: i32) {
        jcall!(self.gl11).glGenBuffers(n.clone(), buffers.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glGenBuffers(&mut self, mut n: i32, mut buffers: Option<Rc<RefCell<IntBuffer>>>) {
        jcall!(self.gl11).glGenBuffers(n.clone(), buffers.clone());
    }
    

    pub fn glGetBufferParameteriv4(&mut self, mut target: i32, mut pname: i32, mut params: Vec<i32>, mut offset: i32) {
        jcall!(self.gl11).glGetBufferParameteriv(target.clone(), pname.clone(), params.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glGetBufferParameteriv(&mut self, mut target: i32, mut pname: i32, mut params: Option<Rc<RefCell<IntBuffer>>>) {
        jcall!(self.gl11).glGetBufferParameteriv(target.clone(), pname.clone(), params.clone());
    }
    

    pub fn glGetClipPlanef3(&mut self, mut pname: i32, mut eqn: Vec<f32>, mut offset: i32) {
        jcall!(self.gl11).glGetClipPlanef(pname.clone(), eqn.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glGetClipPlanef(&mut self, mut pname: i32, mut eqn: Option<Rc<RefCell<FloatBuffer>>>) {
        jcall!(self.gl11).glGetClipPlanef(pname.clone(), eqn.clone());
    }
    

    pub fn glGetFloatv3(&mut self, mut pname: i32, mut params: Vec<f32>, mut offset: i32) {
        jcall!(self.gl11).glGetFloatv(pname.clone(), params.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glGetFloatv(&mut self, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) {
        jcall!(self.gl11).glGetFloatv(pname.clone(), params.clone());
    }
    

    pub fn glGetLightfv4(&mut self, mut light: i32, mut pname: i32, mut params: Vec<f32>, mut offset: i32) {
        jcall!(self.gl11).glGetLightfv(light.clone(), pname.clone(), params.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glGetLightfv(&mut self, mut light: i32, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) {
        jcall!(self.gl11).glGetLightfv(light.clone(), pname.clone(), params.clone());
    }
    

    pub fn glGetMaterialfv4(&mut self, mut face: i32, mut pname: i32, mut params: Vec<f32>, mut offset: i32) {
        jcall!(self.gl11).glGetMaterialfv(face.clone(), pname.clone(), params.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glGetMaterialfv(&mut self, mut face: i32, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) {
        jcall!(self.gl11).glGetMaterialfv(face.clone(), pname.clone(), params.clone());
    }
    

    pub fn glGetTexEnviv4(&mut self, mut env: i32, mut pname: i32, mut params: Vec<i32>, mut offset: i32) {
        jcall!(self.gl11).glGetTexEnviv(env.clone(), pname.clone(), params.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glGetTexEnviv(&mut self, mut env: i32, mut pname: i32, mut params: Option<Rc<RefCell<IntBuffer>>>) {
        jcall!(self.gl11).glGetTexEnviv(env.clone(), pname.clone(), params.clone());
    }
    

    pub fn glGetTexParameterfv4(&mut self, mut target: i32, mut pname: i32, mut params: Vec<f32>, mut offset: i32) {
        jcall!(self.gl11).glGetTexParameterfv(target.clone(), pname.clone(), params.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glGetTexParameterfv(&mut self, mut target: i32, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) {
        jcall!(self.gl11).glGetTexParameterfv(target.clone(), pname.clone(), params.clone());
    }
    

    pub fn glGetTexParameteriv4(&mut self, mut target: i32, mut pname: i32, mut params: Vec<i32>, mut offset: i32) {
        jcall!(self.gl11).glGetTexParameteriv(target.clone(), pname.clone(), params.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glGetTexParameteriv(&mut self, mut target: i32, mut pname: i32, mut params: Option<Rc<RefCell<IntBuffer>>>) {
        jcall!(self.gl11).glGetTexParameteriv(target.clone(), pname.clone(), params.clone());
    }
    

    pub fn glIsBuffer(&mut self, mut buffer: i32) -> bool {
        return jcall!(self.gl11).glIsBuffer(buffer.clone());
    }
    

    pub fn glIsEnabled(&mut self, mut cap: i32) -> bool {
        return jcall!(self.gl11).glIsEnabled(cap.clone());
    }
    

    pub fn glIsTexture(&mut self, mut texture: i32) -> bool {
        return jcall!(self.gl11).glIsTexture(texture.clone());
    }
    

    pub fn glNormalPointer(&mut self, mut r#type: i32, mut stride: i32, mut offset: i32) {
        jcall!(self.gl11).glNormalPointer(r#type.clone(), stride.clone(), offset.clone());
    }
    

    pub fn glPointParameterf(&mut self, mut pname: i32, mut param: f32) {
        jcall!(self.gl11).glPointParameterf(pname.clone(), param.clone());
    }
    

    pub fn glPointParameterfv3(&mut self, mut pname: i32, mut params: Vec<f32>, mut offset: i32) {
        jcall!(self.gl11).glPointParameterfv(pname.clone(), params.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glPointParameterfv(&mut self, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) {
        jcall!(self.gl11).glPointParameterfv(pname.clone(), params.clone());
    }
    

    pub fn glTexCoordPointer(&mut self, mut size: i32, mut r#type: i32, mut stride: i32, mut offset: i32) {
        jcall!(self.gl11).glTexCoordPointer(size.clone(), r#type.clone(), stride.clone(), offset.clone());
    }
    

    pub fn glTexEnvi(&mut self, mut target: i32, mut pname: i32, mut param: i32) {
        jcall!(self.gl11).glTexEnvi(target.clone(), pname.clone(), param.clone());
    }
    

    pub fn glTexEnviv4(&mut self, mut target: i32, mut pname: i32, mut params: Vec<i32>, mut offset: i32) {
        jcall!(self.gl11).glTexEnviv(target.clone(), pname.clone(), params.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glTexEnviv(&mut self, mut target: i32, mut pname: i32, mut params: Option<Rc<RefCell<IntBuffer>>>) {
        jcall!(self.gl11).glTexEnviv(target.clone(), pname.clone(), params.clone());
    }
    

    pub fn glTexParameterfv4(&mut self, mut target: i32, mut pname: i32, mut params: Vec<f32>, mut offset: i32) {
        jcall!(self.gl11).glTexParameterfv(target.clone(), pname.clone(), params.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glTexParameterfv(&mut self, mut target: i32, mut pname: i32, mut params: Option<Rc<RefCell<FloatBuffer>>>) {
        jcall!(self.gl11).glTexParameterfv(target.clone(), pname.clone(), params.clone());
    }
    

    pub fn glTexParameteri(&mut self, mut target: i32, mut pname: i32, mut param: i32) {
        jcall!(self.gl11).glTexParameteri(target.clone(), pname.clone(), param.clone());
    }
    

    pub fn glTexParameteriv4(&mut self, mut target: i32, mut pname: i32, mut params: Vec<i32>, mut offset: i32) {
        jcall!(self.gl11).glTexParameteriv(target.clone(), pname.clone(), params.clone(), offset.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn glTexParameteriv(&mut self, mut target: i32, mut pname: i32, mut params: Option<Rc<RefCell<IntBuffer>>>) {
        jcall!(self.gl11).glTexParameteriv(target.clone(), pname.clone(), params.clone());
    }
    

    pub fn glTexParameterxv(&mut self, mut target: i32, mut pname: i32, mut params: Vec<i32>, mut offset: i32) {
        jcall!(self.gl11).glTexParameteriv(target.clone(), pname.clone(), params.clone(), offset.clone());
    }
    

    pub fn glVertexPointer(&mut self, mut size: i32, mut r#type: i32, mut stride: i32, mut offset: i32) {
        jcall!(self.gl11).glVertexPointer(size.clone(), r#type.clone(), stride.clone(), offset.clone());
    }
}
impl std::ops::Deref for JOGL11 {
    type Target = JOGL10;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for JOGL11 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::opengles::JOGL10::JOGL10Dyn for JOGL11 {
    fn as_JOGL10(&self) -> &crate::org::allbinary::opengles::JOGL10::JOGL10 { &self.base }
    fn as_JOGL10_mut(&mut self) -> &mut crate::org::allbinary::opengles::JOGL10::JOGL10 { &mut self.base }
    fn glTexParameteri(&mut self, mut target: i32, mut pname: i32, mut param: i32) { Self::glTexParameteri(self, target, pname, param) }
}


