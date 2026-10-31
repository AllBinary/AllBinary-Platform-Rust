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
// package org.allbinary.logic.system.security.licensing
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::AbeLicenseInterface::AbeLicenseInterface;
use super::LicenseType::LicenseType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbeNoLicense {
    pub serverVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

pub static mut AbeNoLicense_abeNoLicense: std::cell::LazyCell<Option<Rc<RefCell<AbeNoLicense>>>> = std::cell::LazyCell::new(|| AbeNoLicense::new());
// impl AbeLicenseInterface for AbeNoLicense with the methods of the trait

impl AbeNoLicense {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            serverVector: None,
        };
        this.serverVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AbeNoLicense>>> {
        return unsafe { AbeNoLicense_abeNoLicense.clone() }.clone();
    }
    

    pub fn hasKey(&mut self) -> bool {
        return false;
    }
    

    pub fn getKey(&mut self, mut keyName: String) -> String {
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
    

    pub fn getLicenseId(&mut self) -> String {
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
    

    pub fn getSpecial(&mut self) -> String {
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
    

    pub fn getServers(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.serverVector.clone();
    }
    

    pub fn isValid(&mut self) -> bool {
        return false;
    }
    

    pub fn getLicenseType(&mut self) -> Option<Rc<RefCell<LicenseType>>> {
        return jcall!(LicenseTypeFactory::LicenseTypeFactory::getInstance()).UNKNOWN.clone();
    }
}

