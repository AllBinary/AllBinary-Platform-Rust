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
// package org.allbinary.logic.io
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::DataOutputStream::DataOutputStream;
// use crate::java::io::OutputStream::OutputStream;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbDataOutputStream {
    pub base: DataOutputStream,
}


impl AbDataOutputStream {
    

    pub fn new_struct(mut out: Option<Rc<RefCell<OutputStream>>>) -> Self {
        let mut this = Self {
            base: DataOutputStream::new_struct1(out.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut out: Option<Rc<RefCell<OutputStream>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(out))))
    }
}
impl std::ops::Deref for AbDataOutputStream {
    type Target = DataOutputStream;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AbDataOutputStream {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



