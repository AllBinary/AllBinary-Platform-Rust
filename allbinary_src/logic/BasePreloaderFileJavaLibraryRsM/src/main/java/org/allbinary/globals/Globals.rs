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
// package org.allbinary.globals
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::system::loader::WebappClassLoaderInfo::WebappClassLoaderInfo;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::AppUrlGlobals::AppUrlGlobals;
use super::ClassLoader::ClassLoader;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Globals {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static Globals_instance: Option<Rc<RefCell<Globals>>> = Globals::new(); }

impl Globals {
    

    pub fn getInstance() -> Option<Rc<RefCell<Globals>>> {
        return Globals_instance.with(Clone::clone).clone();
    }
    

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
    

    pub fn init(&mut self, mut classLoader: Option<Rc<RefCell<ClassLoader>>>, mut pathString: String) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut appUrlGlobals: Option<Rc<RefCell<AppUrlGlobals>>> = AppUrlGlobals::new();
        jcall!(appUrlGlobals).setWebappPath(pathString.clone());
        jcall!(URLGLOBALS).init(appUrlGlobals.clone());
        WebappClassLoaderInfo::setLoader(classLoader.clone());
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Setting Up AllBinary System Configuration Args: "));
        jcall!(stringBuffer).append(pathString.clone());
        jcall!(stringBuffer).append(String::from(" ClassLoader: "));
        jcall!(stringBuffer).append(std::any::type_name_of_val(&classLoader).to_string());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).INIT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        if jcall!(stringValidationUtil).isEmpty(pathString.clone()) {
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            jcall!(stringBuffer).append(String::from("No Path Provided. Using Process Path: "));
            jcall!(stringBuffer).append(pathString.clone());
            jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
            jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).EQUALS.clone());
            jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
            jcall!(stringBuffer).append(jcall!(URLGLOBALS).getWebappPath());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).INIT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(String::from("Webapp Path Set To: "));
        jcall!(stringBuffer).append(jcall!(URLGLOBALS).getWebappPath());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).INIT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
}

