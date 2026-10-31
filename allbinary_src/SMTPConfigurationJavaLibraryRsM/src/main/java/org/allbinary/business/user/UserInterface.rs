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
use crate::org::allbinary::business::user::modules::configuration::UserConfigurationInterface::UserConfigurationInterface;
use crate::org::allbinary::business::user::role::UserRole::UserRole;
use crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionInterface::WeblisketSessionInterface;
use crate::org::allbinary::logic::control::validate::ValidationInterface::ValidationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait UserInterface: TableMappingInterface + ValidationInterface {
    

    fn setUserName(&mut self, value: String);
    

    fn setPrefixName(&mut self, value: String);
    

    fn setFirstName(&mut self, value: String);
    

    fn setLastName(&mut self, value: String);
    

    fn setMiddleName(&mut self, value: String);
    

    fn setSuffixName(&mut self, value: String);
    

    fn setCompany(&mut self, value: String);
    

    fn setPositionAtCompany(&mut self, value: String);
    

    fn setMainEmail(&mut self, value: String);
    

    fn setSecondaryEmail(&mut self, value: String);
    

    fn setHomePhone(&mut self, value: String);
    

    fn setCellPhone(&mut self, value: String);
    

    fn setWorkPhone(&mut self, value: String);
    

    fn setOtherContact(&mut self, value: String);
    

    fn setElectronicContact(&mut self, value: String);
    

    fn setFax(&mut self, value: String);
    

    fn setRole(&mut self, aRole: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>);
    

    fn setEncryption(&mut self, value: String);
    

    fn setSecret(&mut self, value: String);
    

    fn setPassword(&mut self, value: String);
    

    fn setPermissions(&mut self, value: String);
    

    fn enable(&mut self);
    

    fn setEnable(&mut self, enable: String);
    

    fn getUserName(&mut self) -> String;
    

    fn getPrefixName(&mut self) -> String;
    

    fn getFirstName(&mut self) -> String;
    

    fn getLastName(&mut self) -> String;
    

    fn getMiddleName(&mut self) -> String;
    

    fn getSuffixName(&mut self) -> String;
    

    fn getCompany(&mut self) -> String;
    

    fn getPositionAtCompany(&mut self) -> String;
    

    fn getMainEmail(&mut self) -> String;
    

    fn getSecondaryEmail(&mut self) -> String;
    

    fn getHomePhone(&mut self) -> String;
    

    fn getCellPhone(&mut self) -> String;
    

    fn getWorkPhone(&mut self) -> String;
    

    fn getOtherContact(&mut self) -> String;
    

    fn getElectronicContact(&mut self) -> String;
    

    fn getFax(&mut self) -> String;
    

    fn getRole(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>;
    

    fn getEncryption(&mut self) -> String;
    

    fn getSecret(&mut self) -> String;
    

    fn getPassword(&mut self) -> String;
    

    fn getPermissions(&mut self) -> String;
    

    fn getUserConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn UserConfigurationInterface>>>;
    

    fn setUserConfigurationInterface(&mut self, userConfigurationInterface: Option<Rc<RefCell<dyn UserConfigurationInterface>>>);
    

    fn toPasswordHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>>;
    

    fn validateSession(&mut self, weblisketSession: Option<Rc<RefCell<dyn WeblisketSessionInterface>>>);
    

    fn updateSession(&mut self, weblisketSession: Option<Rc<RefCell<dyn WeblisketSessionInterface>>>);
    

    fn isSessionValid(&mut self) -> Option<bool>;
}


