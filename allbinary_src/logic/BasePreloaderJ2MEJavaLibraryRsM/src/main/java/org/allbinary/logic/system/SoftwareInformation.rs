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
// package org.allbinary.logic.system
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::KotlinUtil::KotlinUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SoftwareInformation {
    pub name: String,
    pub shortName: String,
    pub version: String,
}


impl SoftwareInformation {
    

    pub fn new_struct(mut name: String, mut version: String, mut shortName: String) -> Self {
        let mut this = Self {
            name: String::new(),
            shortName: String::new(),
            version: String::new(),
        };
        this.name = name.clone();
        this.version = version.clone();
        this.shortName = jcall!(StringMaker::new()).append(shortName.clone()).append(KotlinUtil::getShortName()).toString();
        this
    }

    pub fn new(mut name: String, mut version: String, mut shortName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, version, shortName))))
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getVersion(&mut self) -> String {
        return self.version.clone();
    }
    

    pub fn toShortString(&mut self) -> String {
        return self.shortName.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut NAME: String = String::from("Name: ");
        let mut VERSION: String = String::from("Version: ");
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(NAME.clone());
        jcall!(stringBuffer).append(self.getName());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(stringBuffer).append(VERSION.clone());
        jcall!(stringBuffer).append(self.getVersion());
        return jcall!(stringBuffer).toString();
    }
}

