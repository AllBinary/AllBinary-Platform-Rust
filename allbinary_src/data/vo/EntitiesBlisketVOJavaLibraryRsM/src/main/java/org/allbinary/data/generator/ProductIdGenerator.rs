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
// package org.allbinary.data.generator
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::globals::PATH_GLOBALS::PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::UniqueIdGenerator::UniqueIdGenerator;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ProductIdGenerator {
    pub base: UniqueIdGenerator,
    pub IDFILE: String,
}


impl ProductIdGenerator {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: UniqueIdGenerator::new_struct(),
            IDFILE: String::new(),
        };
        this.IDFILE = String::from("productId");
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(URLGLOBALS::getMainPath());
        jcall!(stringBuffer).append(jcall!(PATH_GLOBALS::getInstance()).INIT_PATH.clone());
        jcall!(stringBuffer).append(this.IDFILE.clone());
        jcall!(stringBuffer).append(UniqueIdGenerator_EXT.with(Clone::clone).clone());
        this.base.setFile(jcall!(stringBuffer).toString(), this.IDFILE.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for ProductIdGenerator {
    type Target = UniqueIdGenerator;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ProductIdGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::data::generator::UniqueIdGenerator::UniqueIdGeneratorDyn for ProductIdGenerator {
    fn as_UniqueIdGenerator(&self) -> &crate::org::allbinary::data::generator::UniqueIdGenerator::UniqueIdGenerator { &self.base }
    fn as_UniqueIdGenerator_mut(&mut self) -> &mut crate::org::allbinary::data::generator::UniqueIdGenerator::UniqueIdGenerator { &mut self.base }
}
impl crate::org::allbinary::data::generator::IdGeneratorInterface::IdGeneratorInterface for ProductIdGenerator {
    fn getNext(&mut self) -> String { crate::org::allbinary::data::generator::UniqueIdGenerator::UniqueIdGenerator::getNext(crate::org::allbinary::data::generator::UniqueIdGenerator::UniqueIdGeneratorDyn::as_UniqueIdGenerator_mut(self)) }
}


