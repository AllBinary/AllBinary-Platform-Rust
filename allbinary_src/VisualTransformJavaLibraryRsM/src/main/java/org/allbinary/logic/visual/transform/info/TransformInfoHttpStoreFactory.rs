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
// package org.allbinary.logic.visual.transform.info
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use super::TransformInfoFactoryInterface::TransformInfoFactoryInterface;
use super::TransformInfoHttpStore::TransformInfoHttpStore;
use super::TransformInfoInterface::TransformInfoInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoHttpStoreFactory {
}

thread_local! { pub static TransformInfoHttpStoreFactory_instance: Option<Rc<RefCell<TransformInfoHttpStoreFactory>>> = TransformInfoHttpStoreFactory::new(); }
impl TransformInfoFactoryInterface for TransformInfoHttpStoreFactory {
    fn getInstance(&mut self, mut databaseHashMap: Option<Rc<RefCell<HashMap>>>, mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> { Self::getInstance3(self, databaseHashMap, propertiesHashMap, pageContext) }
}

impl TransformInfoHttpStoreFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<TransformInfoHttpStoreFactory>>> {
        return TransformInfoHttpStoreFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance3(&mut self, mut databaseHashMap: Option<Rc<RefCell<HashMap>>>, mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> {
        return TransformInfoHttpStore::new3(databaseHashMap.clone(), propertiesHashMap.clone(), pageContext.clone()).map(|reference| reference as Rc<RefCell<dyn TransformInfoInterface>>);
    }
}

