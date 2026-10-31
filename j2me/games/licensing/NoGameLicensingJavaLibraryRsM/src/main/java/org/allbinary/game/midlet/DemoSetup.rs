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
// package org.allbinary.game.midlet
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::GameMidlet::GameMidlet;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DemoSetup {
}


impl DemoSetup {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self, mut gameMidlet: Option<Rc<RefCell<dyn crate::org::allbinary::game::midlet::GameMidlet::GameMidletDyn>>>) {
    }
}
pub trait DemoSetupDyn {
    fn as_DemoSetup(&self) -> &DemoSetup;
    fn as_DemoSetup_mut(&mut self) -> &mut DemoSetup;
    fn init(&mut self, mut gameMidlet: Option<Rc<RefCell<dyn crate::org::allbinary::game::midlet::GameMidlet::GameMidletDyn>>>) { DemoSetup::init(self.as_DemoSetup_mut(), gameMidlet) }
}
impl DemoSetupDyn for DemoSetup {
    fn as_DemoSetup(&self) -> &DemoSetup { self }
    fn as_DemoSetup_mut(&mut self) -> &mut DemoSetup { self }
}


