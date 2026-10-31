// 
// AllBinary Open License Version 1 
// Copyright (c) 2026 AllBinary 
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
// package org.allbinary.logic
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Item::Item;
use crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//Handle the differences in the public platform vs the raw JS build
pub struct MEUtil {
}

thread_local! { pub static MEUtil_instance: Option<Rc<RefCell<MEUtil>>> = MEUtil::new(); }

impl MEUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<MEUtil>>> {
        return MEUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn appendItem(&mut self, mut form: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>>, mut item: Option<Rc<RefCell<Item>>>) {
        jcall!(form).as_CommandForm_mut().append(item.clone());
    }
}

