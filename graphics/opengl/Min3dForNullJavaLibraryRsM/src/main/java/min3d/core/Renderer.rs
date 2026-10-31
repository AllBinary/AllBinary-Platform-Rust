// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package min3d.core
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::egl::EGLConfig::EGLConfig;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait Renderer {
    

    fn onSurfaceCreated(&mut self, gl: Option<Rc<RefCell<dyn GL10>>>, config: Option<Rc<RefCell<EGLConfig>>>);
    

    fn onSurfaceChanged(&mut self, gl: Option<Rc<RefCell<dyn GL10>>>, width: i32, height: i32);
    

    fn onDrawFrame(&mut self, gl: Option<Rc<RefCell<dyn GL10>>>);

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


