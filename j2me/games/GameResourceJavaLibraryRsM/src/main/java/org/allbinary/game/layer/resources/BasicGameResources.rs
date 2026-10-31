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
// package org.allbinary.game.layer.resources
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::GameConfigurationCentral::GameConfigurationCentral;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicGameResources {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub RESOURCE: String,
    pub RESOURCE_DESTROY: String,
}

thread_local! { pub static BasicGameResources_stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new(); }

impl BasicGameResources {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            RESOURCE: String::new(),
            RESOURCE_DESTROY: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.RESOURCE = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.RESOURCE_DESTROY = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self, mut ROOT: String, mut SIZE: Vec<String>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut scale: i32 = ((jcall!(jcall!(GameConfigurationCentral::getInstance()).SCALE).getValue()).unwrap() as i32);
            if scale > 1 && scale - 1 < (SIZE.len() as i32) {
                self.append(ROOT.clone(), SIZE[(scale - 1) as usize].clone());
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(StringMaker::new()).append(String::from("Invalid Value: ")).appendint(scale.clone()).toString())));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).INIT.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn append(&mut self, mut ROOT: String, mut sizeString: String) {
        let mut DESTROY: String = String::from("_destroy");
        let mut string: String = self.getString();
        jcall!(BasicGameResources_stringBuffer.with(Clone::clone)).delete(0, jcall!(BasicGameResources_stringBuffer.with(Clone::clone)).length());
        jcall!(BasicGameResources_stringBuffer.with(Clone::clone)).append(ROOT.clone());
        jcall!(BasicGameResources_stringBuffer.with(Clone::clone)).append(string.clone());
        jcall!(BasicGameResources_stringBuffer.with(Clone::clone)).append(sizeString.clone());
        self.RESOURCE = jcall!(BasicGameResources_stringBuffer.with(Clone::clone)).toString();
        jcall!(BasicGameResources_stringBuffer.with(Clone::clone)).delete(0, jcall!(BasicGameResources_stringBuffer.with(Clone::clone)).length());
        jcall!(BasicGameResources_stringBuffer.with(Clone::clone)).append(ROOT.clone());
        jcall!(BasicGameResources_stringBuffer.with(Clone::clone)).append(DESTROY.clone());
        jcall!(BasicGameResources_stringBuffer.with(Clone::clone)).append(string.clone());
        jcall!(BasicGameResources_stringBuffer.with(Clone::clone)).append(sizeString.clone());
        self.RESOURCE_DESTROY = jcall!(BasicGameResources_stringBuffer.with(Clone::clone)).toString();
    }
    

    // throws Exception
    pub fn getString(&mut self) -> String {
        return jcall!(GameGraphicsResourceUtil::GameGraphicsResourceUtil::getInstance()).getName();
    }
}
pub trait BasicGameResourcesDyn {
    fn as_BasicGameResources(&self) -> &BasicGameResources;
    fn as_BasicGameResources_mut(&mut self) -> &mut BasicGameResources;
    fn init(&mut self, mut ROOT: String, mut SIZE: Vec<String>) { BasicGameResources::init(self.as_BasicGameResources_mut(), ROOT, SIZE) }
    fn append(&mut self, mut ROOT: String, mut sizeString: String) { BasicGameResources::append(self.as_BasicGameResources_mut(), ROOT, sizeString) }
    fn getString(&mut self) -> String { BasicGameResources::getString(self.as_BasicGameResources_mut()) }
}
impl BasicGameResourcesDyn for BasicGameResources {
    fn as_BasicGameResources(&self) -> &BasicGameResources { self }
    fn as_BasicGameResources_mut(&mut self) -> &mut BasicGameResources { self }
}


