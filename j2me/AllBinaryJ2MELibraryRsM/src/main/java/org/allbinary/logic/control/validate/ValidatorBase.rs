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
// package org.allbinary.logic.control.validate
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::ValidatorBaseInterface::ValidatorBaseInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ValidatorBase {
}

thread_local! { pub static ValidatorBase_instance: Option<Rc<RefCell<dyn ValidatorBaseDyn>>> = ValidatorBase::new().map(|reference| reference as Rc<RefCell<dyn ValidatorBaseDyn>>); }
impl ValidatorBaseInterface for ValidatorBase {
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
    fn toHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { Self::toHashtable(self) }
    fn toList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toList(self) }
}

impl ValidatorBase {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn ValidatorBaseDyn>>> {
        return ValidatorBase_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        return jcall!(BooleanFactory::getInstance()).TRUE.clone();
    }
    

    pub fn validationInfo(&mut self) -> String {
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
    

    pub fn toHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return jcall!(StdUtil::getInstance()).NULL_TABLE.clone();
    }
    

    pub fn toList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance();
    }
}
pub trait ValidatorBaseDyn {
    fn as_ValidatorBase(&self) -> &ValidatorBase;
    fn as_ValidatorBase_mut(&mut self) -> &mut ValidatorBase;
    fn isValid(&mut self) -> Option<bool> { ValidatorBase::isValid(self.as_ValidatorBase_mut()) }
    fn validationInfo(&mut self) -> String { ValidatorBase::validationInfo(self.as_ValidatorBase_mut()) }
    fn toHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { ValidatorBase::toHashtable(self.as_ValidatorBase_mut()) }
    fn toList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { ValidatorBase::toList(self.as_ValidatorBase_mut()) }
}
impl ValidatorBaseDyn for ValidatorBase {
    fn as_ValidatorBase(&self) -> &ValidatorBase { self }
    fn as_ValidatorBase_mut(&mut self) -> &mut ValidatorBase { self }
}


