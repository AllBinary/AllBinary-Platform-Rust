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
// package org.allbinary.game.input.mapping
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Enumeration;
use super::*;
// use crate::java::util::Enumeration::Enumeration;
use crate::org::allbinary::game::configuration::persistance::GamePersistanceStrings::GamePersistanceStrings;
use crate::org::allbinary::game::input::Input::Input;
use crate::org::allbinary::game::input::InputPersistance::InputPersistance;
use crate::org::allbinary::game::input::mapping::event::InputMappingEvent::InputMappingEvent;
use crate::org::allbinary::game::input::mapping::event::InputMappingEventListenerInterface::InputMappingEventListenerInterface;
use crate::org::allbinary::game::input::mapping::event::NullInputMappingEventListener::NullInputMappingEventListener;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::EnumerationUtil::EnumerationUtil;
use super::InputToGameKeyMapping::InputToGameKeyMapping;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//GameInputPersistanceLibrary
pub struct PersistentInputMapping {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub enumerationUtil: Option<Rc<RefCell<EnumerationUtil>>>,
    pub inputMapping: Option<Rc<RefCell<InputToGameKeyMapping>>>,
    pub inputMappingEventListenerInterface: Option<Rc<RefCell<dyn InputMappingEventListenerInterface>>>,
    pub inputPersistance: Option<Rc<RefCell<InputPersistance>>>,
    pub inputMappingEvent: Option<Rc<RefCell<InputMappingEvent>>>,
}

