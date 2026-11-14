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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::init::Init::Init;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct KeyFactoryInitializer {
    pub base: Init,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl KeyFactoryInitializer {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Init::new_struct(),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone());
        jcall!(SmallIntegerSingletonFactory::getInstance()).initWithRange(0x2D0, 6);
    }
}
impl std::ops::Deref for KeyFactoryInitializer {
    type Target = Init;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for KeyFactoryInitializer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::init::Init::InitDyn for KeyFactoryInitializer {
    fn as_Init(&self) -> &crate::org::allbinary::init::Init::Init { &self.base }
    fn as_Init_mut(&mut self) -> &mut crate::org::allbinary::init::Init::Init { &mut self.base }
}
impl crate::org::allbinary::init::InitInterface::InitInterface for KeyFactoryInitializer {
    fn init(&mut self) { Self::init(self) }
}


