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
use super::BasicUserRoleFactory::BasicUserRoleFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreRoleFactory {
    pub roles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub subscriberRoles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub wholesaleRoles: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static StoreRoleFactory_instance: Option<Rc<RefCell<StoreRoleFactory>>> = StoreRoleFactory::new(); }

impl StoreRoleFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<StoreRoleFactory>>> {
        return StoreRoleFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            roles: None,
            subscriberRoles: None,
            wholesaleRoles: None,
        };
        this.roles = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.subscriberRoles = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.wholesaleRoles = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut basicUserRoleFactory: Option<Rc<RefCell<BasicUserRoleFactory>>> = BasicUserRoleFactory::getInstance();
        jcall!(this.roles).add(crate::java_into_object(jcall!(basicUserRoleFactory).CUSTOMER.clone()));
        jcall!(this.roles).add(crate::java_into_object(jcall!(basicUserRoleFactory).SUBSCRIBERCUSTOMER.clone()));
        jcall!(this.roles).add(crate::java_into_object(jcall!(basicUserRoleFactory).WHOLESALECUSTOMER.clone()));
        jcall!(this.subscriberRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).SUBSCRIBERCUSTOMER.clone()));
        jcall!(this.wholesaleRoles).add(crate::java_into_object(jcall!(basicUserRoleFactory).WHOLESALECUSTOMER.clone()));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getRoles(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.roles.clone();
    }
    

    pub fn getSubscriberRoles(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.subscriberRoles.clone();
    }
    

    pub fn getWholesaleRoles(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.wholesaleRoles.clone();
    }
}

