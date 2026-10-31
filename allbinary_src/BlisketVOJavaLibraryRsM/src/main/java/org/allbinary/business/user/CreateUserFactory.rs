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
// package org.allbinary.business.user
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::user::role::UserRole::UserRole;
use crate::org::allbinary::business::user::role::UserRoleB::UserRoleB;
use crate::org::allbinary::business::user::role::UserRoleData::UserRoleData;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use super::UserInterface::UserInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CreateUserFactory {
}


impl CreateUserFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<dyn UserInterface>>> {
        if hashMap.is_some() {
            let mut roleString: String = (jcall!(hashMap).get(crate::org::allbinary::business::user::role::UserRoleData::UserRoleData_NAME.with(Clone::clone).clone())).clone() /* as String */;
            if !jcall!(StringValidationUtil::getInstance()).isEmpty(roleString.clone()) {
                let mut role: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>> = UserRoleB::getRole(roleString.clone());
                return jcall!(jcall!(role).getUserFactory()).getInstance(hashMap.clone());
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Role Specified"))));
            }
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Null HashMap"))));
        }
    }
}

