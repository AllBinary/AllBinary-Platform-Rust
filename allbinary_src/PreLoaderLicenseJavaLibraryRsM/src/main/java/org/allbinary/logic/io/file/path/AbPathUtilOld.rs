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
// package org.allbinary.logic.io.file.path
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::file::FilePathData::FilePathData;
use crate::org::allbinary::logic::io::path::AbFilePath::AbFilePath;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//Used to correct user input for paths
pub struct AbPathUtilOld {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub abPathData: Option<Rc<RefCell<AbPathData>>>,
    pub filePathData: Option<Rc<RefCell<FilePathData>>>,
}

thread_local! { pub static AbPathUtilOld_instance: Option<Rc<RefCell<AbPathUtilOld>>> = AbPathUtilOld::new(); }
thread_local! { pub static AbPathUtilOld_MIN: i32 = 4; }

impl AbPathUtilOld {
    

    pub fn getInstance() -> Option<Rc<RefCell<AbPathUtilOld>>> {
        return AbPathUtilOld_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            abPathData: None,
            filePathData: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.abPathData = AbPathData::getInstance();
        this.filePathData = FilePathData::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getExtension(&mut self, mut abFilePath: Option<Rc<RefCell<AbFilePath>>>) -> String {
        return { let java_arguments = (jcall!(abFilePath).toString(),); self.getExtension(java_arguments.0) };
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getExtension(&mut self, mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> String {
        return { let java_arguments = (jcall!(abPath).toString(),); self.getExtension(java_arguments.0) };
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getExtension(&mut self, mut filePath: String) -> String {
        if (filePath.chars().count() as i32) < AbPathUtilOld_MIN.with(Clone::clone) {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Could not be a file path since its less that 4 characters"))));
        }
        let mut beginIndex: i32 = (filePath.chars().count() as i32) - AbPathUtilOld_MIN.with(Clone::clone);
        let mut extension: String = filePath[(beginIndex + 1) as usize..].to_string();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("FileExtension: "), extension), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getExtension()"));
        }
        return extension.clone();
    }
    

    // throws Exception
    pub fn getWithoutExtension(&mut self, mut abFilePath: Option<Rc<RefCell<AbFilePath>>>) -> String {
        return { let java_arguments = (jcall!(abFilePath).toString(),); self.getWithoutExtension(java_arguments.0) };
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getWithoutExtension(&mut self, mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> String {
        return { let java_arguments = (jcall!(abPath).toString(),); self.getWithoutExtension(java_arguments.0) };
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getWithoutExtension(&mut self, mut filePath: String) -> String {
        if (filePath.chars().count() as i32) < AbPathUtilOld_MIN.with(Clone::clone) {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Could not be a file path since its less that 4 characters"))));
        }
        let mut endIndex: i32 = (filePath.chars().count() as i32) - AbPathUtilOld_MIN.with(Clone::clone);
        let mut pathWithoutExtension: String = filePath[(0) as usize..(endIndex) as usize].to_string();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("FileWithoutExtension: "), pathWithoutExtension), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getWithoutExtension()"));
        }
        return pathWithoutExtension.clone();
    }
    

    pub fn isValid(&mut self, mut path: String) -> bool {
        if self.isValidStart(path.clone()) {
            if self.isValidEnd(path.clone()) {
                return true;
            }
        }
        return false;
    }
    

    pub fn isValidStart(&mut self, mut path: String) -> bool {
        if path.chars().nth((0) as usize).unwrap() == jcall!(self.abPathData).SEPARATORCHAR {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn isValidEnd(&mut self, mut path: String) -> bool {
        if jcall!(StringValidationUtil::getInstance()).isEmpty(path.clone()) {
            return false;
        }
        if path.chars().nth(((path.chars().count() as i32) - 1) as usize).unwrap() == jcall!(self.abPathData).SEPARATORCHAR {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn adjustStart(&mut self, mut path: String) -> String {
        if !self.isValidStart(path.clone()) {
            path = format!("{}{}", jcall!(self.abPathData).SEPARATOR, path);
        }
        return path.clone();
    }
    

    pub fn adjustEnd(&mut self, mut path: String) -> String {
        if !self.isValidEnd(path.clone()) {
            path = format!("{}{}", path, jcall!(self.abPathData).SEPARATOR);
        }
        return path.clone();
    }
    

    pub fn adjust(&mut self, mut path: String) -> String {
        if jcall!(StringValidationUtil::getInstance()).isEmpty(path.clone()) {
            return jcall!(self.abPathData).SEPARATOR.clone();
        }
        if path.find(&String::from("\\")).map(|index| index as i32).unwrap_or(-1) >= 0 {
            path = path.replace(&'\\', &jcall!(self.abPathData).SEPARATORCHAR);
        }
        return path.clone();
    }
    

    // synchronized
    pub fn getNameFromPath(&mut self, mut categoryPath: String) -> String {
        let mut endIndex: i32 = categoryPath.rfind(&jcall!(self.abPathData).SEPARATOR.clone()).map(|index| index as i32).unwrap_or(-1);
        if endIndex < 0 {
            endIndex = categoryPath.rfind(jcall!(self.filePathData).SEPARATORCHAR.clone()).map(|index| index as i32).unwrap_or(-1);
        }
        if endIndex < 0 {
            return categoryPath.clone();
        }
        if (categoryPath.chars().count() as i32) == endIndex + 1 {
            let mut categoryName: String = categoryPath[(0) as usize..(endIndex) as usize].to_string();
            return self.getNameFromPath(categoryName.clone());
        } else {
            let mut categoryName: String = categoryPath[(endIndex + 1) as usize..].to_string();
            return categoryName.clone();
        }
    }
    

    // throws Exception
    // synchronized
    pub fn removeNameFromPath(&mut self, mut categoryPath: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        let mut endIndex: i32 = categoryPath.rfind(&jcall!(self.abPathData).SEPARATOR.clone()).map(|index| index as i32).unwrap_or(-1);
        if endIndex < 0 {
            endIndex = categoryPath.rfind(jcall!(self.filePathData).SEPARATORCHAR.clone()).map(|index| index as i32).unwrap_or(-1);
        }
        if endIndex < 0 {
            return AbPath::createAbPath();
        }
        if (categoryPath.chars().count() as i32) == endIndex + 1 {
            return { let java_arguments = (categoryPath[(0) as usize..(endIndex - 1) as usize].to_string(),); self.removeNameFromPath(java_arguments.0) };
        } else {
            let mut categoryName: String = categoryPath[(0) as usize..(endIndex) as usize].to_string();
            return AbPath::new(categoryName.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        }
    }
}

