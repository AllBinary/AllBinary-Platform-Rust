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
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use super::AbPathData::AbPathData;
use super::AbFilePath::AbFilePath;
use super::AbPath::AbPath;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//Used to correct user input for paths
pub struct PathUtil {
    pub abPathData: Option<Rc<RefCell<AbPathData>>>,
    pub filePathData: Option<Rc<RefCell<FilePathData>>>,
}

thread_local! { pub static PathUtil_instance: Option<Rc<RefCell<PathUtil>>> = PathUtil::new(); }

impl PathUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<PathUtil>>> {
        return PathUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            abPathData: None,
            filePathData: None,
        };
        this.abPathData = AbPathData::getInstance();
        this.filePathData = FilePathData::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getExtensionWithAbFilePath(&mut self, mut abFilePath: Option<Rc<RefCell<AbFilePath>>>) -> String {
        return { let java_receiver = self.abPathData.clone(); let java_arguments = (jcall!(abFilePath).toString(),); let java_result = jcall!(java_receiver).getExtension(java_arguments.0); java_result };
    }
    

    pub fn getExtensionWithAbPath(&mut self, mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> String {
        return { let java_receiver = self.abPathData.clone(); let java_arguments = (jcall!(abPath).toString(),); let java_result = jcall!(java_receiver).getExtension(java_arguments.0); java_result };
    }
    

    // throws Exception
    pub fn getWithoutExtensionWithAbFilePath(&mut self, mut abFilePath: Option<Rc<RefCell<AbFilePath>>>) -> String {
        return { let java_arguments = (jcall!(abFilePath).toString(),); self.getWithoutExtension(java_arguments.0) };
    }
    

    // throws Exception
    pub fn getWithoutExtensionWithAbPath(&mut self, mut abPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> String {
        return { let java_arguments = (jcall!(abPath).toString(),); self.getWithoutExtension(java_arguments.0) };
    }
    

    // throws Exception
    pub fn getWithoutExtension(&mut self, mut filePath: String) -> String {
        let mut indexOfFileExtensionDelmiter: i32 = jcall!(self.abPathData).getExtensionIndex(filePath.clone());
        let mut pathWithoutExtension: String = filePath.clone();
        if indexOfFileExtensionDelmiter >= 0 {
            pathWithoutExtension = filePath[(0) as usize..(indexOfFileExtensionDelmiter) as usize].to_string();
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
        if path.chars().nth((0) as usize).unwrap() == jcall!(self.abPathData).SEPARATORCHAR || path.chars().nth((0) as usize).unwrap() == '.' {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn isValidEnd(&mut self, mut path: String) -> bool {
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        if jcall!(stringValidationUtil).isEmpty(path.clone()) {
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
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        if jcall!(stringValidationUtil).isEmpty(path.clone()) {
            return jcall!(self.abPathData).SEPARATOR.clone();
        }
        if path.find(&String::from("\\")).map(|index| index as i32).unwrap_or(-1) >= 0 {
            path = path.replace(&'\\', &jcall!(self.abPathData).SEPARATORCHAR);
        }
        return path.clone();
    }
    

    // throws Exception
    pub fn removeNameFromPath(&mut self, mut path: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        let mut endIndex: i32 = path.rfind(&jcall!(self.abPathData).SEPARATOR.clone()).map(|index| index as i32).unwrap_or(-1);
        if endIndex < 0 {
            endIndex = path.rfind(jcall!(self.filePathData).SEPARATORCHAR.clone()).map(|index| index as i32).unwrap_or(-1);
        }
        if endIndex < 0 {
            return AbPath::createAbPath();
        }
        if (path.chars().count() as i32) == endIndex + 1 {
            return { let java_arguments = (path[(0) as usize..(endIndex - 1) as usize].to_string(),); self.removeNameFromPath(java_arguments.0) };
        } else {
            let mut pathWithoutName: String = path[(0) as usize..(endIndex) as usize].to_string();
            return AbPath::new(pathWithoutName.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        }
    }
}

