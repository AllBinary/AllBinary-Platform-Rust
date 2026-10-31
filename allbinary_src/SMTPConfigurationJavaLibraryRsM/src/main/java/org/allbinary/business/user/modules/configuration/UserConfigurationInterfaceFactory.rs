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
// package org.allbinary.business.user.modules.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::role::UserRole::UserRole;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::globals::FREEBLISKET_PATH_GLOBALS::FREEBLISKET_PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::control::crypt::file::CryptFileReader::CryptFileReader;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::UserConfigurationView::UserConfigurationView;
use super::UserConfigurationInterface::UserConfigurationInterface;
use super::UserConfiguration::UserConfiguration;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserConfigurationInterfaceFactory {
}

thread_local! { pub static UserConfigurationInterfaceFactory_configurationName: String = String::from("DefaultUserConfiguration"); }

impl UserConfigurationInterfaceFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance1(mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<dyn UserConfigurationInterface>>> {
        let mut configurationNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(UserConfigurationData_NAME.with(Clone::clone).clone(), jcall!(document).getChildNodes());
        return (jcall!(UserConfigurationView::new1(configurationNode.clone())).getUserConfigurationInterface()).clone() /* as UserConfigurationInterface */;
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance1(mut userRole: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>) -> Option<Rc<RefCell<dyn UserConfigurationInterface>>> {
        let mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(format!("{}{}", URLGLOBALS::getMainPath(), jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).USERCONFIGURATIONPATH), format!("{}{}", format!("{}{}", format!("{}{}", UserConfigurationInterfaceFactory_configurationName.with(Clone::clone), jcall!(userRole).toString()), jcall!(AbPathData::getInstance()).EXTENSION_SEP), UserConfigurationData_UNCRYPTED_EXTENSION.with(Clone::clone))).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        let mut documentString: String = jcall!(CryptFileReader::new(UserConfigurationData_UNCRYPTED_EXTENSION.with(Clone::clone).clone(), UserConfigurationData_ENCRYPTED_EXTENSION.with(Clone::clone).clone())).get(abPath.clone());
        let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(documentString.clone());
        return UserConfigurationInterfaceFactory::getInstance1(document.clone());
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance() -> Option<Rc<RefCell<dyn UserConfigurationInterface>>> {
        return UserConfiguration::new().map(|reference| reference as Rc<RefCell<dyn UserConfigurationInterface>>);
    }
}

