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
// use crate::java::util::List::List;
// use crate::javax::xml::bind::JAXBContext::JAXBContext;
// use crate::javax::xml::bind::Marshaller::Marshaller;
// use crate::javax::xml::bind::annotation::XmlAccessType::XmlAccessType;
// use crate::javax::xml::bind::annotation::XmlAccessorType::XmlAccessorType;
// use crate::javax::xml::bind::annotation::XmlElement::XmlElement;
// use crate::javax::xml::bind::annotation::XmlRootElement::XmlRootElement;
// use crate::javax::xml::bind::annotation::XmlType::XmlType;
use crate::org::allbinary::data::tree::dom::document::DomDocumentFileHelper::DomDocumentFileHelper;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::input::automation::module::configuration::InputAutomationModuleConfiguration::InputAutomationModuleConfiguration;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::file::FileUtil::FileUtil;
use crate::org::allbinary::logic::io::file::FileWrapperUtil::FileWrapperUtil;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::w3c::dom::Document::Document;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//name="inputAutomationConfiguration", 
pub struct InputAutomationConfiguration {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub installed: bool,
    pub inputAutomationModuleConfigurationList: Option<Rc<RefCell<dyn List<Option<Rc<RefCell<InputAutomationModuleConfiguration>>>>>>>,
}

thread_local! { pub static InputAutomationConfiguration_FILENAME: String = String::from("InputAutomationConfig.xml"); }

impl InputAutomationConfiguration {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            installed: false,
            inputAutomationModuleConfigurationList: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getFile() -> Option<Rc<RefCell<File>>> {
        return File::new(InputAutomationConfiguration_FILENAME.with(Clone::clone).clone());
    }
    

    pub fn isInstalled(&mut self) -> bool {
        return self.installed.clone();
    }
    

    pub fn setInstalled(&mut self, mut installed: bool) {
        self.installed = installed.clone();
        jcall!(self.logUtil).putF(format!("{}{}", String::from("Installed: "), installed), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setIntalled"));
    }
    

    pub fn getInputAutomationModuleConfigurationList(&mut self) -> Option<Rc<RefCell<dyn List<Option<Rc<RefCell<InputAutomationModuleConfiguration>>>>>>> {
        return self.inputAutomationModuleConfigurationList.clone();
    }
    

    // throws Exception
    pub fn save(&mut self) {
        let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create();
        let mut file: Option<Rc<RefCell<File>>> = File::new(InputAutomationConfiguration_FILENAME.with(Clone::clone).clone());
        jcall!(FileUtil::getInstance()).copy(AbPath::new(jcall!(file).getAbsolutePath(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>), AbPath::new(format!("{}{}", jcall!(file).getAbsolutePath(), String::from(".bak.xml")), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>));
        let mut jaxbContext: Option<Rc<RefCell<JAXBContext>>> = JAXBContext::newInstance(std::any::type_name::<InputAutomationConfiguration>());
        let mut marshaller: Option<Rc<RefCell<Marshaller>>> = jcall!(jaxbContext).createMarshaller();
        jcall!(marshaller).setProperty(unsafe { crate::javax::xml::bind::Marshaller::Marshaller_JAXB_FORMATTED_OUTPUT.clone() }.clone(), true);
        jcall!(marshaller).marshal(None /* Java this: a &mut self method has no Rc of itself */, document.clone());
        DomDocumentFileHelper::save(FileWrapperUtil::wrapFile(file.clone()), document.clone());
    }
}

