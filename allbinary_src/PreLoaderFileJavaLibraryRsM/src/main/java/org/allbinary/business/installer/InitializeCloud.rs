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
use crate::org::allbinary::globals::PATH_GLOBALS::PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::AbIOSystem::AbIOSystem;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::FileUtil::FileUtil;
use crate::org::allbinary::logic::io::file::directory::Directory::Directory;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InitializeCloud {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static InitializeCloud_CLOUD: String = String::from("cloud/"); }
thread_local! { pub static InitializeCloud_CLOUD_UPDATE: String = String::from("cloudupdate/"); }

impl InitializeCloud {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn initialize(&mut self, mut cloud: String, mut overwriteNewer: bool, mut overwriteAll: bool, mut current: i32, mut total: i32) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(AbIOSystem::getInstance()).isType(String::from("com.vobject.appengine.java.io")) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(URLGLOBALS::getWebappPath());
                jcall!(stringBuffer).append(cloud.clone());
                jcall!(stringBuffer).append(jcall!(PATH_GLOBALS::getInstance()).DATA_PATH.clone());
                let mut path: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(jcall!(stringBuffer).toString(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append(URLGLOBALS::getWebappPath());
                let mut realPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(jcall!(stringBuffer).toString(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
                let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(path.clone());
                let mut fileBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(Directory::getInstance()).search2(file.clone(), true);
                let mut size: i32 = jcall!(fileBasicArrayList).size();
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append(String::from("Searched: "));
                jcall!(stringBuffer).append(jcall!(path).toFileSystemString());
                jcall!(stringBuffer).append(String::from(" BasicArrayList: "));
                jcall!(stringBuffer).appendint(size.clone());
                let mut portion: i32 = size / total + 1;
                let mut start: i32 = portion * current;
                let mut end: i32 = start + portion;
                if end > size {
                    end = size.clone();
                }
                jcall!(stringBuffer).append(String::from(" Section: "));
                jcall!(stringBuffer).appendint(start.clone());
                jcall!(stringBuffer).append(String::from(" - "));
                jcall!(stringBuffer).appendint(end.clone());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("initialize()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                let mut nextFile: Option<Rc<RefCell<AbFile>>> = None;
                for index in (start..end) {
                    nextFile = { let java_object = (jcall!(fileBasicArrayList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<AbFile>>>>().expect("ClassCastException").clone()) };
                    if jcall!(nextFile).isDirectory() {
                    } else {
                        jcall!(FileUtil::getInstance()).copyToCloud6(nextFile.clone(), path.clone(), realPath.clone(), cloud.clone(), overwriteNewer.clone(), overwriteAll.clone());
                    }
                }
                jcall!(self.logUtil).putF(String::from("Copied Files To Cloud"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("initialize()"));
            }
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(String::from("Unable to copy installer files into cloud"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("initialize()"), crate::java_into_object(e.clone()));
            return false;
            }
        }
    }
}

