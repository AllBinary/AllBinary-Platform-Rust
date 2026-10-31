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
// package org.allbinary.logic.visual.transform.info.objectConfig.generator
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::TransformInfoObjectConfigGenerator::TransformInfoObjectConfigGenerator;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicGenerator {
    pub base: TransformInfoObjectConfigGenerator,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl BasicGenerator {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TransformInfoObjectConfigGenerator::new_struct(),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn process(&mut self, mut input: String) -> String {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
            jcall!(self.logUtil).putF(String::from("Processing"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone());
        }
        return input.clone();
    }
}
impl std::ops::Deref for BasicGenerator {
    type Target = TransformInfoObjectConfigGenerator;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::transform::info::objectConfig::generator::TransformInfoObjectConfigGenerator::TransformInfoObjectConfigGeneratorDyn for BasicGenerator {
    fn as_TransformInfoObjectConfigGenerator(&self) -> &crate::org::allbinary::logic::visual::transform::info::objectConfig::generator::TransformInfoObjectConfigGenerator::TransformInfoObjectConfigGenerator { &self.base }
    fn as_TransformInfoObjectConfigGenerator_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::objectConfig::generator::TransformInfoObjectConfigGenerator::TransformInfoObjectConfigGenerator { &mut self.base }
}
impl crate::org::allbinary::logic::visual::transform::info::objectConfig::generator::TransformInfoObjectConfigGeneratorInterface::TransformInfoObjectConfigGeneratorInterface for BasicGenerator {
    fn process(&mut self, mut input: String) -> String { Self::process(self, input) }
}


