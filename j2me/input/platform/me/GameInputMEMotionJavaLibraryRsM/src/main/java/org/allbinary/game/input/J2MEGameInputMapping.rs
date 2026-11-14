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
use crate::java::lang::Exception::*;
use super::Input::Input;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct J2MEGameInputMapping {
    pub base: PersistentInputMapping,
}


impl J2MEGameInputMapping {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: PersistentInputMapping::new_struct({ let java_field = jcall!(GamePersistanceStrings::getInstance()).SAVED_INPUT_CONFIGURATION_RECORD_ID.clone(); java_field }.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone());
        jcall!(J2MEKeyFactory::J2MEKeyFactory::getInstance()).init();
        { let java_arguments = (abeClientInformation.clone(),); self.base.init(java_arguments.0) };
        if self.getTotalMapped() == 0 || self.isDefaultNew() {
            jcall!(self.getInputMapping()).addMapping(self.getDefault());
            self.save(abeClientInformation.clone());
        }
    }
    

    pub fn isDelete(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        if match (&input, &{ let java_field = jcall!(GameKeyFactory::GameKeyFactory::getInstance()).GAME_D.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn isSystemInput(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        return false;
    }
}
impl std::ops::Deref for J2MEGameInputMapping {
    type Target = PersistentInputMapping;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for J2MEGameInputMapping {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait J2MEGameInputMappingDyn: crate::org::allbinary::game::input::mapping::PersistentInputMapping::PersistentInputMappingDyn {
    fn as_J2MEGameInputMapping(&self) -> &J2MEGameInputMapping;
    fn as_J2MEGameInputMapping_mut(&mut self) -> &mut J2MEGameInputMapping;
}
impl J2MEGameInputMappingDyn for J2MEGameInputMapping {
    fn as_J2MEGameInputMapping(&self) -> &J2MEGameInputMapping { self }
    fn as_J2MEGameInputMapping_mut(&mut self) -> &mut J2MEGameInputMapping { self }
}

impl crate::org::allbinary::game::input::mapping::PersistentInputMapping::PersistentInputMappingDyn for J2MEGameInputMapping {
    fn as_PersistentInputMapping(&self) -> &crate::org::allbinary::game::input::mapping::PersistentInputMapping::PersistentInputMapping { &self.base }
    fn as_PersistentInputMapping_mut(&mut self) -> &mut crate::org::allbinary::game::input::mapping::PersistentInputMapping::PersistentInputMapping { &mut self.base }
    fn init(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) { Self::init(self, abeClientInformation) }
    fn isDelete(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool { Self::isDelete(self, input) }
    fn isSystemInput(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool { Self::isSystemInput(self, input) }
}


