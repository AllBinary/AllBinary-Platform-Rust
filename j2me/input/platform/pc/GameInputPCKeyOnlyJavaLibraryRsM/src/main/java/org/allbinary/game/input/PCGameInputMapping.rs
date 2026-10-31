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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::persistance::GamePersistanceStrings::GamePersistanceStrings;
use crate::org::allbinary::game::input::mapping::PersistentInputMapping::PersistentInputMapping;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use super::Input::Input;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PCGameInputMapping {
    pub base: PersistentInputMapping,
}


impl PCGameInputMapping {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: PersistentInputMapping::new_struct1(jcall!(GamePersistanceStrings::getInstance()).SAVED_INPUT_CONFIGURATION_RECORD_ID.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), None /* Java this: a &mut self method has no Rc of itself */, jcall!(self.commonStrings).INIT.clone());
        PCKeyFactory::PCKeyFactory::getInstance();
        self.base.init(abeClientInformation.clone());
        if self.getTotalMapped() == 0 || self.isDefaultNew() {
            jcall!(self.getInputMapping()).addMapping(self.getDefault());
            self.save(abeClientInformation.clone());
        }
    }
    

    pub fn isDelete(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool {
        if match (&input, &jcall!(PCKeyFactory::PCKeyFactory::getInstance()).DEL) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn isSystemInput(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool {
        return false;
    }
}
impl std::ops::Deref for PCGameInputMapping {
    type Target = PersistentInputMapping;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PCGameInputMapping {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait PCGameInputMappingDyn {
    fn as_PCGameInputMapping(&self) -> &PCGameInputMapping;
    fn as_PCGameInputMapping_mut(&mut self) -> &mut PCGameInputMapping;
    fn init(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) { PCGameInputMapping::init(self.as_PCGameInputMapping_mut(), abeClientInformation) }
    fn isDelete(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool { PCGameInputMapping::isDelete(self.as_PCGameInputMapping_mut(), input) }
    fn isSystemInput(&mut self, mut input: Option<Rc<RefCell<Input>>>) -> bool { PCGameInputMapping::isSystemInput(self.as_PCGameInputMapping_mut(), input) }
}
impl PCGameInputMappingDyn for PCGameInputMapping {
    fn as_PCGameInputMapping(&self) -> &PCGameInputMapping { self }
    fn as_PCGameInputMapping_mut(&mut self) -> &mut PCGameInputMapping { self }
}



