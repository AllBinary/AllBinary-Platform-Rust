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
// package org.allbinary.business.init
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypeFactory::LogConfigTypeFactory;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::directory::SubDirectory::SubDirectory;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WeblisketFinder {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub subDirectory: Option<Rc<RefCell<SubDirectory>>>,
}

thread_local! { pub static WeblisketFinder_instance: Option<Rc<RefCell<WeblisketFinder>>> = WeblisketFinder::new(); }
thread_local! { pub static WeblisketFinder_KEY: String = String::from("installerdata.dat"); }

impl WeblisketFinder {
    

    pub fn getInstance() -> Option<Rc<RefCell<WeblisketFinder>>> {
        return WeblisketFinder_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            subDirectory: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.subDirectory = SubDirectory::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn findAll(&mut self, mut path: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("findAll"));
            return { let java_receiver = self.subDirectory.clone(); let java_arguments = (WeblisketFinder_KEY.with(Clone::clone).clone(), AbFile::createAbFile(path.clone()),); let java_result = jcall!(java_receiver).search2(java_arguments.0, java_arguments.1); java_result };
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("findAll"), crate::java_into_object(e.clone()));
            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).PRELOADERERROR.clone())) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("findAll"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn getInstallationPath(&mut self, mut file: Option<Rc<RefCell<AbFile>>>) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut filePath: String = jcall!(file).getPath();
            let mut end: i32 = filePath.find(&WeblisketFinder_KEY.with(Clone::clone).clone()).map(|index| index as i32).unwrap_or(-1);
            if end < 0 {
                return None;
            }
            return filePath[(0) as usize..(end) as usize].to_string();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRELOADERERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstallationPath"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    // throws Exception
    pub fn main(mut args: Vec<String>) {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(WeblisketFinder::getInstance()).findAll(jcall!(stringUtil).EMPTY_STRING.clone());
        println!("{}", jcall!(vector).size());
    }
}

