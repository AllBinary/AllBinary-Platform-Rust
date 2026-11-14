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
// package org.allbinary.logic.system.loader
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::lang::reflect::Constructor::Constructor;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::java::anyType::ConstructorUtil::ConstructorUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::AbKeys::AbKeys;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::java::lang::Exception::*;
use crate::java::lang::ClassLoader::*;
use super::AbeClassLoader::AbeClassLoader;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbeFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub useCustomLoader: bool,
}

thread_local! { pub static AbeFactory_instance: Option<Rc<RefCell<AbeFactory>>> = AbeFactory::new(); }

impl AbeFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AbeFactory>>> {
        return AbeFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            useCustomLoader: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.useCustomLoader = false;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws LicensingException
    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn getInstance2(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut className: String) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return Some(crate::java_into_object(jcall!(self.getClass(abeClientInformation.clone(), className.clone())).newInstance()));
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).LOADERERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", String::from("Failure for: "), className), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstance(classname)"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws LicensingException
    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn getInstance4(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut className: String, mut classes: Vec<&'static str>, mut params: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut constructor: Option<Rc<RefCell<Constructor>>> = None;
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut parent: Option<Rc<RefCell<ClassLoader>>> = WebappClassLoaderInfo::WebappClassLoaderInfo::getLoader();
            if self.useCustomLoader {
                let mut loader: Option<Rc<RefCell<ClassLoader>>> = AbeClassLoader::new(parent.clone(), jcall!(AbKeys::getInstance()).getKey(abeClientInformation.clone(), className.clone())).map(|reference| Rc::new(RefCell::new(Rc::try_unwrap(reference).ok().expect("new object is not shared").into_inner().base)));
                let mut myClass: &'static str = jcall!(loader).loadClass(className.clone());
                constructor = { let java_value = (jcall!(myClass).getConstructor(classes.clone())).clone(); java_value };
                return Some(crate::java_into_object(jcall!(constructor).newInstance(params.clone())));
            } else {
                let mut myClass: &'static str = jcall!(parent).loadClass(className.clone());
                constructor = { let java_value = (jcall!(myClass).getConstructor(classes.clone())).clone(); java_value };
                return Some(crate::java_into_object(jcall!(constructor).newInstance(params.clone())));
            }None
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).LOADERERROR.clone(); java_field }.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("Failure for: "));
                jcall!(stringBuffer).append(className.clone());
                jcall!(stringBuffer).append({ let java_field = jcall!(CommonSeps::getInstance()).SPACE.clone(); java_field }.clone());
                jcall!(stringBuffer).append(ConstructorUtil::view(constructor.clone(), { let java_field = jcall!(CommonSeps::getInstance()).NEW_LINE.clone(); java_field }.clone()));
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getInstance(className,params)"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws LicensingException
    // synchronized
    pub fn getClass(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut className: String) -> &'static str {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut parent: Option<Rc<RefCell<ClassLoader>>> = WebappClassLoaderInfo::WebappClassLoaderInfo::getLoader();
            if self.useCustomLoader {
                let mut loader: Option<Rc<RefCell<ClassLoader>>> = AbeClassLoader::new(parent.clone(), jcall!(AbKeys::getInstance()).getKey(abeClientInformation.clone(), className.clone())).map(|reference| Rc::new(RefCell::new(Rc::try_unwrap(reference).ok().expect("new object is not shared").into_inner().base)));
                let mut c: &'static str = jcall!(loader).loadClass(className.clone());
                return Some(c.clone());
            } else {
                let mut c: &'static str = jcall!(parent).loadClass(className.clone());
                return Some(c.clone());
            }None
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).LOADERERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", String::from("Failure for: "), className), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getClass(className)"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}

