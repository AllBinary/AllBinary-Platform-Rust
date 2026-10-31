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
// package org.allbinary.logic.system.security.licensing
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::AbeClientInformation::AbeClientInformation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationClientInformation {
    pub base: AbeClientInformation,
}


impl InputAutomationClientInformation {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AbeClientInformation::new_struct(String::from("InputAutomationTool"), String::from("0.0.1"), String::from("InputAutomationTool 0.0.1"), String::from("IA")),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for InputAutomationClientInformation {
    type Target = AbeClientInformation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InputAutomationClientInformation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn for InputAutomationClientInformation {
    fn as_ClientInformation(&self) -> &crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation { &self.base.base }
    fn as_ClientInformation_mut(&mut self) -> &mut crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation { &mut self.base.base }
    fn init(&mut self) { crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformation::init(crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformationDyn::as_AbeClientInformation_mut(self)) }
}
impl crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformationDyn for InputAutomationClientInformation {
    fn as_AbeClientInformation(&self) -> &crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformation { &self.base }
    fn as_AbeClientInformation_mut(&mut self) -> &mut crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformation { &mut self.base }
}


