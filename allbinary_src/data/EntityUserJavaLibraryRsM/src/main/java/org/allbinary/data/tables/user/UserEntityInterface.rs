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
// package org.allbinary.data.tables.user
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::UserInterface::UserInterface;
use crate::org::allbinary::business::user::role::UserRole::UserRole;
use crate::org::allbinary::data::tables::BasicDataTableInterface::BasicDataTableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait UserEntityInterface: BasicDataTableInterface {
    

    // throws Exception
    fn getAdministrators(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    // throws Exception
    fn getStoreManagers(&mut self, storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    // throws Exception
    fn getCustomers(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    // throws Exception
    fn getUsersWithRole(&mut self, userRole: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    // throws Exception
    fn getUsers(&mut self, storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    // throws Exception
    fn getUser(&mut self, userName: String) -> Option<Rc<RefCell<dyn UserInterface>>>;
    

    fn login(&mut self, userName: String, password: String) -> String;
    

    fn deleteWhere(&mut self, key: String, value: String);
    

    fn insert(&mut self, values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>);
    

    fn update(&mut self, userName: String, updatedValues: Option<Rc<RefCell<HashMap>>>);
}


