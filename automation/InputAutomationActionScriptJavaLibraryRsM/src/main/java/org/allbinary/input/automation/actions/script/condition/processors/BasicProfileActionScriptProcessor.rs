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
// package org.allbinary.input.automation.actions.script.condition.processors
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::event::ActionEvent::ActionEvent;
use crate::org::allbinary::input::automation::actions::script::JTreeInterfaceFactory::JTreeInterfaceFactory;
use crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem;
use crate::org::allbinary::input::automation::actions::script::condition::ProfileActionScriptConditionInterface::ProfileActionScriptConditionInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::ProfileActionScriptProcessorInterface::ProfileActionScriptProcessorInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicProfileActionScriptProcessor {
    pub base: ProfileActionScriptItem,
}

// impl ProfileActionScriptProcessorInterface for BasicProfileActionScriptProcessor with the methods of the trait

impl BasicProfileActionScriptProcessor {
    

    pub fn new_struct2(mut label: String, mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            base: ProfileActionScriptItem::new_struct2(label.clone(), node.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new2(mut label: String, mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(label, node))))
    }
    

    pub fn new_struct1(mut label: String) -> Self {
        let mut this = Self {
            base: ProfileActionScriptItem::new_struct1(label.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut label: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(label))))
    }
    

    pub fn actionPerformed(&mut self, mut actionEvent: Option<Rc<RefCell<ActionEvent>>>) {
        self.base.actionPerformed(actionEvent.clone());
        if jcall!(jcall!(actionEvent).getActionCommand()).compareTo(DELETE.clone()) == 0 {
            if (self.getParent()).is_some() /* instanceof ProfileActionScriptConditionInterface */ {
                let mut profileActionScriptConditionInterface: Option<Rc<RefCell<dyn ProfileActionScriptConditionInterface>>> = (self.getParent()).clone() /* as ProfileActionScriptConditionInterface */;
                jcall!(profileActionScriptConditionInterface).removeProcessor((self).clone() /* as ProfileActionScriptProcessorInterface */);
                jcall!(jcall!(JTreeInterfaceFactory::getInstance()).getJTreeInterface()).updateJTree();
            }
        }
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(unsafe { GenericProfileActionScriptProcessorData_NAME.clone() }.clone());
        return node.clone();
    }
}
impl std::ops::Deref for BasicProfileActionScriptProcessor {
    type Target = ProfileActionScriptItem;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicProfileActionScriptProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait BasicProfileActionScriptProcessorDyn: crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItemDyn {
    fn as_BasicProfileActionScriptProcessor(&self) -> &BasicProfileActionScriptProcessor;
    fn as_BasicProfileActionScriptProcessor_mut(&mut self) -> &mut BasicProfileActionScriptProcessor;
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { BasicProfileActionScriptProcessor::toXmlNode(self.as_BasicProfileActionScriptProcessor_mut(), document) }
}
impl BasicProfileActionScriptProcessorDyn for BasicProfileActionScriptProcessor {
    fn as_BasicProfileActionScriptProcessor(&self) -> &BasicProfileActionScriptProcessor { self }
    fn as_BasicProfileActionScriptProcessor_mut(&mut self) -> &mut BasicProfileActionScriptProcessor { self }
}

impl crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItemDyn for BasicProfileActionScriptProcessor {
    fn as_ProfileActionScriptItem(&self) -> &crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem { &self.base }
    fn as_ProfileActionScriptItem_mut(&mut self) -> &mut crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem { &mut self.base }
    fn actionPerformed(&mut self, mut actionEvent: Option<Rc<RefCell<ActionEvent>>>) { Self::actionPerformed(self, actionEvent) }
}


