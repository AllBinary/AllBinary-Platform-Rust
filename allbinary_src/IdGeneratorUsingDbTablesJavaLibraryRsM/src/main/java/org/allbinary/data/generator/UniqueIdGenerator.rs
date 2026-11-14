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
use crate::java::io::IOException::IOException;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::data::tables::generator::IdGeneratorEntity::IdGeneratorEntity;
use crate::org::allbinary::data::tables::generator::IdGeneratorEntityFactory::IdGeneratorEntityFactory;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::java::lang::Exception::*;
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

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            idGeneratorEntity: None,
            name: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.idGeneratorEntity = { let java_value = ((IdGeneratorEntityFactory::getInstance()).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<IdGeneratorEntity>() }, "ClassCastException: not a IdGeneratorEntity"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<IdGeneratorEntity>) } })).clone(); java_value };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn initialize(&mut self, mut value: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(vector).add(crate::java_into_object(self.name.clone()));
            jcall!(vector).add(crate::java_into_object(jcall!(Some((value.clone()) as i64)).toString()));
            jcall!(self.idGeneratorEntity).insert(vector.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).IDLOGGING.clone(); java_field }.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("initialize"), crate::java_into_object(e.clone()));
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
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut idLong: Option<i64> = jcall!(self.idGeneratorEntity).get(self.name.clone());
            let mut newValue: Option<i64> = Some((((idLong).unwrap() as i64) + 1_i64) as i64);
            jcall!(self.idGeneratorEntity).update2(self.name.clone(), newValue.clone());
            return Some((idLong).expect("NullPointerException").to_string());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).IDLOGGING.clone(); java_field }.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getNext"), crate::java_into_object(e.clone()));
            }
            return String::from("Error");
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
}
pub trait UniqueIdGeneratorDyn: std::any::Any + crate::org::allbinary::data::generator::IdGeneratorInterface::IdGeneratorInterface {
    fn as_UniqueIdGenerator(&self) -> &UniqueIdGenerator;
    fn as_UniqueIdGenerator_mut(&mut self) -> &mut UniqueIdGenerator;
    fn initialize(&mut self, mut value: i32) { UniqueIdGenerator::initialize(self.as_UniqueIdGenerator_mut(), value) }
    fn setFile(&mut self, mut filePathName: String, mut name: String) { UniqueIdGenerator::setFile(self.as_UniqueIdGenerator_mut(), filePathName, name) }
}
impl UniqueIdGeneratorDyn for UniqueIdGenerator {
    fn as_UniqueIdGenerator(&self) -> &UniqueIdGenerator { self }
    fn as_UniqueIdGenerator_mut(&mut self) -> &mut UniqueIdGenerator { self }
}


