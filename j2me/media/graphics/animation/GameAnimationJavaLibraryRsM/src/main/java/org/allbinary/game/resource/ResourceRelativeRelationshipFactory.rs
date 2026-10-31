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
// package org.allbinary.game.resource
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationship;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use super::ResourceRelativeRelationshipFactoryInterface::ResourceRelativeRelationshipFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ResourceRelativeRelationshipFactory {
    pub basicArrayListUtil: Option<Rc<RefCell<BasicArrayListUtil>>>,
    pub hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
    pub initialized: bool,
}

impl ResourceRelativeRelationshipFactoryInterface for ResourceRelativeRelationshipFactory {
    fn getResourceRelativeRelationshipList(&mut self, mut resource: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getResourceRelativeRelationshipList(self, resource) }
    fn addResourceRelativeRelationship(&mut self, mut resource: String, mut hardPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationshipDyn>>>) { Self::addResourceRelativeRelationship(self, resource, hardPoint) }
}
impl crate::org::allbinary::game::configuration::feature::GameFeatureControlledInterface::GameFeatureControlledInterface for ResourceRelativeRelationshipFactory {
    fn init(&mut self, mut level: i32) { Self::init(self, level) }
    fn isFeature(&mut self) -> bool { Self::isFeature(self) }
    fn isLoadingLevel(&mut self, mut level: i32) -> bool { Self::isLoadingLevel(self, level) }
}

impl ResourceRelativeRelationshipFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            basicArrayListUtil: None,
            hashtable: None,
            initialized: false,
        };
        this.basicArrayListUtil = BasicArrayListUtil::getInstance();
        this.hashtable = jcall!(StdUtil::getInstance()).createHashtable();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(&mut self, mut level: i32) {
        self.setInitialized(true);
    }
    

    // throws Exception
    pub fn getResourceRelativeRelationshipList(&mut self, mut resource: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut listCanBeNull: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.hashtable).get(crate::java_into_object(resource.clone()));
        if listCanBeNull.is_none() {
            return jcall!(self.basicArrayListUtil).getImmutableInstance();
        }
        return (listCanBeNull).clone() /* as BasicArrayList */;
    }
    

    // throws Exception
    pub fn addResourceRelativeRelationship(&mut self, mut resource: String, mut hardPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationshipDyn>>>) {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getResourceRelativeRelationshipList(resource.clone());
        if match (&list, &jcall!(self.basicArrayListUtil).getImmutableInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        }
        jcall!(list).add(crate::java_into_object(hardPoint.clone()));
        jcall!(self.hashtable).put(crate::java_into_object(resource.clone()), crate::java_into_object(list.clone()));
    }
    

    pub fn isLoadingLevel(&mut self, mut level: i32) -> bool {
        return false;
    }
    

    pub fn isFeature(&mut self) -> bool {
        return false;
    }
    

    pub fn setInitialized(&mut self, mut initialized: bool) {
        self.initialized = initialized.clone();
    }
    

    pub fn isInitialized(&mut self) -> bool {
        return self.initialized.clone();
    }
}

