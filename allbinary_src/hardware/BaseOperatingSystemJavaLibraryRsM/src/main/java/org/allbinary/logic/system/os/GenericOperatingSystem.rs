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
// package org.allbinary.logic.system.os
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::java::character::CharacterSet::CharacterSet;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::OperatingSystemInterface::OperatingSystemInterface;
use super::SystemProperties::SystemProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GenericOperatingSystem {
    pub osName: String,
    pub osArch: String,
    pub osVersion: String,
    pub characterSet: Option<Rc<RefCell<CharacterSet>>>,
    pub scalable: bool,
}

impl OperatingSystemInterface for GenericOperatingSystem {
    fn getName(&mut self) -> String { Self::getName(self) }
    fn getVersion(&mut self) -> String { Self::getVersion(self) }
    fn getArch(&mut self) -> String { Self::getArch(self) }
    fn getCharacterSet(&mut self) -> Option<Rc<RefCell<CharacterSet>>> { Self::getCharacterSet(self) }
    fn isOverScan(&mut self) -> bool { Self::isOverScan(self) }
    fn getOverScanXPercent(&mut self) -> i32 { Self::getOverScanXPercent(self) }
    fn getOverScanYPercent(&mut self) -> i32 { Self::getOverScanYPercent(self) }
    fn isScalable(&mut self) -> bool { Self::isScalable(self) }
    fn isAutoHide(&mut self) -> bool { Self::isAutoHide(self) }
    fn toString(&mut self) -> String { Self::toString(self) }
}

impl GenericOperatingSystem {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            osName: String::new(),
            osArch: String::new(),
            osVersion: String::new(),
            characterSet: None,
            scalable: false,
        };
        this.characterSet = CharacterSet::new();
        this.scalable = false;
        let mut systemProperties: Option<Rc<RefCell<SystemProperties>>> = SystemProperties::getInstance();
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        let mut EMPTY_STRING: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        if jcall!(stringValidationUtil).isEmpty(jcall!(systemProperties).getName()) {
            this.osName = EMPTY_STRING.clone();
        } else {
            this.osName = jcall!(systemProperties).getName();
        }
        if jcall!(stringValidationUtil).isEmpty(jcall!(systemProperties).getArch()) {
            this.osArch = EMPTY_STRING.clone();
        } else {
            this.osArch = jcall!(systemProperties).getArch();
        }
        if jcall!(stringValidationUtil).isEmpty(jcall!(systemProperties).getVersion()) {
            this.osVersion = EMPTY_STRING.clone();
        } else {
            this.osVersion = jcall!(systemProperties).getVersion();
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getName(&mut self) -> String {
        return self.osName.clone();
    }
    

    pub fn getVersion(&mut self) -> String {
        return self.osVersion.clone();
    }
    

    pub fn getArch(&mut self) -> String {
        return self.osArch.clone();
    }
    

    pub fn getCharacterSet(&mut self) -> Option<Rc<RefCell<CharacterSet>>> {
        return self.characterSet.clone();
    }
    

    pub fn isOverScan(&mut self) -> bool {
        return false;
    }
    

    pub fn getOverScanXPercent(&mut self) -> i32 {
        return 100;
    }
    

    pub fn getOverScanYPercent(&mut self) -> i32 {
        return 100;
    }
    

    pub fn isScalable(&mut self) -> bool {
        return self.scalable.clone();
    }
    

    pub fn isAutoHide(&mut self) -> bool {
        return false;
    }
    

    pub fn toString(&mut self) -> String {
        let mut NEW_LINE: String = jcall!(CommonSeps::getInstance()).NEW_LINE.clone();
        let mut osBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(osBuffer).append(String::from("Operating System Info: \n"));
        jcall!(osBuffer).append(self.getName());
        jcall!(osBuffer).append(NEW_LINE.clone());
        jcall!(osBuffer).append(self.getArch());
        jcall!(osBuffer).append(NEW_LINE.clone());
        jcall!(osBuffer).append(self.getVersion());
        jcall!(osBuffer).append(NEW_LINE.clone());
        jcall!(osBuffer).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(self.getCharacterSet())));
        jcall!(osBuffer).append(NEW_LINE.clone());
        return jcall!(osBuffer).toString();
    }
}
pub trait GenericOperatingSystemDyn: crate::org::allbinary::logic::system::os::OperatingSystemInterface::OperatingSystemInterface {
    fn as_GenericOperatingSystem(&self) -> &GenericOperatingSystem;
    fn as_GenericOperatingSystem_mut(&mut self) -> &mut GenericOperatingSystem;
}
impl GenericOperatingSystemDyn for GenericOperatingSystem {
    fn as_GenericOperatingSystem(&self) -> &GenericOperatingSystem { self }
    fn as_GenericOperatingSystem_mut(&mut self) -> &mut GenericOperatingSystem { self }
}


