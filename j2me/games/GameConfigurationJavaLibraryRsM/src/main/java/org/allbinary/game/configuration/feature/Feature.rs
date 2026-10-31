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
// package org.allbinary.game.configuration.feature
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::event::ChangedGameFeatureListener::ChangedGameFeatureListener;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Feature {
    pub name: String,
}

pub static mut Feature_hashtable: std::cell::LazyCell<Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>> = std::cell::LazyCell::new(|| jcall!(StdUtil::getInstance()).createHashtable());

impl Feature {
    

    pub fn new_struct(mut name: String) -> Self {
        let mut this = Self {
            name: String::new(),
        };
        this.name = name.clone();
        this
    }

    pub fn new(mut name: String) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(name))));
            Feature::add(name.clone(), (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
            jcall!(ChangedGameFeatureListener::getInstance()).add((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn getInstance(mut name: String) -> Option<Rc<RefCell<dyn FeatureDyn>>> {
        return { let java_object = (jcall!(unsafe { Feature_hashtable.clone() }).get(crate::java_into_object(name.clone()))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn FeatureDyn>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn add(mut name: String, mut gameFeature: Option<Rc<RefCell<dyn FeatureDyn>>>) {
        jcall!(unsafe { Feature_hashtable.clone() }).put(crate::java_into_object(name.clone()), crate::java_into_object(gameFeature.clone()));
    }
    

    pub fn toString(&mut self) -> String {
        return self.getName();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
}
pub trait FeatureDyn {
    fn as_Feature(&self) -> &Feature;
    fn as_Feature_mut(&mut self) -> &mut Feature;
    fn toString(&mut self) -> String { Feature::toString(self.as_Feature_mut()) }
    fn getName(&mut self) -> String { Feature::getName(self.as_Feature_mut()) }
}
impl FeatureDyn for Feature {
    fn as_Feature(&self) -> &Feature { self }
    fn as_Feature_mut(&mut self) -> &mut Feature { self }
}


