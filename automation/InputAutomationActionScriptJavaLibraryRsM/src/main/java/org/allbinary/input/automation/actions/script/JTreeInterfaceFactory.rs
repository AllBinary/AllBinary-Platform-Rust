// 
// To change this license header, choose License Headers in Project Properties. 
// To change this template file, choose Tools | Templates 
// and open the template in the editor.  
// Generated Code Do Not Modify
// package org.allbinary.input.automation.actions.script
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::JTreeInterface::JTreeInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct JTreeInterfaceFactory {
    pub jTreeInterface: Option<Rc<RefCell<dyn JTreeInterface>>>,
}

thread_local! { pub static JTreeInterfaceFactory_instance: Option<Rc<RefCell<JTreeInterfaceFactory>>> = JTreeInterfaceFactory::new(); }

impl JTreeInterfaceFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            jTreeInterface: None,
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<JTreeInterfaceFactory>>> {
        return JTreeInterfaceFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn set(&mut self, mut jTreeInterface: Option<Rc<RefCell<dyn JTreeInterface>>>) {
        self.jTreeInterface = jTreeInterface.clone();
    }
    

    pub fn getJTreeInterface(&mut self) -> Option<Rc<RefCell<dyn JTreeInterface>>> {
        return self.jTreeInterface.clone();
    }
}

