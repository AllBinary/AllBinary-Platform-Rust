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
// package org.allbinary.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::AllBinaryLayer::AllBinaryLayer;
use super::AllBinaryLayerManager::AllBinaryLayerManager;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicLayerProcessor {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static BasicLayerProcessor_NULL_LAYER_PROCESSOR: Option<Rc<RefCell<dyn BasicLayerProcessorDyn>>> = BasicLayerProcessor::new().map(|reference| reference as Rc<RefCell<dyn BasicLayerProcessorDyn>>); }

impl BasicLayerProcessor {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            list: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn add(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        if !jcall!(self.list).contains(crate::java_into_object(layerInterface.clone())) {
            jcall!(self.list).add(crate::java_into_object(layerInterface.clone()));
        }
    }
    

    // throws Exception
    pub fn process(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(CommonStrings::getInstance()).NOT_IMPLEMENTED.clone())));
    }
    

    pub fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.list.clone();
    }
}
pub trait BasicLayerProcessorDyn {
    fn as_BasicLayerProcessor(&self) -> &BasicLayerProcessor;
    fn as_BasicLayerProcessor_mut(&mut self) -> &mut BasicLayerProcessor;
    fn add(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { BasicLayerProcessor::add(self.as_BasicLayerProcessor_mut(), layerInterface) }
    fn process(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { BasicLayerProcessor::process(self.as_BasicLayerProcessor_mut(), allBinaryLayerManager) }
    fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { BasicLayerProcessor::getList(self.as_BasicLayerProcessor_mut()) }
}
impl BasicLayerProcessorDyn for BasicLayerProcessor {
    fn as_BasicLayerProcessor(&self) -> &BasicLayerProcessor { self }
    fn as_BasicLayerProcessor_mut(&mut self) -> &mut BasicLayerProcessor { self }
}


