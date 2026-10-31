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
// package org.allbinary.game.ai
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use super::ArtificialIntelligenceInterfaceFactoryInterface::ArtificialIntelligenceInterfaceFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ArtificialIntelligenceInterfaceFactoryInterfaceFactory {
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

pub static mut ArtificialIntelligenceInterfaceFactoryInterfaceFactory_SINGLETON: std::cell::LazyCell<Option<Rc<RefCell<ArtificialIntelligenceInterfaceFactoryInterfaceFactory>>>> = std::cell::LazyCell::new(|| ArtificialIntelligenceInterfaceFactoryInterfaceFactory::new());

impl ArtificialIntelligenceInterfaceFactoryInterfaceFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            list: None,
        };
        this.list = jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn clear(&mut self) {
        self.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ArtificialIntelligenceInterfaceFactoryInterfaceFactory>>> {
        return unsafe { ArtificialIntelligenceInterfaceFactoryInterfaceFactory_SINGLETON.clone() }.clone();
    }
    

    pub fn getInstanceFromCollection(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) -> Option<Rc<RefCell<dyn ArtificialIntelligenceInterfaceFactoryInterface>>> {
        let mut typeInteger: Option<i32> = { let java_object = jcall!(hashtable).get(crate::java_into_object((BasicAI_ID.with(Clone::clone)).clone() /* as Any */)); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) };
        let mut r#type: i32 = ((typeInteger).unwrap() as i32);
        let mut artificialIntelligenceInterfaceFactoryInterface: Option<Rc<RefCell<dyn ArtificialIntelligenceInterfaceFactoryInterface>>> = { let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(r#type - 1) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ArtificialIntelligenceInterfaceFactoryInterface>>>>().expect("ClassCastException").clone()) };
        return artificialIntelligenceInterfaceFactoryInterface.clone();
    }
    

    pub fn add(&mut self, mut artificialIntelligenceInterfaceFactoryInterface: Option<Rc<RefCell<dyn ArtificialIntelligenceInterfaceFactoryInterface>>>) {
        jcall!(self.list).add(crate::java_into_object(artificialIntelligenceInterfaceFactoryInterface.clone()));
    }
}

