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
// package org.allbinary.input.automation.module
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::util::Collection::Collection;
use crate::java::util::HashMap::HashMap;
// use crate::java::util::Iterator::Iterator;
// use crate::javax::help::HelpSet::HelpSet;
// use crate::javax::help::event::HelpSetEvent::HelpSetEvent;
// use crate::javax::help::event::HelpSetListener::HelpSetListener;
// use crate::javax::swing::ListModel::ListModel;
use crate::org::allbinary::input::automation::module::configuration::InputAutomationModuleConfiguration::InputAutomationModuleConfiguration;
use crate::org::allbinary::input::automation::module::configuration::InputAutomationModuleConfigurations::InputAutomationModuleConfigurations;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::java::help::JavaHelpSetNotifier::JavaHelpSetNotifier;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
use super::DefaultListModelHelper::DefaultListModelHelper;
use super::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationModuleFactoryFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>,
    pub defaultListModelHelper: Option<Rc<RefCell<DefaultListModelHelper>>>,
    pub helpSetListenerInterface: Option<Rc<RefCell<HelpSetListener>>>,
}


impl InputAutomationModuleFactoryFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut inputAutomationModuleConfigurations: Option<Rc<RefCell<InputAutomationModuleConfigurations>>>, mut helpSetListenerInterface: Option<Rc<RefCell<HelpSetListener>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            hashMap: None,
            defaultListModelHelper: None,
            helpSetListenerInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.helpSetListenerInterface = helpSetListenerInterface.clone();
        this.defaultListModelHelper = DefaultListModelHelper::new();
        this.hashMap = { let java_value = (jcall!(StdUtil::getInstance()).createHashMap()).clone(); java_value };
        let mut collection: Option<Rc<RefCell<dyn Collection>>> = jcall!(jcall!(inputAutomationModuleConfigurations).getHashMap()).values();
        let mut iterator: Option<Rc<RefCell<dyn Iterator>>> = jcall!(collection).iterator();
        let mut inputAutomationModuleConfiguration: Option<Rc<RefCell<InputAutomationModuleConfiguration>>> = None;
        while jcall!(iterator).hasNext() {
            inputAutomationModuleConfiguration = { let java_value = ((jcall!(iterator).next()).clone()).clone(); java_value };
            this.add(inputAutomationModuleConfiguration.clone());
        }
        jcall!(this.defaultListModelHelper).initDefaultModelList();
        { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Loaded "), jcall!(this.hashMap).size()), String::from("/")), jcall!(jcall!(this.defaultListModelHelper).getListModel()).getSize()), String::from(" Input Automation Modules")), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Contructor"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        this
    }

    pub fn new(mut inputAutomationModuleConfigurations: Option<Rc<RefCell<InputAutomationModuleConfigurations>>>, mut helpSetListenerInterface: Option<Rc<RefCell<HelpSetListener>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(inputAutomationModuleConfigurations, helpSetListenerInterface))))
    }
    

    pub fn add(&mut self, mut inputAutomationModuleConfiguration: Option<Rc<RefCell<InputAutomationModuleConfiguration>>>) {
        let mut inputAutomationModuleInterface: Option<Rc<RefCell<dyn crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterfaceDyn>>> = jcall!(inputAutomationModuleConfiguration).getInputAutomationModuleInterface();
        { let java_receiver = self.hashMap.clone(); let java_arguments = (crate::java_into_object(jcall!(inputAutomationModuleConfiguration).getName()), crate::java_into_object(inputAutomationModuleInterface.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
        { let java_receiver = self.defaultListModelHelper.clone(); let java_arguments = (jcall!(inputAutomationModuleConfiguration).getName(),); jcall!(java_receiver).add(java_arguments.0); };
        let mut helpSet: Option<Rc<RefCell<HelpSet>>> = jcall!(inputAutomationModuleInterface).getHelpSet();
        if !JavaHelpSetNotifier::isNotified(helpSet.clone()) {
            let mut helpSetEvent: Option<Rc<RefCell<HelpSetEvent>>> = HelpSetEvent::new(None /* Java this: a &mut self method has no Rc of itself */, helpSet.clone(), unsafe { crate::javax::help::event::HelpSetEvent::HelpSetEvent_HELPSET_ADDED.clone() }.clone());
            jcall!(self.helpSetListenerInterface).helpSetAdded(helpSetEvent.clone());
        }
    }
    

    pub fn getListModel(&mut self) -> Option<Rc<RefCell<ListModel>>> {
        return (jcall!(self.defaultListModelHelper).getListModel()).clone() /* as ListModel */;
    }
    

    pub fn getInstance(&mut self, mut moduleName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterfaceDyn>>> {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        jcall!(self.logUtil).putF(format!("{}{}", String::from("Getting Module: "), moduleName), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).GET_INSTANCE.clone(); java_field }.clone());
        let mut inputAutomationModuleFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterfaceDyn>>> = { let java_object = (jcall!(self.hashMap).get(crate::java_into_object(moduleName.clone()))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::input::automation::module::InputAutomationModuleFactoryInterface::InputAutomationModuleFactoryInterfaceDyn>>>>().expect("ClassCastException").clone()) };
        if inputAutomationModuleFactoryInterface.is_none() {
            jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", String::from("Module: "), moduleName), String::from(" was null")), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).GET_INSTANCE.clone(); java_field }.clone());
        }
        return inputAutomationModuleFactoryInterface.clone();
    }
}

