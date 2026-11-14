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
use crate::javax::microedition::rms::RecordEnumeration::RecordEnumeration;
use crate::javax::microedition::rms::RecordStore::RecordStore;
use crate::org::allbinary::game::configuration::persistance::BasicPersitance::BasicPersitance;
use crate::org::allbinary::game::configuration::persistance::NullRecordComparator::NullRecordComparator;
use crate::org::allbinary::game::configuration::persistance::NullRecordFilter::NullRecordFilter;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::HashtableUtil::HashtableUtil;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::java::lang::Exception::*;
use super::Input::Input;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//GameInputNoPersistanceLibrary
pub struct InputPersistance {
    pub base: BasicPersitance,
    pub hashtableUtil: Option<Rc<RefCell<HashtableUtil>>>,
}


impl InputPersistance {
    

    pub fn new_struct(mut name: String) -> Self {
        let mut this = Self {
            base: BasicPersitance::new_struct(name.clone()),
            hashtableUtil: None,
        };
        this.hashtableUtil = HashtableUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name))))
    }
    

    // throws Exception
    pub fn loadAll(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        let mut recordStore: Option<Rc<RefCell<dyn crate::javax::microedition::rms::RecordStore::RecordStoreDyn>>> = RecordStore::openRecordStore(self.getRecordId(abeClientInformation.clone()), true);
        let mut recordEnum: Option<Rc<RefCell<dyn RecordEnumeration>>> = jcall!(recordStore).enumerateRecords(unsafe { crate::org::allbinary::game::configuration::persistance::NullRecordFilter::NullRecordFilter_NULL_RECORD_FILTER.clone() }.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::rms::RecordFilter::RecordFilter>>), crate::org::allbinary::game::configuration::persistance::NullRecordComparator::NullRecordComparator_NULL_RECORD_COMPARATOR.with(Clone::clone).clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::rms::RecordComparator::RecordComparator>>), true);
        let mut hashtable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> = None;
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
        let mut id: i32 = 0;
        while jcall!(recordEnum).hasNextElement() {
            id = { let java_value = (jcall!(recordEnum).nextRecordId()).clone(); java_value };
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).append({ let java_field = jcall!(self.persistanceStrings).LOADING_ID.clone(); java_field }.clone()).appendint(id.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.persistanceStrings).LOAD_ALL.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            hashtable = { let java_value = (jcall!(StdUtil::getInstance()).createHashtable()).clone(); java_value };
            jcall!(self.valueList).add(crate::java_into_object(hashtable.clone()));
            { let java_receiver = self.idList.clone(); let java_arguments = (crate::java_into_object(jcall!(smallIntegerSingletonFactory).getAt(id.clone())),); jcall!(java_receiver).add(java_arguments.0); };
        }
        jcall!(recordStore).closeRecordStore();
    }
    

    // throws Exception
    pub fn save(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut hashtable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>>) {
        PreLogUtil::put(jcall!(StringMaker::new()).append({ let java_field = jcall!(self.persistanceStrings).NOT_SAVING.clone(); java_field }.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(hashtable.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).SAVE.clone(); java_field }.clone());
        let mut recordStore: Option<Rc<RefCell<dyn crate::javax::microedition::rms::RecordStore::RecordStoreDyn>>> = RecordStore::openRecordStore(self.getRecordId(abeClientInformation.clone()), true);
        let mut gameActionInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = None;
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        let mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = None;
        let mut inputObjectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(self.hashtableUtil).getKeysAsArray(hashtable.clone());
        let mut size: i32 = (inputObjectArray.len() as i32).clone();
        for index in (0..size) {
            gameActionInput = { let java_object = (inputObjectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>>().expect("ClassCastException").clone()) };
            list = { let java_value = ({ let java_object = (jcall!(hashtable).get(inputObjectArray[(index) as usize].clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
            for index2 in (0..{ let java_bound = jcall!(list).size(); java_bound }) {
                input = { let java_value = ({ let java_object = (jcall!(list).get(index2.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
            }
        }
        jcall!(recordStore).closeRecordStore();
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


