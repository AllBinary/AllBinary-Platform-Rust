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
// package org.allbinary.logic.control.contraints.display.browser
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::w3c::dom::Node::Node;
use super::DisplayInBrowserContraintsInterface::DisplayInBrowserContraintsInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DisplayInBrowserContraints {
    pub frameable: bool,
    pub mountable: bool,
    pub quickFrameable: bool,
    pub mountedLink: String,
}

impl DisplayInBrowserContraintsInterface for DisplayInBrowserContraints {
    fn isMountable(&mut self) -> bool { Self::isMountable(self) }
    fn isFrameable(&mut self) -> bool { Self::isFrameable(self) }
    fn isQuickFrameable(&mut self) -> bool { Self::isQuickFrameable(self) }
    fn setMountable(&mut self, mut mountable: bool) { Self::setMountable(self, mountable) }
    fn setFrameable(&mut self, mut frameable: bool) { Self::setFrameable(self, frameable) }
    fn setQuickFrameable(&mut self, mut quickFrameable: bool) { Self::setQuickFrameable(self, quickFrameable) }
    fn setMountedLink(&mut self, mut mountedLink: String) { Self::setMountedLink(self, mountedLink) }
    fn getMountedLink(&mut self) -> String { Self::getMountedLink(self) }
}

impl DisplayInBrowserContraints {
    

    pub fn new_struct(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            frameable: false,
            mountable: false,
            quickFrameable: false,
            mountedLink: String::new(),
        };
        this
    }

    pub fn new(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(node))))
    }
    

    pub fn isMountable(&mut self) -> bool {
        return self.mountable.clone();
    }
    

    pub fn setMountable(&mut self, mut mountable: bool) {
        self.mountable = mountable.clone();
    }
    

    pub fn isFrameable(&mut self) -> bool {
        return self.frameable.clone();
    }
    

    pub fn setFrameable(&mut self, mut frameable: bool) {
        self.frameable = frameable.clone();
    }
    

    pub fn isQuickFrameable(&mut self) -> bool {
        return self.quickFrameable.clone();
    }
    

    pub fn setQuickFrameable(&mut self, mut quickFrameable: bool) {
        self.quickFrameable = quickFrameable.clone();
    }
    

    pub fn getMountedLink(&mut self) -> String {
        return self.mountedLink.clone();
    }
    

    pub fn setMountedLink(&mut self, mut mountedLink: String) {
        self.mountedLink = mountedLink.clone();
    }
}

