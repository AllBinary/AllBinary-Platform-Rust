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
// package bundle.input.automation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::util::Collection::Collection;
// use crate::java::util::Map::Map;
// use crate::java::util::Set::Set;
use crate::org::osgi::framework::ServiceReference::ServiceReference;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ServiceReferenceMap {
    pub serviceReference: Option<Rc<RefCell<dyn ServiceReference>>>,
}

// impl Map for ServiceReferenceMap with the methods of the trait

impl ServiceReferenceMap {
    

    pub fn new_struct(mut serviceReference: Option<Rc<RefCell<dyn ServiceReference>>>) -> Self {
        let mut this = Self {
            serviceReference: None,
        };
        this.serviceReference = serviceReference.clone();
        this
    }

    pub fn new(mut serviceReference: Option<Rc<RefCell<dyn ServiceReference>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(serviceReference))))
    }
    

    pub fn size(&mut self) -> i32 {
        return -1;
    }
    

    pub fn isEmpty(&mut self) -> bool {
        return false;
    }
    

    pub fn containsKey(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return false;
    }
    

    pub fn containsValue(&mut self, mut value: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return false;
    }
    

    pub fn get(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return jcall!(self.serviceReference).getProperty((key).clone() /* as String */);
    }
    

    pub fn put(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut value: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return None;
    }
    

    pub fn remove(&mut self, mut key: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return None;
    }
    

    pub fn putAll(&mut self, mut m: Option<Rc<RefCell<dyn Map>>>) {
    }
    

    pub fn clear(&mut self) {
    }
    

    pub fn keySet(&mut self) -> Option<Rc<RefCell<dyn Set>>> {
        let mut keyStringArray: Vec<String> = jcall!(self.serviceReference).getPropertyKeys();
        return None;
    }
    

    pub fn values(&mut self) -> Option<Rc<RefCell<dyn Collection<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return None;
    }
    

    pub fn entrySet(&mut self) -> Option<Rc<RefCell<dyn Set>>> {
        return None;
    }
    

    pub fn toString(mut map: Option<Rc<RefCell<dyn Map>>>) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        return jcall!(stringBuffer).toString();
    }
}

