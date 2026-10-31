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
// package org.allbinary.input.automation.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::File::File;
// use crate::java::io::FileInputStream::FileInputStream;
// use crate::java::util::List::List;
// use crate::javax::xml::bind::JAXBContext::JAXBContext;
// use crate::javax::xml::bind::JAXBElement::JAXBElement;
// use crate::javax::xml::bind::Unmarshaller::Unmarshaller;
// use crate::javax::xml::transform::stream::StreamSource::StreamSource;
use crate::org::allbinary::input::automation::module::configuration::InputAutomationModuleConfiguration::InputAutomationModuleConfiguration;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::InputAutomationConfiguration::InputAutomationConfiguration;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationConfigurationFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub inputAutomationConfiguration: Option<Rc<RefCell<InputAutomationConfiguration>>>,
}

thread_local! { pub static InputAutomationConfigurationFactory_instance: Option<Rc<RefCell<InputAutomationConfigurationFactory>>> = InputAutomationConfigurationFactory::new(); }

impl InputAutomationConfigurationFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<InputAutomationConfigurationFactory>>> {
        return InputAutomationConfigurationFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            inputAutomationConfiguration: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.inputAutomationConfiguration = None;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut INPUT_AUTOMATION_CONFIGURATION: String = String::from("InputAutomationConfiguration");
        let mut file: Option<Rc<RefCell<File>>> = InputAutomationConfiguration::getFile();
        if jcall!(file).isFile() {
            jcall!(logUtil).putF(String::from("LoadingConfiguration"), crate::java_into_object(INPUT_AUTOMATION_CONFIGURATION.clone()), jcall!(commonStrings).INIT.clone());
            let mut jaxbContext: Option<Rc<RefCell<JAXBContext>>> = JAXBContext::newInstance(std::any::type_name::<InputAutomationConfiguration>());
            let mut unmarshaller: Option<Rc<RefCell<Unmarshaller>>> = jcall!(jaxbContext).createUnmarshaller();
            let mut root: Option<Rc<RefCell<JAXBElement<Option<Rc<RefCell<InputAutomationConfiguration>>>>>>> = jcall!(unmarshaller).unmarshal(StreamSource::new(FileInputStream::new(file.clone())), std::any::type_name::<InputAutomationConfiguration>());
            self.inputAutomationConfiguration = (jcall!(root).getValue()).clone() /* as InputAutomationConfiguration */;
            let mut inputAutomationModuleConfigurationList: Option<Rc<RefCell<dyn List<Option<Rc<RefCell<InputAutomationModuleConfiguration>>>>>>> = jcall!(self.inputAutomationConfiguration).getInputAutomationModuleConfigurationList();
            jcall!(logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", String::from("isInstalled: "), jcall!(self.inputAutomationConfiguration).isInstalled()), String::from(" inputAutomationModuleConfigurationList: ")), inputAutomationModuleConfigurationList), crate::java_into_object(INPUT_AUTOMATION_CONFIGURATION.clone()), jcall!(commonStrings).INIT.clone());
            if inputAutomationModuleConfigurationList.is_some() {
                let mut size: i32 = jcall!(inputAutomationModuleConfigurationList).size();
                let mut inputAutomationModuleConfiguration: Option<Rc<RefCell<InputAutomationModuleConfiguration>>> = None;
                for index in (0..size) {
                    inputAutomationModuleConfiguration = jcall!(inputAutomationModuleConfigurationList).get(index.clone());
                    jcall!(inputAutomationModuleConfiguration).init(abeClientInformation.clone());
                }
            }
            jcall!(logUtil).putF(String::from("LoadedConfiguration"), crate::java_into_object(INPUT_AUTOMATION_CONFIGURATION.clone()), jcall!(commonStrings).INIT.clone());
        } else {
            jcall!(logUtil).putF(String::from("New Configuration"), crate::java_into_object(INPUT_AUTOMATION_CONFIGURATION.clone()), jcall!(commonStrings).INIT.clone());
            self.inputAutomationConfiguration = InputAutomationConfiguration::new();
        }
    }
}

