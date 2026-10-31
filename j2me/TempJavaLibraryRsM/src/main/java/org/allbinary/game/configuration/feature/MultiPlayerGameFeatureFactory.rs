// Generated Code Do Not Modify
// package org.allbinary.game.configuration.feature
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MultiPlayerGameFeatureFactory {
}

thread_local! { pub static MultiPlayerGameFeatureFactory_instance: Option<Rc<RefCell<MultiPlayerGameFeatureFactory>>> = MultiPlayerGameFeatureFactory::new(); }

impl MultiPlayerGameFeatureFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<MultiPlayerGameFeatureFactory>>> {
        return MultiPlayerGameFeatureFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn addCommands(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
}

