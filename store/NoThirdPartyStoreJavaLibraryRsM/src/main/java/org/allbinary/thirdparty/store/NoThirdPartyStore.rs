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
// package org.allbinary.thirdparty.store
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use super::ThirdPartyStore::ThirdPartyStore;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoThirdPartyStore {
    pub base: ThirdPartyStore,
}


impl NoThirdPartyStore {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ThirdPartyStore::new_struct(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn authorize(&mut self, mut clientInformationInterface: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        self.setAuthorized(true);
    }
}
impl std::ops::Deref for NoThirdPartyStore {
    type Target = ThirdPartyStore;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NoThirdPartyStore {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::thirdparty::store::ThirdPartyStore::ThirdPartyStoreDyn for NoThirdPartyStore {
    fn as_ThirdPartyStore(&self) -> &crate::org::allbinary::thirdparty::store::ThirdPartyStore::ThirdPartyStore { &self.base }
    fn as_ThirdPartyStore_mut(&mut self) -> &mut crate::org::allbinary::thirdparty::store::ThirdPartyStore::ThirdPartyStore { &mut self.base }
    fn authorize(&mut self, mut clientInformationInterface: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) { Self::authorize(self, clientInformationInterface) }
}


