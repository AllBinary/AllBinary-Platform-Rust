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
// package org.allbinary.business.user.role
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::Serializable::Serializable;
use crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface;
use super::BasicUserRole::BasicUserRole;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserRole {
    pub basicUserRole: Option<Rc<RefCell<BasicUserRole>>>,
    pub userFactoryInterface: Option<Rc<RefCell<dyn UserFactoryInterface>>>,
}

// impl Serializable for UserRole with the methods of the trait

impl UserRole {
    

    pub fn new_struct(mut aBasicUserRole: Option<Rc<RefCell<BasicUserRole>>>, mut aUserFactoryInterface: Option<Rc<RefCell<dyn UserFactoryInterface>>>) -> Self {
        let mut this = Self {
            basicUserRole: None,
            userFactoryInterface: None,
        };
        this.basicUserRole = aBasicUserRole.clone();
        this.userFactoryInterface = aUserFactoryInterface.clone();
        this
    }

    pub fn new(mut aBasicUserRole: Option<Rc<RefCell<BasicUserRole>>>, mut aUserFactoryInterface: Option<Rc<RefCell<dyn UserFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(aBasicUserRole, aUserFactoryInterface))))
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(self.basicUserRole).toString();
    }
    

    pub fn getBasicUserRole(&mut self) -> Option<Rc<RefCell<BasicUserRole>>> {
        return self.basicUserRole.clone();
    }
    

    // throws Exception
    pub fn getUserFactory(&mut self) -> Option<Rc<RefCell<dyn UserFactoryInterface>>> {
        return self.userFactoryInterface.clone();
    }
    

    pub fn getSessionTimeout(&mut self) -> i64 {
        return jcall!(self.basicUserRole).getSessionTimeout();
    }
    

    pub fn getSessionInactivityTimeout(&mut self) -> i64 {
        return jcall!(self.basicUserRole).getSessionInactivityTimeout();
    }
}
pub trait UserRoleDyn {
    fn as_UserRole(&self) -> &UserRole;
    fn as_UserRole_mut(&mut self) -> &mut UserRole;
    fn toString(&mut self) -> String { UserRole::toString(self.as_UserRole_mut()) }
    fn getBasicUserRole(&mut self) -> Option<Rc<RefCell<BasicUserRole>>> { UserRole::getBasicUserRole(self.as_UserRole_mut()) }
    fn getUserFactory(&mut self) -> Option<Rc<RefCell<dyn UserFactoryInterface>>> { UserRole::getUserFactory(self.as_UserRole_mut()) }
    fn getSessionTimeout(&mut self) -> i64 { UserRole::getSessionTimeout(self.as_UserRole_mut()) }
    fn getSessionInactivityTimeout(&mut self) -> i64 { UserRole::getSessionInactivityTimeout(self.as_UserRole_mut()) }
}
impl UserRoleDyn for UserRole {
    fn as_UserRole(&self) -> &UserRole { self }
    fn as_UserRole_mut(&mut self) -> &mut UserRole { self }
}


