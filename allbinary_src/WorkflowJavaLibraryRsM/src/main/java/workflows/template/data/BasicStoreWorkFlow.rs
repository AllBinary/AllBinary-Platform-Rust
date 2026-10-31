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
// package workflows.template.data
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::logic::control::workflow::StoreWorkFlowInterface::StoreWorkFlowInterface;
use super::BasicWorkFlow::BasicWorkFlow;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicStoreWorkFlow {
    pub base: BasicWorkFlow,
}

// impl StoreWorkFlowInterface for BasicStoreWorkFlow with the methods of the trait

impl BasicStoreWorkFlow {
    

    pub fn new_struct(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: BasicWorkFlow::new_struct(propertiesHashMap.clone(), pageContext.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut propertiesHashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(propertiesHashMap, pageContext))))
    }
    

    // throws Exception
    pub fn getStoreName(&mut self) -> String {
        return (jcall!(self.getPropertiesHashMap()).get(jcall!(StoreFrontData::getInstance()).NAME.clone())).clone() /* as String */;
    }
}
impl std::ops::Deref for BasicStoreWorkFlow {
    type Target = BasicWorkFlow;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicStoreWorkFlow {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::workflows::template::data::BasicWorkFlow::BasicWorkFlowDyn for BasicStoreWorkFlow {
    fn as_BasicWorkFlow(&self) -> &crate::workflows::template::data::BasicWorkFlow::BasicWorkFlow { &self.base }
    fn as_BasicWorkFlow_mut(&mut self) -> &mut crate::workflows::template::data::BasicWorkFlow::BasicWorkFlow { &mut self.base }
}


