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
// package org.allbinary.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FullScreenUtil {
}

thread_local! { pub static FullScreenUtil_instance: Option<Rc<RefCell<FullScreenUtil>>> = FullScreenUtil::new(); }

impl FullScreenUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<FullScreenUtil>>> {
        return FullScreenUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn initOnRun(&mut self, mut fullScreenInterface: Option<Rc<RefCell<Canvas>>>, mut commandListener: Option<Rc<RefCell<CommandListener>>>) {
    }
    

    pub fn init(&mut self, mut fullScreenInterface: Option<Rc<RefCell<Canvas>>>) {
    }
    

    pub fn isScreenChange(&mut self, mut isFullScreen: bool) -> bool {
        return false;
    }
    

    pub fn isScreenChangeCanvas(&mut self, mut fullScreenInterface: Option<Rc<RefCell<Canvas>>>) -> bool {
        return { let java_arguments = (jcall!(fullScreenInterface).isFullScreenMode(),); self.isScreenChange(java_arguments.0) };
    }
}

