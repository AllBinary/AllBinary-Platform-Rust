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
use crate::org::allbinary::thirdparty::store::LongArrayIdentifierInterface::LongArrayIdentifierInterface;
use super::AbeClientInformation::AbeClientInformation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreAppClientInformation {
    pub base: AbeClientInformation,
    pub longArrayIdentifier: Vec<i64>,
}

impl LongArrayIdentifierInterface for StoreAppClientInformation {
    fn getLongArrayIdentifier(&mut self) -> Vec<i64> { Self::getLongArrayIdentifier(self) }
}

impl StoreAppClientInformation {
    

    pub fn new_struct(mut name: String, mut version: String, mut specialName: String, mut shortName: String, mut longArrayIdentifier: Vec<i64>) -> Self {
        let mut this = Self {
            base: AbeClientInformation::new_struct(name.clone(), version.clone(), specialName.clone(), shortName.clone()),
            longArrayIdentifier: Vec::new(),
        };
        // super(...) is in the struct literal above
        this.longArrayIdentifier = longArrayIdentifier.clone();
        this
    }

    pub fn new(mut name: String, mut version: String, mut specialName: String, mut shortName: String, mut longArrayIdentifier: Vec<i64>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, version, specialName, shortName, longArrayIdentifier))))
    }
    

    pub fn getLongArrayIdentifier(&mut self) -> Vec<i64> {
        return self.longArrayIdentifier.clone();
    }
}
impl std::ops::Deref for StoreAppClientInformation {
    type Target = AbeClientInformation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StoreAppClientInformation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn for StoreAppClientInformation {
    fn as_ClientInformation(&self) -> &crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation { &self.base.base }
    fn as_ClientInformation_mut(&mut self) -> &mut crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformation { &mut self.base.base }
    fn init(&mut self) { crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformation::init(crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformationDyn::as_AbeClientInformation_mut(self)) }
}
impl crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformationDyn for StoreAppClientInformation {
    fn as_AbeClientInformation(&self) -> &crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformation { &self.base }
    fn as_AbeClientInformation_mut(&mut self) -> &mut crate::org::allbinary::logic::system::security::licensing::AbeClientInformation::AbeClientInformation { &mut self.base }
}


