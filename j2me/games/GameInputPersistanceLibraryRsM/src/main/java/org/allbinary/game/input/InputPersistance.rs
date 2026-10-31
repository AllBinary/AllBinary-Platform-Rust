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
// use crate::java::io::ByteArrayInputStream::ByteArrayInputStream;
// use crate::java::io::ByteArrayOutputStream::ByteArrayOutputStream;
// use crate::java::io::DataInputStream::DataInputStream;
// use crate::java::io::DataOutputStream::DataOutputStream;
// use crate::javax::microedition::rms::RecordEnumeration::RecordEnumeration;
// use crate::javax::microedition::rms::RecordStore::RecordStore;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::game::configuration::persistance::BasicPersitance::BasicPersitance;
use crate::org::allbinary::game::configuration::persistance::NullRecordComparator::NullRecordComparator;
use crate::org::allbinary::game::configuration::persistance::NullRecordFilter::NullRecordFilter;
use crate::org::allbinary::game::configuration::persistance::NullRecordStore::NullRecordStore;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::HashtableUtil::HashtableUtil;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::Input::Input;
use super::GameKeyMappingFactory::GameKeyMappingFactory;
use super::InputFactory::InputFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//GameInputPersistanceLibrary
pub struct InputPersistance {
    pub base: BasicPersitance,
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
    pub hashtableUtil: Option<Rc<RefCell<HashtableUtil>>>,
}


impl InputPersistance {
    

