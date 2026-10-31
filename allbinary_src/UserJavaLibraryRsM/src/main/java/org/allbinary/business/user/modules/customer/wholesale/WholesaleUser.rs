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
// package org.allbinary.business.user.modules.customer.wholesale
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::user::modules::User::User;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionInterface::WeblisketSessionInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WholesaleUser {
    pub base: User,
}


impl WholesaleUser {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: User::new_struct(),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut userHashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            base: User::new_struct1(userHashMap.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut userHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(userHashMap))))
    }
    

    pub fn validateSession(&mut self, mut weblisketSession: Option<Rc<RefCell<dyn WeblisketSessionInterface>>>) {
        self.base.validateSession(weblisketSession.clone());
        self.updateSession(weblisketSession.clone());
    }
    

    pub fn updateSession(&mut self, mut weblisketSession: Option<Rc<RefCell<dyn WeblisketSessionInterface>>>) {
        jcall!(weblisketSession).setStoreName(self.getPermissions());
    }
}
impl std::ops::Deref for WholesaleUser {
    type Target = User;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for WholesaleUser {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::user::modules::User::UserDyn for WholesaleUser {
    fn as_User(&self) -> &crate::org::allbinary::business::user::modules::User::User { &self.base }
    fn as_User_mut(&mut self) -> &mut crate::org::allbinary::business::user::modules::User::User { &mut self.base }
    fn validateSession(&mut self, mut weblisketSession: Option<Rc<RefCell<dyn WeblisketSessionInterface>>>) { Self::validateSession(self, weblisketSession) }
    fn updateSession(&mut self, mut weblisketSession: Option<Rc<RefCell<dyn WeblisketSessionInterface>>>) { Self::updateSession(self, weblisketSession) }
}


