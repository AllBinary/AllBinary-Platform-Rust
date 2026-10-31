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
// package org.allbinary.logic.io.path
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::io::file::FilePathData::FilePathData;
use crate::org::allbinary::logic::io::file::FilePathUtil::FilePathUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::AbPathData::AbPathData;
use super::PathUtil::PathUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbPath {
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub abPathData: Option<Rc<RefCell<AbPathData>>>,
    pub abPathUtil: Option<Rc<RefCell<PathUtil>>>,
    pub EMPTY_STRING: String,
    pub schema: String,
    pub path: String,
    pub nameP: String,
    pub hasSchemaP: bool,
    pub numberOfSeps: i32,
}

thread_local! { pub static AbPath_NETWORK_SEP: String = String::from(":/"); }

impl AbPath {
    

    pub fn createAbPath() -> Option<Rc<RefCell<dyn AbPathDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut EMPTY_STRING: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
            return AbPath::new(EMPTY_STRING.clone(), EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn AbPathDyn>>);
        })) {
            Ok(java_return) => return java_return,
            Err(ex) => {
            let ex: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(ex as Box<dyn std::any::Any>)));

            panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
            }
        }
    }
    

    pub fn new_struct(mut aPath: String, mut name: String) -> Self {
        let mut this = Self {
            commonSeps: None,
            abPathData: None,
            abPathUtil: None,
            EMPTY_STRING: String::new(),
            schema: String::new(),
            path: String::new(),
            nameP: String::new(),
            hasSchemaP: false,
            numberOfSeps: 0,
        };
        this.commonSeps = CommonSeps::getInstance();
        this.abPathData = AbPathData::getInstance();
        this.abPathUtil = PathUtil::getInstance();
        this.EMPTY_STRING = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.schema = this.EMPTY_STRING.clone();
        this.path = this.EMPTY_STRING.clone();
        this.nameP = this.EMPTY_STRING.clone();
        this.hasSchemaP = false;
        this.numberOfSeps = 0;
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        if !jcall!(stringValidationUtil).isEmpty(aPath.clone()) {
            this.schema = this.getSchema(aPath.clone());
            this.nameP = name.clone();
            this.path = { let java_receiver = this.abPathUtil.clone(); let java_arguments = ({ let java_receiver = this.abPathUtil.clone(); let java_arguments = (this.getPathFromPath(aPath.clone()),); let java_result = jcall!(java_receiver).adjust(java_arguments.0); java_result },); let java_result = jcall!(java_receiver).adjustEnd(java_arguments.0); java_result };
        } else {
            this.init();
            if !jcall!(stringValidationUtil).isEmpty(name.clone()) {
                this.nameP = name.clone();
            }
        }
        this
    }

    pub fn new(mut aPath: String, mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(aPath, name))))
    }
    

    pub fn init(&mut self) {
        self.schema = self.EMPTY_STRING.clone();
        self.setPath(jcall!(self.abPathData).SEPARATOR.clone());
        self.nameP = self.EMPTY_STRING.clone();
    }
    

    pub fn getSchema(&mut self, mut aPath: String) -> String {
        let mut beginIndex: i32 = aPath.find(&jcall!(self.commonSeps).COLON.clone()).map(|index| index as i32).unwrap_or(-1);
        if beginIndex >= 0 {
            self.hasSchemaP = true;
            return aPath[(0) as usize..(beginIndex) as usize].to_string();
        }
        self.hasSchemaP = false;
        return self.EMPTY_STRING.clone();
    }
    

    pub fn hasSchema(&mut self) -> bool {
        return self.hasSchemaP.clone();
    }
    

    // throws Exception
    pub fn getPathFromPath(&mut self, mut aPath: String) -> String {
        let mut filePathData: Option<Rc<RefCell<FilePathData>>> = FilePathData::getInstance();
        let mut tempPath: String = aPath.clone();
        if !self.hasSchema() {
            tempPath = jcall!(self.abPathUtil).adjustStart(tempPath.clone());
        } else {
            let mut beginIndex: i32 = tempPath.find(&jcall!(self.commonSeps).COLON.clone()).map(|index| index as i32).unwrap_or(-1);
            if beginIndex >= 0 {
                beginIndex += 1;
                while tempPath.chars().nth((beginIndex.clone()) as usize).unwrap() == jcall!(self.abPathData).SEPARATORCHAR || tempPath.chars().nth((beginIndex.clone()) as usize).unwrap() == jcall!(filePathData).SEPARATORCHAR {
                    beginIndex += 1;
                    self.numberOfSeps += 1;
                    if self.numberOfSeps > 2 {
                        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Should Not Have More Than Two Seps"))));
                    }
                }
                tempPath = tempPath[(beginIndex) as usize..((tempPath.chars().count() as i32)) as usize].to_string();
            }
        }
        return tempPath.clone();
    }
    

    pub fn setName(&mut self, mut name: String) {
        self.nameP = name.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.nameP.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        if self.hasSchema() {
            jcall!(stringBuffer).append(self.schema.clone());
            jcall!(stringBuffer).append(AbPath_NETWORK_SEP.with(Clone::clone).clone());
        }
        jcall!(stringBuffer).append(self.getPath());
        jcall!(stringBuffer).append(self.nameP.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn toFileSystemString(&mut self) -> String {
        if self.hasSchema() {
            let mut filePathData: Option<Rc<RefCell<FilePathData>>> = FilePathData::getInstance();
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(self.schema.clone());
            jcall!(stringBuffer).append(jcall!(self.commonSeps).COLON.clone());
            jcall!(stringBuffer).append(jcall!(filePathData).PATH_START.clone());
            jcall!(stringBuffer).append(FilePathUtil::adjust(self.getPath()));
            jcall!(stringBuffer).append(self.nameP.clone());
            return jcall!(stringBuffer).toString();
        } else {
            return format!("{}{}", FilePathUtil::adjust(self.getPath()), self.nameP);
        }
    }
    

    pub fn getPath(&mut self) -> String {
        return self.path.clone();
    }
    

    pub fn setPath(&mut self, mut path: String) {
        self.path = path.clone();
    }
}
pub trait AbPathDyn {
    fn as_AbPath(&self) -> &AbPath;
    fn as_AbPath_mut(&mut self) -> &mut AbPath;
    fn init(&mut self) { AbPath::init(self.as_AbPath_mut()) }
    fn getSchema(&mut self, mut aPath: String) -> String { AbPath::getSchema(self.as_AbPath_mut(), aPath) }
    fn hasSchema(&mut self) -> bool { AbPath::hasSchema(self.as_AbPath_mut()) }
    fn getPathFromPath(&mut self, mut aPath: String) -> String { AbPath::getPathFromPath(self.as_AbPath_mut(), aPath) }
    fn setName(&mut self, mut name: String) { AbPath::setName(self.as_AbPath_mut(), name) }
    fn getName(&mut self) -> String { AbPath::getName(self.as_AbPath_mut()) }
    fn toString(&mut self) -> String { AbPath::toString(self.as_AbPath_mut()) }
    fn toFileSystemString(&mut self) -> String { AbPath::toFileSystemString(self.as_AbPath_mut()) }
    fn getPath(&mut self) -> String { AbPath::getPath(self.as_AbPath_mut()) }
    fn setPath(&mut self, mut path: String) { AbPath::setPath(self.as_AbPath_mut(), path) }
}
impl AbPathDyn for AbPath {
    fn as_AbPath(&self) -> &AbPath { self }
    fn as_AbPath_mut(&mut self) -> &mut AbPath { self }
}


