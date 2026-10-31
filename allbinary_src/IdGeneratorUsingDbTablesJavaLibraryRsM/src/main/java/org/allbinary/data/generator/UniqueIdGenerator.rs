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
// use crate::java::io::IOException::IOException;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tables::generator::IdGeneratorEntity::IdGeneratorEntity;
use crate::org::allbinary::data::tables::generator::IdGeneratorEntityFactory::IdGeneratorEntityFactory;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::IdGeneratorInterface::IdGeneratorInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UniqueIdGenerator {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub idGeneratorEntity: Option<Rc<RefCell<IdGeneratorEntity>>>,
    pub name: String,
}

thread_local! { pub static UniqueIdGenerator_EXT: String = String::from(".unq"); }
impl IdGeneratorInterface for UniqueIdGenerator {
    fn getNext(&mut self) -> String { Self::getNext(self) }
}

impl UniqueIdGenerator {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            idGeneratorEntity: None,
            name: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.idGeneratorEntity = (IdGeneratorEntityFactory::getInstance()).clone() /* as IdGeneratorEntity */;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn initialize(&mut self, mut value: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(vector).add(crate::java_into_object(self.name.clone()));
            jcall!(vector).add(crate::java_into_object(jcall!((value.clone()).to_string()).toString()));
            jcall!(self.idGeneratorEntity).insert(vector.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).IDLOGGING.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("initialize"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    // throws Exception
    // synchronized
    pub fn setFile(&mut self, mut filePathName: String, mut name: String) {
        self.name = name.clone();
    }
    

    // throws IOException
    // synchronized
    pub fn getNext(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut idLong: Option<i64> = jcall!(self.idGeneratorEntity).get(self.name.clone());
            let mut newValue: Option<i64> = (((idLong).unwrap() as i64) + 1_i64).to_string();
            jcall!(self.idGeneratorEntity).update2(self.name.clone(), newValue.clone());
            return (idLong).expect("NullPointerException").to_string();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).IDLOGGING.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getNext"), crate::java_into_object(e.clone()));
            }
            return String::from("Error");
            }
        }
    }
}
pub trait UniqueIdGeneratorDyn: crate::org::allbinary::data::generator::IdGeneratorInterface::IdGeneratorInterface {
    fn as_UniqueIdGenerator(&self) -> &UniqueIdGenerator;
    fn as_UniqueIdGenerator_mut(&mut self) -> &mut UniqueIdGenerator;
    fn initialize(&mut self, mut value: i32) { UniqueIdGenerator::initialize(self.as_UniqueIdGenerator_mut(), value) }
    fn setFile(&mut self, mut filePathName: String, mut name: String) { UniqueIdGenerator::setFile(self.as_UniqueIdGenerator_mut(), filePathName, name) }
}
impl UniqueIdGeneratorDyn for UniqueIdGenerator {
    fn as_UniqueIdGenerator(&self) -> &UniqueIdGenerator { self }
    fn as_UniqueIdGenerator_mut(&mut self) -> &mut UniqueIdGenerator { self }
}


