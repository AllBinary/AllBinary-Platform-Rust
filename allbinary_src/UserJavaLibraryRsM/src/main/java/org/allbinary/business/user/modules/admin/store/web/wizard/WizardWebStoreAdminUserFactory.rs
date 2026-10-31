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
// package org.allbinary.business.user.modules.admin.store.web.wizard
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface;
use crate::org::allbinary::business::user::UserInterface::UserInterface;
use super::WizardWebStoreAdminUser::WizardWebStoreAdminUser;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WizardWebStoreAdminUserFactory {
}

// impl UserFactoryInterface for WizardWebStoreAdminUserFactory with the methods of the trait

impl WizardWebStoreAdminUserFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(&mut self) -> Option<Rc<RefCell<dyn UserInterface>>> {
        return WizardWebStoreAdminUser::new().map(|reference| reference as Rc<RefCell<dyn UserInterface>>);
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance1(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<dyn UserInterface>>> {
        return WizardWebStoreAdminUser::new1(hashMap.clone()).map(|reference| reference as Rc<RefCell<dyn UserInterface>>);
    }
}

