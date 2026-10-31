// Generated Code Do Not Modify
// package org.platform.opengl
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLGLU {
}

thread_local! { pub static OpenGLGLU_instance: Option<Rc<RefCell<OpenGLGLU>>> = OpenGLGLU::new(); }

impl OpenGLGLU {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLGLU>>> {
        return OpenGLGLU_instance.with(Clone::clone).clone();
    }
    

    pub fn gluLookAt(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut eyeX: f32, mut eyeY: f32, mut eyeZ: f32, mut centerX: f32, mut centerY: f32, mut centerZ: f32, mut upX: f32, mut upY: f32, mut upZ: f32) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn gluProject(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut objX: f32, mut objY: f32, mut objZ: f32, mut model: Vec<f32>, mut modelOffset: i32, mut project: Vec<f32>, mut projectOffset: i32, mut view: Vec<i32>, mut viewOffset: i32, mut win: Vec<f32>, mut winOffset: i32) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn gluUnProject(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut winX: f32, mut winY: f32, mut winZ: f32, mut model: Vec<f32>, mut modelOffset: i32, mut project: Vec<f32>, mut projectOffset: i32, mut view: Vec<i32>, mut viewOffset: i32, mut obj: Vec<f32>, mut objOffset: i32) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn glFrustumf(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut lt: f32, mut rt: f32, mut btm: f32, mut top: f32, mut zNear: f32, mut zFar: f32) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}

