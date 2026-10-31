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
// package org.allbinary.android.input.motion
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::analog::AnalogLocationInput::AnalogLocationInput;
use crate::org::allbinary::game::input::analog::AnalogLocationInputFactory::AnalogLocationInputFactory;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::AnalogControllerVisitor::AnalogControllerVisitor;
use super::AnalogLocationInputProcessor::AnalogLocationInputProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllAnalogControllerVisitor {
    pub base: AnalogControllerVisitor,
}


impl AllAnalogControllerVisitor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AnalogControllerVisitor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn process(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut analogLocationInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::android::input::motion::AnalogLocationInputProcessor::AnalogLocationInputProcessorDyn>>>) {
        let mut analogLocationInputList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(AnalogLocationInputFactory::getInstance()).getList();
        let mut analogLocationInput: Option<Rc<RefCell<AnalogLocationInput>>> = None;
        for index in (0..=jcall!(analogLocationInputList).size() - 1).rev() {
            analogLocationInput = { let java_object = (jcall!(analogLocationInputList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<AnalogLocationInput>>>>().expect("ClassCastException").clone()) };
            jcall!(analogLocationInputProcessor).process(allbinaryLayerManager.clone(), analogLocationInput.clone());
        }
    }
}
impl std::ops::Deref for AllAnalogControllerVisitor {
    type Target = AnalogControllerVisitor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllAnalogControllerVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::android::input::motion::AnalogControllerVisitor::AnalogControllerVisitorDyn for AllAnalogControllerVisitor {
    fn as_AnalogControllerVisitor(&self) -> &crate::org::allbinary::android::input::motion::AnalogControllerVisitor::AnalogControllerVisitor { &self.base }
    fn as_AnalogControllerVisitor_mut(&mut self) -> &mut crate::org::allbinary::android::input::motion::AnalogControllerVisitor::AnalogControllerVisitor { &mut self.base }
    fn process(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut analogLocationInputProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::android::input::motion::AnalogLocationInputProcessor::AnalogLocationInputProcessorDyn>>>) { Self::process(self, allbinaryLayerManager, analogLocationInputProcessor) }
}


