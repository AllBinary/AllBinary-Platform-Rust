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
// package org.allbinary.game.score
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::ByteArrayInputStream::ByteArrayInputStream;
// use crate::java::io::ByteArrayOutputStream::ByteArrayOutputStream;
// use crate::java::io::DataInputStream::DataInputStream;
// use crate::java::io::DataOutputStream::DataOutputStream;
// use crate::javax::microedition::rms::RecordEnumeration::RecordEnumeration;
// use crate::javax::microedition::rms::RecordStore::RecordStore;
// use crate::javax::microedition::rms::RecordStoreException::RecordStoreException;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::game::GameInfo::GameInfo;
use crate::org::allbinary::game::configuration::persistance::NullRecordComparator::NullRecordComparator;
use crate::org::allbinary::game::configuration::persistance::NullRecordFilter::NullRecordFilter;
use crate::org::allbinary::game::configuration::persistance::NullRecordStore::NullRecordStore;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::java::exception::ExceptionUtil::ExceptionUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::persistance::PlatformRecordIdUtil::PlatformRecordIdUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HighScoreNamePersistanceSingleton {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub platformRecordIdUtil: Option<Rc<RefCell<PlatformRecordIdUtil>>>,
    pub RECORD_ID: String,
    pub name: String,
    pub nameBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

pub static mut HighScoreNamePersistanceSingleton_SINGLETON: std::cell::LazyCell<Option<Rc<RefCell<HighScoreNamePersistanceSingleton>>>> = std::cell::LazyCell::new(|| HighScoreNamePersistanceSingleton::new());

impl HighScoreNamePersistanceSingleton {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            tsUtil: None,
            commonStrings: None,
            platformRecordIdUtil: None,
            RECORD_ID: String::new(),
            name: String::new(),
            nameBasicArrayList: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.tsUtil = TsUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.platformRecordIdUtil = PlatformRecordIdUtil::getInstance();
        this.RECORD_ID = String::from("_SN");
        this.name = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.nameBasicArrayList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<HighScoreNamePersistanceSingleton>>> {
        return unsafe { HighScoreNamePersistanceSingleton_SINGLETON.clone() }.clone();
    }
    

    pub fn clear(&mut self) {
        self.name = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
    

    // throws Exception
    pub fn deleteAll(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>) {
        let mut size: i32 = jcall!(self.nameBasicArrayList).size();
        for index in (0..size) {
            let mut integer: Option<i32> = { let java_object = jcall!(self.nameBasicArrayList).as_BasicArrayList_mut().objectArray[(index) as usize]; java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) };
            { let java_arguments = (abeClientInformation.clone(), gameInfo.clone(), ((integer).unwrap() as i32),); self.delete(java_arguments.0, java_arguments.1, java_arguments.2) };
        }
        self.clear();
    }
    

    pub fn getRecordId(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) -> String {
        return jcall!(self.platformRecordIdUtil).getRecordId(abeClientInformation.clone(), self.RECORD_ID.clone());
    }
    

    // throws Exception
    pub fn delete(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut deleteId: i32) {
        let mut recordStore: Option<Rc<RefCell<RecordStore>>> = unsafe { crate::org::allbinary::game::configuration::persistance::NullRecordStore::NullRecordStore_NULL_RECORD_STORE.clone() }.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Deleting: ")).appendint(deleteId.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).delete.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
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
                PreLogUtil::put(String::from("Closing RecordStore"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).delete.clone());
                jcall!(recordStore).closeRecordStore();
            }
        }
    }
    

    pub fn getIds(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.nameBasicArrayList.clone();
    }
    

    pub fn load(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>) -> String {
        let mut recordStore: Option<Rc<RefCell<RecordStore>>> = unsafe { crate::org::allbinary::game::configuration::persistance::NullRecordStore::NullRecordStore_NULL_RECORD_STORE.clone() }.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.name == jcall!(StringUtil::getInstance()).EMPTY_STRING {
                let mut LOADING_ID: String = String::from("Loading id: ");
                recordStore = RecordStore::openRecordStore(self.getRecordId(abeClientInformation.clone()), true);
                let mut recordEnum: Option<Rc<RefCell<RecordEnumeration>>> = jcall!(recordStore).enumerateRecords(unsafe { crate::org::allbinary::game::configuration::persistance::NullRecordFilter::NullRecordFilter_NULL_RECORD_FILTER.clone() }.clone(), crate::org::allbinary::game::configuration::persistance::NullRecordComparator::NullRecordComparator_NULL_RECORD_COMPARATOR.with(Clone::clone).clone(), true);
                let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
                let mut recordAsBytes: Vec<i8> = Vec::new();
                let mut byteArrayInputStream: Option<Rc<RefCell<ByteArrayInputStream>>> = None;
                let mut inputStream: Option<Rc<RefCell<DataInputStream>>> = None;
                while jcall!(recordEnum).hasNextElement() {
                    let mut id: i32 = jcall!(recordEnum).nextRecordId();
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(LOADING_ID.clone()).appendint(id.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).LOAD.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    recordAsBytes = jcall!(self.tsUtil).getRecord(crate::java_into_object(recordStore.clone()), id.clone());
                    byteArrayInputStream = ByteArrayInputStream::new(recordAsBytes.clone());
                    inputStream = DataInputStream::new(byteArrayInputStream.clone());
                    while jcall!(inputStream).available() > 0 {
                        self.name = jcall!(inputStream).readUTF();
                    }
                    { let java_receiver = self.nameBasicArrayList.clone(); let java_arguments = (crate::java_into_object(jcall!(smallIntegerSingletonFactory).getAt(id.clone())),); jcall!(java_receiver).add(java_arguments.0); };
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            self.save(abeClientInformation.clone(), gameInfo.clone(), self.name.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.commonStrings).EXCEPTION_LABEL.clone()).append(jcall!(ExceptionUtil::getInstance()).getStackTrace(e.clone())).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).LOAD.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
        }
        { // finally
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if recordStore.is_some() {
                    PreLogUtil::put(String::from("Closing RecordStore"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).LOAD.clone());
                    jcall!(recordStore).closeRecordStore();
                }
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).LOAD.clone(), crate::java_into_object(e.clone()));
                }
            }
        }
        return self.name.clone();
    }
    

    pub fn save(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut name: String) {
        let mut recordStore: Option<Rc<RefCell<RecordStore>>> = unsafe { crate::org::allbinary::game::configuration::persistance::NullRecordStore::NullRecordStore_NULL_RECORD_STORE.clone() }.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Saving: ")).append(name.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).SAVE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            recordStore = RecordStore::openRecordStore(self.getRecordId(abeClientInformation.clone()), true);
            let mut byteArrayOutputStream: Vec<u8> = Vec::new();
            let mut outputStream: Option<Rc<RefCell<DataOutputStream>>> = DataOutputStream::new(byteArrayOutputStream.clone());
            jcall!(outputStream).writeUTF(name.clone());
            let mut savedGameBytes: Vec<i8> = jcall!(byteArrayOutputStream).toByteArray();
            jcall!(recordStore).addRecord(savedGameBytes.clone(), 0, (savedGameBytes.len() as i32).clone());
            self.name = name.clone();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).SAVE.clone(), crate::java_into_object(e.clone()));
            }
        }
        { // finally
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if recordStore.is_some() {
                    PreLogUtil::put(String::from("Closing RecordStore"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).SAVE.clone());
                    jcall!(recordStore).closeRecordStore();
                }
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).SAVE.clone(), crate::java_into_object(e.clone()));
                }
            }
        }
    }
}

