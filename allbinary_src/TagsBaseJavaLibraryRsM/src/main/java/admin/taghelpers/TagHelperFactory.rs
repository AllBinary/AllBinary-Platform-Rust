// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package admin.taghelpers
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::TagHelperFactoryInterface::TagHelperFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TagHelperFactory {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

impl TagHelperFactoryInterface for TagHelperFactory {
    fn getInstance(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getInstance(self, hashMap, pageContext) }
}

impl TagHelperFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonStrings: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception, LicensingException
    pub fn getInstance(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait TagHelperFactoryDyn {
    fn as_TagHelperFactory(&self) -> &TagHelperFactory;
    fn as_TagHelperFactory_mut(&mut self) -> &mut TagHelperFactory;
    fn getInstance(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { TagHelperFactory::getInstance(self.as_TagHelperFactory_mut(), hashMap, pageContext) }
}
impl TagHelperFactoryDyn for TagHelperFactory {
    fn as_TagHelperFactory(&self) -> &TagHelperFactory { self }
    fn as_TagHelperFactory_mut(&mut self) -> &mut TagHelperFactory { self }
}


