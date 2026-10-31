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
// package org.allbinary.game.configuration.persistance
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::rms::RecordStore::RecordStore;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::persistance::PlatformRecordIdUtil::PlatformRecordIdUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::PersistanceStrings::PersistanceStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicPersitance {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub persistanceStrings: Option<Rc<RefCell<PersistanceStrings>>>,
    pub platformRecordIdUtil: Option<Rc<RefCell<PlatformRecordIdUtil>>>,
    pub recordId: String,
    pub valueList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub idList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl BasicPersitance {
    

    pub fn new_struct(mut recordId: String) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            persistanceStrings: None,
            platformRecordIdUtil: None,
            recordId: String::new(),
            valueList: None,
            idList: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.persistanceStrings = PersistanceStrings::getInstance();
        this.platformRecordIdUtil = PlatformRecordIdUtil::getInstance();
        this.valueList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.idList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.recordId = recordId.clone();
        this
    }

    pub fn new(mut recordId: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(recordId))))
    }
    

    // throws Exception
    pub fn deleteRecoreStore(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        RecordStore::deleteRecordStore(self.getRecordId(abeClientInformation.clone()));
    }
    

    // throws Exception
    pub fn deleteAll(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        let mut size: i32 = jcall!(self.idList).size();
        for index in (0..size) {
            let mut integer: Option<i32> = { let java_object = jcall!(self.idList).as_BasicArrayList_mut().objectArray[(index) as usize]; java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) };
            { let java_arguments = (abeClientInformation.clone(), ((integer).unwrap() as i32),); self.delete(java_arguments.0, java_arguments.1) };
        }
        self.clear();
    }
    

    // throws Exception
    pub fn delete(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut deleteId: i32) {
        let mut recordStore: Option<Rc<RefCell<RecordStore>>> = unsafe { NullRecordStore_NULL_RECORD_STORE.clone() }.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.persistanceStrings).DELETING_WITH_ID.clone()).appendint(deleteId.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).delete.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            recordStore = RecordStore::openRecordStore(self.getRecordId(abeClientInformation.clone()), true);
            jcall!(recordStore).deleteRecord(deleteId.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
        { // finally
            if recordStore.is_some() {
                PreLogUtil::put(jcall!(self.persistanceStrings).CLOSING_RECORDSTORE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).delete.clone());
                jcall!(recordStore).closeRecordStore();
            }
        }
    }
    

    pub fn getRecordId(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) -> String {
        return jcall!(self.platformRecordIdUtil).getRecordId(abeClientInformation.clone(), self.recordId.clone());
    }
    

    pub fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.valueList.clone();
    }
    

    pub fn getIds(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.idList.clone();
    }
    

    pub fn clear(&mut self) {
        jcall!(self.valueList).clear();
        jcall!(self.idList).clear();
    }
}
pub trait BasicPersitanceDyn {
    fn as_BasicPersitance(&self) -> &BasicPersitance;
    fn as_BasicPersitance_mut(&mut self) -> &mut BasicPersitance;
    fn deleteRecoreStore(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) { BasicPersitance::deleteRecoreStore(self.as_BasicPersitance_mut(), abeClientInformation) }
    fn deleteAll(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) { BasicPersitance::deleteAll(self.as_BasicPersitance_mut(), abeClientInformation) }
    fn delete(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut deleteId: i32) { BasicPersitance::delete(self.as_BasicPersitance_mut(), abeClientInformation, deleteId) }
    fn getRecordId(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) -> String { BasicPersitance::getRecordId(self.as_BasicPersitance_mut(), abeClientInformation) }
    fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { BasicPersitance::getList(self.as_BasicPersitance_mut()) }
    fn getIds(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { BasicPersitance::getIds(self.as_BasicPersitance_mut()) }
    fn clear(&mut self) { BasicPersitance::clear(self.as_BasicPersitance_mut()) }
}
impl BasicPersitanceDyn for BasicPersitance {
    fn as_BasicPersitance(&self) -> &BasicPersitance { self }
    fn as_BasicPersitance_mut(&mut self) -> &mut BasicPersitance { self }
}