    pub fn new_struct(mut name: String) -> Self {
        let mut this = Self {
            base: BasicPersitance::new_struct(name.clone()),
            tsUtil: None,
            hashtableUtil: None,
        };
        this.tsUtil = TsUtil::getInstance();
        this.hashtableUtil = HashtableUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name))))
    }
    

    // throws Exception
    pub fn loadAll(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        let mut recordStore: Option<Rc<RefCell<RecordStore>>> = unsafe { crate::org::allbinary::game::configuration::persistance::NullRecordStore::NullRecordStore_NULL_RECORD_STORE.clone() }.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            recordStore = RecordStore::openRecordStore(self.getRecordId(abeClientInformation.clone()), true);
            let mut recordEnum: Option<Rc<RefCell<RecordEnumeration>>> = jcall!(recordStore).enumerateRecords(unsafe { crate::org::allbinary::game::configuration::persistance::NullRecordFilter::NullRecordFilter_NULL_RECORD_FILTER.clone() }.clone(), crate::org::allbinary::game::configuration::persistance::NullRecordComparator::NullRecordComparator_NULL_RECORD_COMPARATOR.with(Clone::clone).clone(), true);
            let mut gameActionInputId: i64 = 0_i64;
            let mut inputId: i64 = 0_i64;
            let mut gameActionInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = None;
            let mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = None;
            let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = None;
            let mut gameKeyFactory: Option<Rc<RefCell<GameKeyMappingFactory>>> = GameKeyMappingFactory::getInstance();
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            let mut inputFactory: Option<Rc<RefCell<InputFactory>>> = InputFactory::getInstance();
            let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
            let mut stdUtil: Option<Rc<RefCell<StdUtil>>> = StdUtil::getInstance();
            let mut recordAsBytes: Vec<i8> = Vec::new();
            let mut byteArrayInputStream: Option<Rc<RefCell<ByteArrayInputStream>>> = None;
            let mut inputStream: Option<Rc<RefCell<DataInputStream>>> = None;
            while jcall!(recordEnum).hasNextElement() {
                let mut id: i32 = jcall!(recordEnum).nextRecordId();
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).append(jcall!(self.persistanceStrings).LOADING_ID.clone()).appendint(id.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.persistanceStrings).LOAD_ALL.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                recordAsBytes = jcall!(self.tsUtil).getRecord(crate::java_into_object(recordStore.clone()), id.clone());
                if recordAsBytes.is_some() {
                    byteArrayInputStream = ByteArrayInputStream::new(recordAsBytes.clone());
                    inputStream = DataInputStream::new(byteArrayInputStream.clone());
                    hashtable = jcall!(stdUtil).createHashtable();
                    let mut value: i32 = 0;
                    while jcall!(inputStream).available() > 0 {
                        let mut gameActionInputIdAsString: String = jcall!(inputStream).readUTF();
                        value = gameActionInputIdAsString.clone().parse::<i32>().unwrap();
                        gameActionInputId = ((value) as i64);
                        jcall!(inputStream).readUTF();
                        value = jcall!(inputStream).readUTF().parse::<i32>().unwrap();
                        inputId = ((value) as i64);
                        gameActionInput = jcall!(gameKeyFactory).getGameKey(((gameActionInputId) as i32));
                        input = jcall!(inputFactory).getInstanceById(((inputId) as i32));
                        if match (&input, &jcall!(inputFactory).NO_INPUT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || gameActionInput.is_none() {
                            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                            if match (&input, &jcall!(inputFactory).NO_INPUT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                                jcall!(stringBuffer).append(jcall!(self.persistanceStrings).ERROR_LOADING_ID.clone());
                                jcall!(stringBuffer).appendlong(inputId.clone());
                                jcall!(stringBuffer).append(jcall!(self.persistanceStrings).GAME_ACTION_INPUT.clone());
                                jcall!(stringBuffer).appendlong(gameActionInputId.clone());
                                PreLogUtil::put(jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.persistanceStrings).LOAD_ALL.clone());
                            }
                            if gameActionInput.is_none() {
                                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                                jcall!(stringBuffer).append(jcall!(self.persistanceStrings).ERROR_LOADING.clone());
                                jcall!(stringBuffer).appendlong(gameActionInputId.clone());
                                jcall!(stringBuffer).append(jcall!(self.persistanceStrings).ID.clone());
                                jcall!(stringBuffer).appendlong(inputId.clone());
                                PreLogUtil::put(jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.persistanceStrings).LOAD_ALL.clone());
                            }
                        } else {
                        }
                        jcall!(hashtable).put(crate::java_into_object(input.clone()), crate::java_into_object(gameActionInput.clone()));
                    }
                    jcall!(self.valueList).add(crate::java_into_object(hashtable.clone()));
                    { let java_receiver = self.idList.clone(); let java_arguments = (crate::java_into_object(jcall!(smallIntegerSingletonFactory).getAt(id.clone())),); jcall!(java_receiver).add(java_arguments.0); };
                } else {
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
        let mut recordStore: Option<Rc<RefCell<RecordStore>>> = unsafe { crate::org::allbinary::game::configuration::persistance::NullRecordStore::NullRecordStore_NULL_RECORD_STORE.clone() }.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            PreLogUtil::put(jcall!(stringBuffer).append(jcall!(self.persistanceStrings).SAVING.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(hashtable.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).SAVE.clone());
            recordStore = RecordStore::openRecordStore(self.getRecordId(abeClientInformation.clone()), true);
            let mut byteArrayOutputStream: Vec<u8> = Vec::new();
            let mut outputStream: Option<Rc<RefCell<DataOutputStream>>> = DataOutputStream::new(byteArrayOutputStream.clone());
            let mut gameActionInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = None;
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
            let mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = None;
            let mut savedGameBytes: Vec<i8> = Vec::new();
            let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
            let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
            let mut inputObjectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(self.hashtableUtil).getKeysAsArray(hashtable.clone());
            let mut size: i32 = (inputObjectArray.len() as i32).clone();
            for index in (0..size) {
                gameActionInput = (inputObjectArray[(index) as usize]).clone() /* as Input */;
                list = { let java_object = (jcall!(hashtable).get(crate::java_into_object((inputObjectArray[(index) as usize]).clone() /* as Any */))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
                for index2 in (0..jcall!(list).size()) {
                    let mut gameActionInputIdAsString: String = jcall!(jcall!(smallIntegerSingletonFactory).getAt(jcall!(gameActionInput).getId())).toString();
                    jcall!(outputStream).writeUTF(gameActionInputIdAsString.clone());
                    jcall!(outputStream).writeUTF(jcall!(commonSeps).EQUALS.clone());
                    input = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index2) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>>().expect("ClassCastException").clone()) };
                    let mut inputIdAsString: String = jcall!(jcall!(smallIntegerSingletonFactory).getAt(jcall!(input).getId())).toString();
                    jcall!(outputStream).writeUTF(inputIdAsString.clone());
                }
            }
            savedGameBytes = jcall!(byteArrayOutputStream).toByteArray();
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
}
impl std::ops::Deref for InputPersistance {
    type Target = BasicPersitance;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InputPersistance {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::configuration::persistance::BasicPersitance::BasicPersitanceDyn for InputPersistance {
    fn as_BasicPersitance(&self) -> &crate::org::allbinary::game::configuration::persistance::BasicPersitance::BasicPersitance { &self.base }
    fn as_BasicPersitance_mut(&mut self) -> &mut crate::org::allbinary::game::configuration::persistance::BasicPersitance::BasicPersitance { &mut self.base }
}


