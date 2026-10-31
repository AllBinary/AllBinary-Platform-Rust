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
// package org.allbinary.graphics.j2me.workarea.canvas.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::swing::tree::MutableTreeNode::MutableTreeNode;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MyCanvasEventSource {
    pub command: String,
    pub treeNode: Option<Rc<RefCell<MutableTreeNode>>>,
    pub angle: f64,
}


impl MyCanvasEventSource {
    

    pub fn new_struct2(mut command: String, mut treeNode: Option<Rc<RefCell<MutableTreeNode>>>) -> Self {
        let mut this = Self {
            command: String::new(),
            treeNode: None,
            angle: 0.0,
        };
        this.angle = 0.0;
        this.command = command.clone();
        this.treeNode = treeNode.clone();
        this
    }

    pub fn new2(mut command: String, mut treeNode: Option<Rc<RefCell<MutableTreeNode>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(command, treeNode))))
    }
    

    pub fn new_struct3(mut command: String, mut angle: f64, mut treeNode: Option<Rc<RefCell<MutableTreeNode>>>) -> Self {
        let mut this = Self {
            command: String::new(),
            treeNode: None,
            angle: 0.0,
        };
        this.angle = 0.0;
        this.command = command.clone();
        this.treeNode = treeNode.clone();
        this.setAngle(angle.clone());
        this
    }

    pub fn new3(mut command: String, mut angle: f64, mut treeNode: Option<Rc<RefCell<MutableTreeNode>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(command, angle, treeNode))))
    }
    

    pub fn getCommand(&mut self) -> String {
        return self.command.clone();
    }
    

    pub fn getTreeNode(&mut self) -> Option<Rc<RefCell<MutableTreeNode>>> {
        return self.treeNode.clone();
    }
    

    pub fn getAngle(&mut self) -> f64 {
        return self.angle.clone();
    }
    

    pub fn setAngle(&mut self, mut angle: f64) {
        self.angle = angle.clone();
    }
}

