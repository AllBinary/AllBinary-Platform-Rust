// 
// AllBinary Open License Version 1 
// Copyright (c) 2026 AllBinary 
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
use crate::java::lang::Exception::*;
use super::BasicUserRole::BasicUserRole;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicUserRoles {
    pub roleVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static BasicUserRoles_instance: Option<Rc<RefCell<BasicUserRoles>>> = BasicUserRoles::new(); }

impl BasicUserRoles {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            roleVector: None,
        };
        this.roleVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<BasicUserRoles>>> {
        return BasicUserRoles_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getRole(&mut self, mut role: String) -> Option<Rc<RefCell<BasicUserRole>>> {
        let mut roleVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getAll();
        let mut size: i32 = jcall!(roleVector).size();
        for index in (0..size) {
            let mut userRole: Option<Rc<RefCell<BasicUserRole>>> = { let java_object = (jcall!(roleVector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<BasicUserRole>>>>().expect("ClassCastException").clone()) };
            if (role.cmp(&jcall!(userRole).toString()) as i32) == 0 {
                return userRole.clone();
            }
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Unable to get role - Unknown Role"))));
    }
    

    pub fn getAll(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.roleVector.clone();
    }
}

