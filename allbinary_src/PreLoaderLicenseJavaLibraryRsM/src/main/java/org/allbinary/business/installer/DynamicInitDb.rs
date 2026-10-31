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
// package org.allbinary.business.installer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::init::db::DatabaseConnectionInfoInterface::DatabaseConnectionInfoInterface;
use crate::org::allbinary::business::init::db::InitDbHelperFactory::InitDbHelperFactory;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use super::InitDb::InitDb;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//Warning you must have sql root access
pub struct DynamicInitDb {
    pub base: InitDb,
}


impl DynamicInitDb {
    

    pub fn new_struct1(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) -> Self {
        let mut this = Self {
            base: InitDb::new_struct(),
        };
        { let java_arguments = (InitDbHelperFactory::getInstance(abeClientInformation.clone()),); this.setHelper(java_arguments.0) };
        this
    }

    pub fn new1(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(abeClientInformation))))
    }
    

    pub fn new_struct2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn DatabaseConnectionInfoInterface>>>) -> Self {
        let mut this = Self {
            base: InitDb::new_struct(),
        };
        { let java_arguments = (InitDbHelperFactory::getInstance2(abeClientInformation.clone(), databaseConnectionInfoInterface.clone()),); this.setHelper(java_arguments.0) };
        this
    }

    pub fn new2(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut databaseConnectionInfoInterface: Option<Rc<RefCell<dyn DatabaseConnectionInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(abeClientInformation, databaseConnectionInfoInterface))))
    }
}
impl std::ops::Deref for DynamicInitDb {
    type Target = InitDb;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DynamicInitDb {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::installer::InitDb::InitDbDyn for DynamicInitDb {
    fn as_InitDb(&self) -> &crate::org::allbinary::business::installer::InitDb::InitDb { &self.base }
    fn as_InitDb_mut(&mut self) -> &mut crate::org::allbinary::business::installer::InitDb::InitDb { &mut self.base }
}


