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
// use crate::java::io::ByteArrayInputStream::ByteArrayInputStream;
// use crate::java::io::ByteArrayOutputStream::ByteArrayOutputStream;
// use crate::java::io::DataInputStream::DataInputStream;
// use crate::java::io::DataOutputStream::DataOutputStream;
// use crate::javax::microedition::rms::RecordEnumeration::RecordEnumeration;
// use crate::javax::microedition::rms::RecordStore::RecordStore;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::HashtableUtil::HashtableUtil;
use super::BasicPersitance::BasicPersitance;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct KeyValuePersistance {
    pub base: BasicPersitance,
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
}


impl KeyValuePersistance {
    

    pub fn new_struct(mut recordId: String) -> Self {
        let mut this = Self {
            base: BasicPersitance::new_struct(recordId.clone()),
            tsUtil: None,
        };
        this.tsUtil = TsUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut recordId: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(recordId))))
    }
    

    // throws Exception
    pub fn loadAll(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        self.loadAllSize(abeClientInformation.clone(), 1);
    }
    

    // throws Exception
    pub fn loadAllSize(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut size: i32) {
        let mut recordStore: Option<Rc<RefCell<RecordStore>>> = unsafe { NullRecordStore_NULL_RECORD_STORE.clone() }.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            recordStore = RecordStore::openRecordStore(self.getRecordId(abeClientInformation.clone()), true);
            let mut recordEnum: Option<Rc<RefCell<RecordEnumeration>>> = jcall!(recordStore).enumerateRecords(unsafe { NullRecordFilter_NULL_RECORD_FILTER.clone() }.clone(), NullRecordComparator_NULL_RECORD_COMPARATOR.with(Clone::clone).clone(), true);
            let mut hashtable: Option<Rc<RefCell<ABHashtable<String, String>>>> = None;
            let mut name: String = String::new();
            let mut value: String = String::new();
            let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            let mut recordAsBytes: Vec<i8> = Vec::new();
            let mut byteArrayInputStream: Option<Rc<RefCell<ByteArrayInputStream>>> = None;
            let mut inputStream: Option<Rc<RefCell<DataInputStream>>> = None;
            while jcall!(recordEnum).hasNextElement() {
                let mut id: i32 = jcall!(recordEnum).nextRecordId();
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).append(jcall!(self.persistanceStrings).LOADING_ID.clone()).appendint(id.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.persistanceStrings).LOAD_ALL.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                recordAsBytes = jcall!(self.tsUtil).getRecord(recordStore.clone(), id.clone());
                if recordAsBytes.is_some() {
                    byteArrayInputStream = ByteArrayInputStream::new(recordAsBytes.clone());
                    inputStream = DataInputStream::new(byteArrayInputStream.clone());
                    hashtable = ABHashtable::new();
                    for index in (0..size) {
                        name = jcall!(inputStream).readUTF();
                        jcall!(inputStream).readUTF();
                        value = jcall!(inputStream).readUTF();
                        jcall!(hashtable).put(crate::java_into_object(name.clone()), crate::java_into_object(value.clone()));
                    }
                    jcall!(self.valueList).add(crate::java_into_object(hashtable.clone()));
                    { let java_receiver = self.idList.clone(); let java_arguments = (crate::java_into_object(jcall!(smallIntegerSingletonFactory).getAt(id.clone())),); jcall!(java_receiver).add(java_arguments.0); };
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
        { // finally
            if recordStore.is_some() {
                PreLogUtil::put(jcall!(self.persistanceStrings).CLOSING_RECORDSTORE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.persistanceStrings).LOAD_ALL.clone());
                jcall!(recordStore).closeRecordStore();
            }
        }
    }
    

    // throws Exception
    pub fn save(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) {
        let mut recordStore: Option<Rc<RefCell<RecordStore>>> = unsafe { NullRecordStore_NULL_RECORD_STORE.clone() }.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.persistanceStrings).SAVING.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(hashtable.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).SAVE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            recordStore = RecordStore::openRecordStore(self.getRecordId(abeClientInformation.clone()), true);
            let mut byteArrayOutputStream: Vec<u8> = Vec::new();
            let mut outputStream: Option<Rc<RefCell<DataOutputStream>>> = DataOutputStream::new(byteArrayOutputStream.clone());
            let mut value: String = String::new();
            let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
            let mut objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(HashtableUtil::getInstance()).getKeysAsArray(hashtable.clone());
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = None;
            let mut size: i32 = (objectArray.len() as i32).clone();
            for index in (0..size) {
                jcall!(outputStream).writeUTF((objectArray[(index) as usize]).clone() /* as String */);
                jcall!(outputStream).writeUTF(jcall!(commonSeps).EQUALS.clone());
                anyType = (objectArray[(index) as usize]).clone() /* as Any */;
                value = (jcall!(hashtable).get(anyType.clone())).clone() /* as String */;
                jcall!(outputStream).writeUTF(value.clone());
            }
            let mut tsUtil: Option<Rc<RefCell<TsUtil>>> = TsUtil::getInstance();
            let mut savedGameBytes: Vec<i8> = jcall!(tsUtil).getByteArray(String::from_utf8_lossy(&byteArrayOutputStream).into_owned());
            jcall!(recordStore).addRecord(savedGameBytes.clone(), 0, (savedGameBytes.len() as i32).clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
        { // finally
            if recordStore.is_some() {
                PreLogUtil::put(jcall!(self.persistanceStrings).CLOSING_RECORDSTORE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).SAVE.clone());
                jcall!(recordStore).closeRecordStore();
            }
        }
    }
    

    pub fn get(&mut self, mut index: i32) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = { let java_object = (jcall!(self.valueList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<ABHashtable>>>>().expect("ClassCastException").clone()) };
        return hashtable.clone();
    }
}
impl std::ops::Deref for KeyValuePersistance {
    type Target = BasicPersitance;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for KeyValuePersistance {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait KeyValuePersistanceDyn: crate::org::allbinary::game::configuration::persistance::BasicPersitance::BasicPersitanceDyn {
    fn as_KeyValuePersistance(&self) -> &KeyValuePersistance;
    fn as_KeyValuePersistance_mut(&mut self) -> &mut KeyValuePersistance;
    fn loadAll(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) { KeyValuePersistance::loadAll(self.as_KeyValuePersistance_mut(), abeClientInformation) }
    fn loadAllSize(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut size: i32) { KeyValuePersistance::loadAllSize(self.as_KeyValuePersistance_mut(), abeClientInformation, size) }
    fn save(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) { KeyValuePersistance::save(self.as_KeyValuePersistance_mut(), abeClientInformation, hashtable) }
    fn get(&mut self, mut index: i32) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { KeyValuePersistance::get(self.as_KeyValuePersistance_mut(), index) }
}
impl KeyValuePersistanceDyn for KeyValuePersistance {
    fn as_KeyValuePersistance(&self) -> &KeyValuePersistance { self }
    fn as_KeyValuePersistance_mut(&mut self) -> &mut KeyValuePersistance { self }
}

impl crate::org::allbinary::game::configuration::persistance::BasicPersitance::BasicPersitanceDyn for KeyValuePersistance {
    fn as_BasicPersitance(&self) -> &crate::org::allbinary::game::configuration::persistance::BasicPersitance::BasicPersitance { &self.base }
    fn as_BasicPersitance_mut(&mut self) -> &mut crate::org::allbinary::game::configuration::persistance::BasicPersitance::BasicPersitance { &mut self.base }
}


