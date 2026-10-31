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
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameFeatureChoiceGroups {
    pub hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
}

pub static mut GameFeatureChoiceGroups_gameFeatureChoiceGroupsExclusive: std::cell::LazyCell<Option<Rc<RefCell<GameFeatureChoiceGroups>>>> = std::cell::LazyCell::new(|| GameFeatureChoiceGroups::new());
pub static mut GameFeatureChoiceGroups_gameFeatureChoiceGroupsMultiple: std::cell::LazyCell<Option<Rc<RefCell<GameFeatureChoiceGroups>>>> = std::cell::LazyCell::new(|| GameFeatureChoiceGroups::new());

impl GameFeatureChoiceGroups {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            hashtable: None,
        };
        this.hashtable = jcall!(StdUtil::getInstance()).createHashtable();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getExclusiveInstance() -> Option<Rc<RefCell<GameFeatureChoiceGroups>>> {
        return unsafe { GameFeatureChoiceGroups_gameFeatureChoiceGroupsExclusive.clone() }.clone();
    }
    

    pub fn getMultipleInstance() -> Option<Rc<RefCell<GameFeatureChoiceGroups>>> {
        return unsafe { GameFeatureChoiceGroups_gameFeatureChoiceGroupsMultiple.clone() }.clone();
    }
    

    pub fn get(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return self.hashtable.clone();
    }
    

    pub fn add(&mut self, mut name: String, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        jcall!(self.hashtable).put(crate::java_into_object(name.clone()), crate::java_into_object(list.clone()));
    }
}

