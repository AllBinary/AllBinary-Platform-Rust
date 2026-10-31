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
// package org.allbinary.graphics.j2me.workarea.color
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::JPanel::JPanel;
use super::swing::swing;
use super::javax::javax;
use super::JColorChooser::JColorChooser;
use super::BorderLayout::BorderLayout;
use super::awt::awt;
use super::java::java;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ColorChooser {
    pub base: JPanel,
    pub jColorChooser1: Option<Rc<RefCell<JColorChooser>>>,
}


impl ColorChooser {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: JPanel::new_struct(),
            jColorChooser1: None,
        };
        this.initComponents();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn initComponents(&mut self) {
        self.jColorChooser1 = JColorChooser::new();
        self.setLayout(BorderLayout::new());
        self.add(self.jColorChooser1.clone(), jcall!(jcall!(unsafe { java_awt.clone() }).BorderLayout).CENTER.clone());
    }
}
impl std::ops::Deref for ColorChooser {
    type Target = JPanel;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ColorChooser {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



