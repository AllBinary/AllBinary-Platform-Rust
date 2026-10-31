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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Enumeration;
use super::*;
// use crate::java::util::Enumeration::Enumeration;
use crate::org::allbinary::layer::Layer::Layer;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::util::EnumerationUtil::EnumerationUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LevelLayersFinder {
    pub enumerationUtil: Option<Rc<RefCell<EnumerationUtil>>>,
}

thread_local! { pub static LevelLayersFinder_instance: Option<Rc<RefCell<LevelLayersFinder>>> = LevelLayersFinder::new(); }

impl LevelLayersFinder {

    pub fn new_struct() -> Self {
        let mut this = Self {
            enumerationUtil: None,
        };
        this.enumerationUtil = EnumerationUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<LevelLayersFinder>>> {
        return LevelLayersFinder_instance.with(Clone::clone).clone();
    }
    

    pub fn get(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut enumeration: Option<Rc<RefCell<dyn Enumeration>>> = jcall!(hashtable).keys();
        let mut layerHashtableCanBeNull: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = None;
        let mut integerCanBeNull: Option<i32> = None;
        let mut cachedIntegerCanBeNull: Option<i32> = None;
        while jcall!(self.enumerationUtil).hasMoreElements(enumeration.clone()) {
            layerHashtableCanBeNull = { let java_object = (jcall!(hashtable).get(crate::java_into_object((jcall!(self.enumerationUtil).nextElement(enumeration.clone())).clone() /* as Any */))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<ABHashtable>>>>().expect("ClassCastException").clone()) };
            integerCanBeNull = { let java_object = jcall!(layerHashtableCanBeNull).get(crate::java_into_object((crate::org::allbinary::layer::Layer::Layer_ID.with(Clone::clone)).clone() /* as Any */)); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) };
            cachedIntegerCanBeNull = jcall!(smallIntegerSingletonFactory).getAt(((integerCanBeNull).unwrap() as i32));
            if !jcall!(list).contains(crate::java_into_object((cachedIntegerCanBeNull.clone()).expect("NullPointerException"))) {
                jcall!(list).add(crate::java_into_object((cachedIntegerCanBeNull.clone()).expect("NullPointerException")));
            }
        }
        return list.clone();
    }
}

