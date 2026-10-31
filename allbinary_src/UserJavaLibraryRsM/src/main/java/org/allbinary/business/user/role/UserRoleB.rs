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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use super::UserRole::UserRole;
use super::BasicUserRole::BasicUserRole;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserRoleB {
    pub base: UserRole,
}

pub static mut UserRoleB_roleVector: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>> = std::cell::LazyCell::new(|| BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));

impl UserRoleB {
    

    pub fn new_struct(mut aBasicUserRole: Option<Rc<RefCell<BasicUserRole>>>, mut aUserFactoryInterface: Option<Rc<RefCell<dyn UserFactoryInterface>>>) -> Self {
        let mut this = Self {
            base: UserRole::new_struct(aBasicUserRole.clone(), aUserFactoryInterface.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut aBasicUserRole: Option<Rc<RefCell<BasicUserRole>>>, mut aUserFactoryInterface: Option<Rc<RefCell<dyn UserFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(aBasicUserRole, aUserFactoryInterface))));
            jcall!(unsafe { UserRoleB_roleVector.clone() }).add((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    // throws Exception
    // synchronized
    pub fn getRole(mut role: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>> {
        UserRoleFactory::UserRoleFactory::getInstance();
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(role.clone()) {
            let mut roleVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = UserRoleB::getAll();
            let mut size: i32 = jcall!(roleVector).size();
            for index in (0..size) {
                let mut userRole: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>> = { let java_object = (jcall!(roleVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>>().expect("ClassCastException").clone()) };
                if (role.cmp(&jcall!(userRole).toString()) as i32) == 0 {
                    return userRole.clone();
                }
            }
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Unable to get role - Unknown Role = "), role))));
    }
    

    pub fn getAll() -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        UserRoleFactory::UserRoleFactory::getInstance();
        return unsafe { UserRoleB_roleVector.clone() }.clone();
    }
}
impl std::ops::Deref for UserRoleB {
    type Target = UserRole;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UserRoleB {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::user::role::UserRole::UserRoleDyn for UserRoleB {
    fn as_UserRole(&self) -> &crate::org::allbinary::business::user::role::UserRole::UserRole { &self.base }
    fn as_UserRole_mut(&mut self) -> &mut crate::org::allbinary::business::user::role::UserRole::UserRole { &mut self.base }
}


