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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use super::TestInputInterface::TestInputInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TestInputExample {
    pub testValue: i32,
}

impl TestInputInterface for TestInputExample {
    fn up(&mut self) { Self::up(self) }
    fn down(&mut self) { Self::down(self) }
    fn left(&mut self) { Self::left(self) }
    fn right(&mut self) { Self::right(self) }
    fn strafeLeft(&mut self) { Self::strafeLeft(self) }
    fn strafeRight(&mut self) { Self::strafeRight(self) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for TestInputExample {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paintThreed(self, graphics) }
}

impl TestInputExample {

    pub fn new_struct() -> Self {
        let mut this = Self {
            testValue: 0,
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn up(&mut self) {
    }
    

    pub fn down(&mut self) {
    }
    

    pub fn left(&mut self) {
        self.testValue += 1;
    }
    

    pub fn right(&mut self) {
        self.testValue -= 1;
    }
    

    pub fn strafeLeft(&mut self) {
    }
    

    pub fn strafeRight(&mut self) {
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
}

