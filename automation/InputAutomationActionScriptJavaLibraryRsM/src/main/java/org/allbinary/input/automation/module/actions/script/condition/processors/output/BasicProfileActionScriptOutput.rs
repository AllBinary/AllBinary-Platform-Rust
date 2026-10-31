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
// package org.allbinary.input.automation.module.actions.script.condition.processors.output
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessor;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::ProfileActionScriptOutputInterface::ProfileActionScriptOutputInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicProfileActionScriptOutput {
    pub base: BasicProfileActionScriptProcessor,
}

// impl ProfileActionScriptOutputInterface for BasicProfileActionScriptOutput with the methods of the trait
impl crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface for BasicProfileActionScriptOutput {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl BasicProfileActionScriptOutput {
    

    pub fn new_struct2(mut label: String, mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            base: BasicProfileActionScriptProcessor::new_struct2(label.clone(), node.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new2(mut label: String, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(label, node))))
    }
    

    pub fn new_struct1(mut label: String) -> Self {
        let mut this = Self {
            base: BasicProfileActionScriptProcessor::new_struct1(label.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut label: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(label))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(unsafe { GenericProfileActionScriptOutputData_NAME.clone() }.clone());
        return node.clone();
    }
}
impl std::ops::Deref for BasicProfileActionScriptOutput {
    type Target = BasicProfileActionScriptProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicProfileActionScriptOutput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait BasicProfileActionScriptOutputDyn: crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessorDyn {
    fn as_BasicProfileActionScriptOutput(&self) -> &BasicProfileActionScriptOutput;
    fn as_BasicProfileActionScriptOutput_mut(&mut self) -> &mut BasicProfileActionScriptOutput;
}
impl BasicProfileActionScriptOutputDyn for BasicProfileActionScriptOutput {
    fn as_BasicProfileActionScriptOutput(&self) -> &BasicProfileActionScriptOutput { self }
    fn as_BasicProfileActionScriptOutput_mut(&mut self) -> &mut BasicProfileActionScriptOutput { self }
}

impl crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessorDyn for BasicProfileActionScriptOutput {
    fn as_BasicProfileActionScriptProcessor(&self) -> &crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessor { &self.base }
    fn as_BasicProfileActionScriptProcessor_mut(&mut self) -> &mut crate::org::allbinary::input::automation::actions::script::condition::processors::BasicProfileActionScriptProcessor::BasicProfileActionScriptProcessor { &mut self.base }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItemDyn for BasicProfileActionScriptOutput {
    fn as_ProfileActionScriptItem(&self) -> &crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem { &self.base.base }
    fn as_ProfileActionScriptItem_mut(&mut self) -> &mut crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem { &mut self.base.base }
}