pub static mut PersistentInputMapping_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl PersistentInputMapping {
    

    pub fn getNullInstance() -> Option<Rc<RefCell<dyn PersistentInputMappingDyn>>> {
        if match (&unsafe { PersistentInputMapping_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *PersistentInputMapping_instance = PersistentInputMapping::new(jcall!(GamePersistanceStrings::getInstance()).SAVED_INPUT_CONFIGURATION_RECORD_ID.clone()).map(|reference| reference as Rc<RefCell<dyn PersistentInputMappingDyn>>) };
        }
        return (unsafe { PersistentInputMapping_instance.clone() }).clone() /* as PersistentInputMapping */;
    }
    

    pub fn new_struct(mut name: String) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            enumerationUtil: None,
            inputMapping: None,
            inputMappingEventListenerInterface: None,
            inputPersistance: None,
            inputMappingEvent: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.enumerationUtil = EnumerationUtil::getInstance();
        this.inputMapping = InputToGameKeyMapping::new();
        this.inputMappingEventListenerInterface = crate::org::allbinary::game::input::mapping::event::NullInputMappingEventListener::NullInputMappingEventListener_NULL_INPUT_MAPPING_EVENT_LISTENER.with(Clone::clone).clone();
        this.inputMappingEvent = InputMappingEvent::new(this);
        this.inputPersistance = InputPersistance::new(name.clone());
        this
    }

    pub fn new(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name))))
    }
    

    pub fn getTotalMapped(&mut self) -> i32 {
        return jcall!(jcall!(self.getInputMapping()).getHashtable()).size();
    }
    

    pub fn isDefaultNew(&mut self) -> bool {
        return false;
    }
    

    pub fn getDefault(&mut self) -> Option<Rc<RefCell<InputToGameKeyMapping>>> {
        return InputToGameKeyMapping::getNullInstance();
    }
    

    // throws Exception
    pub fn setDefault(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        jcall!(self.getInputMapping()).removeAll();
        jcall!(self.getInputMapping()).addMapping(self.getDefault());
        self.save(abeClientInformation.clone());
    }
    

    // throws Exception
    pub fn update(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        jcall!(self.inputPersistance).deleteAll(abeClientInformation.clone());
        self.save(abeClientInformation.clone());
    }
    

    // throws Exception
    pub fn save(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        { let java_receiver = self.inputPersistance.clone(); let java_arguments = (abeClientInformation.clone(), jcall!(self.getInputMapping()).getHashtable(),); jcall!(java_receiver).save(java_arguments.0, java_arguments.1); };
        { let java_receiver = self.inputMappingEvent.clone(); let java_arguments = (self.getInputMapping(),); jcall!(java_receiver).setInputToGameKeyMapping(java_arguments.0); };
        if self.getInputMappingEventListenerInterface().is_some() {
            jcall!(self.getInputMappingEventListenerInterface()).onInputMappingEvent(self.inputMappingEvent.clone());
        }
    }
    

    // throws Exception
    pub fn init(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.inputPersistance).loadAll(abeClientInformation.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::putOE(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(), e.clone());
            jcall!(self.inputPersistance).deleteRecoreStore(abeClientInformation.clone());
            self.setDefault(abeClientInformation.clone());
            jcall!(self.inputPersistance).loadAll(abeClientInformation.clone());
            }
        }
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.inputPersistance).getList();
        let mut totalMappedTo: i32 = 0;
        let mut size: i32 = jcall!(list).size();
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = None;
        let mut enumeration: Option<Rc<RefCell<dyn Enumeration>>> = None;
        let mut mappedToInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = None;
        let mut gameActionInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = None;
        for index in (0..size) {
            hashtable = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<ABHashtable>>>>().expect("ClassCastException").clone()) };
            enumeration = jcall!(hashtable).keys();
            while jcall!(self.enumerationUtil).hasMoreElements(enumeration.clone()) {
                mappedToInput = { let java_object = (jcall!(self.enumerationUtil).nextElement(enumeration.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>>().expect("ClassCastException").clone()) };
                gameActionInput = { let java_object = (jcall!(hashtable).get(crate::java_into_object(mappedToInput.clone()))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>>().expect("ClassCastException").clone()) };
                totalMappedTo += 1;
                jcall!(self.getInputMapping()).add(gameActionInput.clone(), mappedToInput.clone());
            }
        }
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("End - Total Loaded Keys Mapped: "));
        jcall!(stringBuffer).appendint(self.getTotalMapped());
        jcall!(stringBuffer).append(String::from(" to: "));
        jcall!(stringBuffer).appendint(totalMappedTo.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn setInputMappingEventListenerInterface(&mut self, mut inputMappingEventListenerInterface: Option<Rc<RefCell<dyn InputMappingEventListenerInterface>>>) {
        self.inputMappingEventListenerInterface = inputMappingEventListenerInterface.clone();
    }
    

    pub fn getInputMappingEventListenerInterface(&mut self) -> Option<Rc<RefCell<dyn InputMappingEventListenerInterface>>> {
        return self.inputMappingEventListenerInterface.clone();
    }
    

    pub fn getInputMapping(&mut self) -> Option<Rc<RefCell<InputToGameKeyMapping>>> {
        return self.inputMapping.clone();
    }
    

    // throws Exception
    pub fn isDelete(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    // throws Exception
    pub fn isSystemInput(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
}
pub trait PersistentInputMappingDyn {
    fn as_PersistentInputMapping(&self) -> &PersistentInputMapping;
    fn as_PersistentInputMapping_mut(&mut self) -> &mut PersistentInputMapping;
    fn getTotalMapped(&mut self) -> i32 { PersistentInputMapping::getTotalMapped(self.as_PersistentInputMapping_mut()) }
    fn isDefaultNew(&mut self) -> bool { PersistentInputMapping::isDefaultNew(self.as_PersistentInputMapping_mut()) }
    fn getDefault(&mut self) -> Option<Rc<RefCell<InputToGameKeyMapping>>> { PersistentInputMapping::getDefault(self.as_PersistentInputMapping_mut()) }
    fn setDefault(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) { PersistentInputMapping::setDefault(self.as_PersistentInputMapping_mut(), abeClientInformation) }
    fn update(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) { PersistentInputMapping::update(self.as_PersistentInputMapping_mut(), abeClientInformation) }
    fn save(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) { PersistentInputMapping::save(self.as_PersistentInputMapping_mut(), abeClientInformation) }
    fn init(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) { PersistentInputMapping::init(self.as_PersistentInputMapping_mut(), abeClientInformation) }
    fn setInputMappingEventListenerInterface(&mut self, mut inputMappingEventListenerInterface: Option<Rc<RefCell<dyn InputMappingEventListenerInterface>>>) { PersistentInputMapping::setInputMappingEventListenerInterface(self.as_PersistentInputMapping_mut(), inputMappingEventListenerInterface) }
    fn getInputMappingEventListenerInterface(&mut self) -> Option<Rc<RefCell<dyn InputMappingEventListenerInterface>>> { PersistentInputMapping::getInputMappingEventListenerInterface(self.as_PersistentInputMapping_mut()) }
    fn getInputMapping(&mut self) -> Option<Rc<RefCell<InputToGameKeyMapping>>> { PersistentInputMapping::getInputMapping(self.as_PersistentInputMapping_mut()) }
    fn isDelete(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool { PersistentInputMapping::isDelete(self.as_PersistentInputMapping_mut(), input) }
    fn isSystemInput(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool { PersistentInputMapping::isSystemInput(self.as_PersistentInputMapping_mut(), input) }
}
impl PersistentInputMappingDyn for PersistentInputMapping {
    fn as_PersistentInputMapping(&self) -> &PersistentInputMapping { self }
    fn as_PersistentInputMapping_mut(&mut self) -> &mut PersistentInputMapping { self }
}


