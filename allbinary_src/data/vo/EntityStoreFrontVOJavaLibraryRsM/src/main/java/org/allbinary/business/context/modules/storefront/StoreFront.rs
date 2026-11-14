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
// package org.allbinary.business.context.modules.storefront
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::Calendar::Calendar;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::business::context::configuration::ContextConfiguration::ContextConfiguration;
use crate::org::allbinary::business::context::configuration::ContextConfigurationDomDocumentMapping::ContextConfigurationDomDocumentMapping;
use crate::org::allbinary::business::context::configuration::ContextConfigurationInterface::ContextConfigurationInterface;
use crate::org::allbinary::business::context::configuration::ContextConfigurationInterfaceFactory::ContextConfigurationInterfaceFactory;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::globals::FREEBLISKET_PATH_GLOBALS::FREEBLISKET_PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParams;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::FileUtil::FileUtil;
use crate::org::allbinary::logic::io::file::directory::Directory::Directory;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::io::path::AbPathUtil::AbPathUtil;
use crate::org::allbinary::logic::string::SpecialCharacterUtil::SpecialCharacterUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::string::regex::replace::Replace::Replace;
use crate::org::allbinary::logic::string::tokens::Tokenizer::Tokenizer;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::apache::commons::lang3::StringEscapeUtils::StringEscapeUtils;
use crate::org::w3c::dom::Document::Document;
use crate::java::lang::Exception::*;
use super::StoreFrontInterface::StoreFrontInterface;
use super::StoreFrontData::StoreFrontData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreFront {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub directory: Option<Rc<RefCell<Directory>>>,
    pub stringUtil: Option<Rc<RefCell<StringUtil>>>,
    pub abPathUtil: Option<Rc<RefCell<AbPathUtil>>>,
    pub name: String,
    pub basketName: String,
    pub homeHostName: String,
    pub homeHostNamePath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub hostName: String,
    pub hostNamePath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub testHomeHostName: String,
    pub testHomeHostNamePath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub testHostName: String,
    pub testHostNamePath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub imagePath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub staticPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub categoryPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub inventoryControl: String,
    pub contextConfigurationInterface: Option<Rc<RefCell<dyn ContextConfigurationInterface>>>,
    pub subStores: String,
    pub tagLocation: String,
    pub packageLocation: String,
    pub ftp: String,
    pub ftpPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub ftpUserName: String,
    pub ftpPassword: String,
    pub testFtp: String,
    pub testFtpPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>,
    pub testFtpUserName: String,
    pub testFtpPassword: String,
    pub timeCreated: String,
    pub lastModified: String,
    pub MINCHAR: i32,
    pub MINSTORENAMELENGTH: i32,
    pub MAXCHAR: i32,
    pub EMPTY_STRING: String,
    pub ONE_EMPTY_STRING_ARRAY: Vec<String>,
}

thread_local! { pub static StoreFront_RESOURCES: String = format!("{}{}", String::from(" Resources"), { let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field }); }
impl StoreFrontInterface for StoreFront {
    fn isNameValid(&mut self) -> Option<bool> { Self::isNameValid(self) }
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn nameValidationInfo(&mut self) -> String { Self::nameValidationInfo(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
    fn getTestHtmlPath(&mut self) -> String { Self::getTestHtmlPath(self) }
    fn getCurrentHostName(&mut self) -> String { Self::getCurrentHostName(self) }
    fn getCurrentHostNamePath(&mut self) -> String { Self::getCurrentHostNamePath(self) }
    fn getCurrentHomeHostName(&mut self) -> String { Self::getCurrentHomeHostName(self) }
    fn getCurrentHomeHostNamePath(&mut self) -> String { Self::getCurrentHomeHostNamePath(self) }
    fn getName(&mut self) -> String { Self::getName(self) }
    fn getBasketName(&mut self) -> String { Self::getBasketName(self) }
    fn getHomeHostName(&mut self) -> String { Self::getHomeHostName(self) }
    fn getHomeHostNamePath(&mut self) -> String { Self::getHomeHostNamePath(self) }
    fn getHostName(&mut self) -> String { Self::getHostName(self) }
    fn getHostNamePath(&mut self) -> String { Self::getHostNamePath(self) }
    fn getTestHomeHostName(&mut self) -> String { Self::getTestHomeHostName(self) }
    fn getTestHomeHostNamePath(&mut self) -> String { Self::getTestHomeHostNamePath(self) }
    fn getTestHostName(&mut self) -> String { Self::getTestHostName(self) }
    fn getTestHostNamePath(&mut self) -> String { Self::getTestHostNamePath(self) }
    fn getStaticPath(&mut self) -> String { Self::getStaticPath(self) }
    fn getCategoryPath(&mut self) -> String { Self::getCategoryPath(self) }
    fn getSubStores(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getSubStores(self) }
    fn getTagLocation(&mut self) -> String { Self::getTagLocation(self) }
    fn getPackageLocation(&mut self) -> String { Self::getPackageLocation(self) }
    fn getInventoryControl(&mut self) -> String { Self::getInventoryControl(self) }
    fn getContextConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn ContextConfigurationInterface>>> { Self::getContextConfigurationInterface(self) }
    fn setContextConfigurationInterface(&mut self, mut contextConfigurationInterface: Option<Rc<RefCell<dyn ContextConfigurationInterface>>>) { Self::setContextConfigurationInterface(self, contextConfigurationInterface) }
    fn getFtp(&mut self) -> String { Self::getFtp(self) }
    fn getFtpPath(&mut self) -> String { Self::getFtpPath(self) }
    fn getFtpUserName(&mut self) -> String { Self::getFtpUserName(self) }
    fn getFtpPassword(&mut self) -> String { Self::getFtpPassword(self) }
    fn getTestFtp(&mut self) -> String { Self::getTestFtp(self) }
    fn getTestFtpPath(&mut self) -> String { Self::getTestFtpPath(self) }
    fn getTestFtpUserName(&mut self) -> String { Self::getTestFtpUserName(self) }
    fn getTestFtpPassword(&mut self) -> String { Self::getTestFtpPassword(self) }
    fn getTimeCreated(&mut self) -> String { Self::getTimeCreated(self) }
    fn getLastModified(&mut self) -> String { Self::getLastModified(self) }
    fn setName(&mut self, mut value: String) { Self::setName(self, value) }
    fn setBasketName(&mut self, mut value: String) { Self::setBasketName(self, value) }
    fn setHomeHostName(&mut self, mut value: String) { Self::setHomeHostName(self, value) }
    fn setHomeHostNamePath(&mut self, mut value: String) { Self::setHomeHostNamePath(self, value) }
    fn setHostName(&mut self, mut value: String) { Self::setHostName(self, value) }
    fn setHostNamePath(&mut self, mut value: String) { Self::setHostNamePath(self, value) }
    fn setTestHomeHostName(&mut self, mut value: String) { Self::setTestHomeHostName(self, value) }
    fn setTestHomeHostNamePath(&mut self, mut value: String) { Self::setTestHomeHostNamePath(self, value) }
    fn setTestHostName(&mut self, mut value: String) { Self::setTestHostName(self, value) }
    fn setTestHostNamePath(&mut self, mut value: String) { Self::setTestHostNamePath(self, value) }
    fn setImagePath(&mut self, mut value: String) { Self::setImagePath(self, value) }
    fn setStaticPath(&mut self, mut value: String) { Self::setStaticPath(self, value) }
    fn setCategoryPath(&mut self, mut value: String) { Self::setCategoryPath(self, value) }
    fn setPackageLocation(&mut self, mut value: String) { Self::setPackageLocation(self, value) }
    fn setInventoryControl(&mut self, mut value: String) { Self::setInventoryControl(self, value) }
    fn setFtp(&mut self, mut value: String) { Self::setFtp(self, value) }
    fn setFtpPath(&mut self, mut value: String) { Self::setFtpPath(self, value) }
    fn setFtpUserName(&mut self, mut value: String) { Self::setFtpUserName(self, value) }
    fn setFtpPassword(&mut self, mut value: String) { Self::setFtpPassword(self, value) }
    fn setTestFtp(&mut self, mut value: String) { Self::setTestFtp(self, value) }
    fn setTestFtpPath(&mut self, mut value: String) { Self::setTestFtpPath(self, value) }
    fn setTestFtpUserName(&mut self, mut value: String) { Self::setTestFtpUserName(self, value) }
    fn setTestFtpPassword(&mut self, mut value: String) { Self::setTestFtpPassword(self, value) }
    fn setTimeCreated(&mut self, mut value: String) { Self::setTimeCreated(self, value) }
    fn setLastModified(&mut self, mut value: String) { Self::setLastModified(self, value) }
    fn install(&mut self, mut current: i32, mut total: i32) { Self::install(self, current, total) }
}
impl crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface for StoreFront {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toVector(self) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> { Self::toHashMap(self) }
}

impl StoreFront {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            directory: None,
            stringUtil: None,
            abPathUtil: None,
            name: String::new(),
            basketName: String::new(),
            homeHostName: String::new(),
            homeHostNamePath: None,
            hostName: String::new(),
            hostNamePath: None,
            testHomeHostName: String::new(),
            testHomeHostNamePath: None,
            testHostName: String::new(),
            testHostNamePath: None,
            imagePath: None,
            staticPath: None,
            categoryPath: None,
            inventoryControl: String::new(),
            contextConfigurationInterface: None,
            subStores: String::new(),
            tagLocation: String::new(),
            packageLocation: String::new(),
            ftp: String::new(),
            ftpPath: None,
            ftpUserName: String::new(),
            ftpPassword: String::new(),
            testFtp: String::new(),
            testFtpPath: None,
            testFtpUserName: String::new(),
            testFtpPassword: String::new(),
            timeCreated: String::new(),
            lastModified: String::new(),
            MINCHAR: 0,
            MINSTORENAMELENGTH: 0,
            MAXCHAR: 0,
            EMPTY_STRING: String::new(),
            ONE_EMPTY_STRING_ARRAY: Vec::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.directory = Directory::getInstance();
        this.stringUtil = StringUtil::getInstance();
        this.abPathUtil = AbPathUtil::getInstance();
        this.name = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.basketName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.homeHostName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.homeHostNamePath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.hostName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.hostNamePath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.testHomeHostName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.testHomeHostNamePath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.testHostName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.testHostNamePath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.imagePath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.staticPath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.categoryPath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.inventoryControl = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.subStores = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.tagLocation = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.packageLocation = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.ftp = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.ftpPath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.ftpUserName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.ftpPassword = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.testFtp = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.testFtpPath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.testFtpUserName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.testFtpPassword = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.timeCreated = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.lastModified = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.MINCHAR = 0;
        this.MINSTORENAMELENGTH = 1;
        this.MAXCHAR = 255;
        this.EMPTY_STRING = { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
        this.ONE_EMPTY_STRING_ARRAY = { let java_field = jcall!(StringUtil::getInstance()).ONE_EMPTY_STRING_ARRAY.clone(); java_field }.clone();
        this.contextConfigurationInterface = ContextConfiguration::new().map(|reference| reference as Rc<RefCell<dyn ContextConfigurationInterface>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            directory: None,
            stringUtil: None,
            abPathUtil: None,
            name: String::new(),
            basketName: String::new(),
            homeHostName: String::new(),
            homeHostNamePath: None,
            hostName: String::new(),
            hostNamePath: None,
            testHomeHostName: String::new(),
            testHomeHostNamePath: None,
            testHostName: String::new(),
            testHostNamePath: None,
            imagePath: None,
            staticPath: None,
            categoryPath: None,
            inventoryControl: String::new(),
            contextConfigurationInterface: None,
            subStores: String::new(),
            tagLocation: String::new(),
            packageLocation: String::new(),
            ftp: String::new(),
            ftpPath: None,
            ftpUserName: String::new(),
            ftpPassword: String::new(),
            testFtp: String::new(),
            testFtpPath: None,
            testFtpUserName: String::new(),
            testFtpPassword: String::new(),
            timeCreated: String::new(),
            lastModified: String::new(),
            MINCHAR: 0,
            MINSTORENAMELENGTH: 0,
            MAXCHAR: 0,
            EMPTY_STRING: String::new(),
            ONE_EMPTY_STRING_ARRAY: Vec::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.directory = Directory::getInstance();
        this.stringUtil = StringUtil::getInstance();
        this.abPathUtil = AbPathUtil::getInstance();
        this.name = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.basketName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.homeHostName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.homeHostNamePath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.hostName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.hostNamePath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.testHomeHostName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.testHomeHostNamePath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.testHostName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.testHostNamePath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.imagePath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.staticPath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.categoryPath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.inventoryControl = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.subStores = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.tagLocation = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.packageLocation = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.ftp = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.ftpPath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.ftpUserName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.ftpPassword = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.testFtp = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.testFtpPath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.testFtpUserName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.testFtpPassword = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.timeCreated = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.lastModified = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.MINCHAR = 0;
        this.MINSTORENAMELENGTH = 1;
        this.MAXCHAR = 255;
        this.EMPTY_STRING = { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
        this.ONE_EMPTY_STRING_ARRAY = { let java_field = jcall!(StringUtil::getInstance()).ONE_EMPTY_STRING_ARRAY.clone(); java_field }.clone();
        { let java_arguments = (jcall!(RequestParams::new1(request.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParamsDyn>>)).toHashMap(),); this.getFormData(java_arguments.0) };
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).EMAILLOGGING.clone(); java_field }.clone()) {
            jcall!(this.logUtil).putF(String::from("TWB- needs update for adding and updating context configuration - currently defaults to admin context config"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("StoreFront"));
        }
        { let java_arguments = (jcall!(ContextConfigurationInterfaceFactory::getInstance()).getInstance1_String(String::from("Admin")),); this.setContextConfigurationInterface(java_arguments.0) };
        this
    }

    pub fn new1(mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(request))))
    }
    

    pub fn new_struct1_HashMap(mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            directory: None,
            stringUtil: None,
            abPathUtil: None,
            name: String::new(),
            basketName: String::new(),
            homeHostName: String::new(),
            homeHostNamePath: None,
            hostName: String::new(),
            hostNamePath: None,
            testHomeHostName: String::new(),
            testHomeHostNamePath: None,
            testHostName: String::new(),
            testHostNamePath: None,
            imagePath: None,
            staticPath: None,
            categoryPath: None,
            inventoryControl: String::new(),
            contextConfigurationInterface: None,
            subStores: String::new(),
            tagLocation: String::new(),
            packageLocation: String::new(),
            ftp: String::new(),
            ftpPath: None,
            ftpUserName: String::new(),
            ftpPassword: String::new(),
            testFtp: String::new(),
            testFtpPath: None,
            testFtpUserName: String::new(),
            testFtpPassword: String::new(),
            timeCreated: String::new(),
            lastModified: String::new(),
            MINCHAR: 0,
            MINSTORENAMELENGTH: 0,
            MAXCHAR: 0,
            EMPTY_STRING: String::new(),
            ONE_EMPTY_STRING_ARRAY: Vec::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.directory = Directory::getInstance();
        this.stringUtil = StringUtil::getInstance();
        this.abPathUtil = AbPathUtil::getInstance();
        this.name = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.basketName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.homeHostName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.homeHostNamePath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.hostName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.hostNamePath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.testHomeHostName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.testHomeHostNamePath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.testHostName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.testHostNamePath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.imagePath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.staticPath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.categoryPath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.inventoryControl = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.subStores = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.tagLocation = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.packageLocation = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.ftp = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.ftpPath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.ftpUserName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.ftpPassword = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.testFtp = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.testFtpPath = { let java_field = jcall!(this.abPathUtil).NO_ABPATH.clone(); java_field }.clone();
        this.testFtpUserName = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.testFtpPassword = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.timeCreated = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.lastModified = { let java_field = jcall!(this.stringUtil).EMPTY_STRING.clone(); java_field }.clone();
        this.MINCHAR = 0;
        this.MINSTORENAMELENGTH = 1;
        this.MAXCHAR = 255;
        this.EMPTY_STRING = { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
        this.ONE_EMPTY_STRING_ARRAY = { let java_field = jcall!(StringUtil::getInstance()).ONE_EMPTY_STRING_ARRAY.clone(); java_field }.clone();
        this.getFormData(hashMap.clone());
        let mut domDocumentString: String = { let java_object = (jcall!(hashMap).get(crate::java_into_object({ let java_field = jcall!(StoreFrontData::getInstance()).CONFIGURATION.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
        if false {
            let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1_String(domDocumentString.clone());
            { let java_arguments = (jcall!(ContextConfigurationInterfaceFactory::getInstance()).getInstance1(document.clone()),); this.setContextConfigurationInterface(java_arguments.0) };
        }
        this
    }

    pub fn new1_HashMap(mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1_HashMap(hashMap))))
    }
    

    pub fn isColumn(&mut self, mut columnName: String) -> bool {
        if (columnName.cmp(&{ let java_field = jcall!(StoreFrontData::getInstance()).NAME.clone(); java_field }.clone()) as i32) == 0 || (columnName.cmp(&crate::org::allbinary::business::user::UserData::UserData_MAINEMAIL.with(Clone::clone).clone()) as i32) == 0 {
            return true;
        } else {
            return false;
        }
    }
    

    // throws Exception
    pub fn createPath(&mut self, mut key: String, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return AbPath::new({ let java_arguments = (self.EMPTY_STRING.clone(), key.clone(), hashMap.clone(),); self.create(java_arguments.0, java_arguments.1, java_arguments.2) }, { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn createPath3(&mut self, mut append: String, mut key: String, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> {
        return AbPath::new(self.create(append.clone(), key.clone(), hashMap.clone()), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
    }
    

    pub fn get(&mut self, mut key: String, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> String {
        return { let java_arguments = (self.EMPTY_STRING.clone(), key.clone(), hashMap.clone(),); self.create(java_arguments.0, java_arguments.1, java_arguments.2) };
    }
    

    pub fn create(&mut self, mut append: String, mut key: String, mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> String {
        let mut path: String = { let java_object = (jcall!(hashMap).get(crate::java_into_object(key.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
        if jcall!(StringValidationUtil::getInstance()).isEmpty(path.clone()) {
            return self.EMPTY_STRING.clone();
        } else {
            if self.isColumn(path.clone()) {
                return format!("{}{}", { let java_object = (jcall!(hashMap).get(crate::java_into_object(path.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }, append);
            } else {
                return path.clone();
            }
        }
    }
    

    // throws Exception
    pub fn getFormData(&mut self, mut storeHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) {
        let mut storeFrontData: Option<Rc<RefCell<StoreFrontData>>> = StoreFrontData::getInstance();
        self.name = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).NAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.basketName = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).NAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.homeHostName = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).HOMEHOSTNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.homeHostNamePath = { let java_value = ({ let java_arguments = ({ let java_field = jcall!(storeFrontData).HOMEHOSTNAMEPATH.clone(); java_field }.clone(), storeHashMap.clone(),); self.createPath(java_arguments.0, java_arguments.1) }).clone(); java_value };
        self.hostName = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).HOSTNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.hostNamePath = { let java_value = ({ let java_arguments = ({ let java_field = jcall!(storeFrontData).HOSTNAMEPATH.clone(); java_field }.clone(), storeHashMap.clone(),); self.createPath(java_arguments.0, java_arguments.1) }).clone(); java_value };
        self.testHomeHostName = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTHOMEHOSTNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.testHomeHostNamePath = { let java_value = ({ let java_arguments = ({ let java_field = jcall!(storeFrontData).TESTHOMEHOSTNAMEPATH.clone(); java_field }.clone(), storeHashMap.clone(),); self.createPath(java_arguments.0, java_arguments.1) }).clone(); java_value };
        self.testHostName = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTHOSTNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.testHostNamePath = { let java_value = ({ let java_arguments = ({ let java_field = jcall!(storeFrontData).TESTHOSTNAMEPATH.clone(); java_field }.clone(), storeHashMap.clone(),); self.createPath(java_arguments.0, java_arguments.1) }).clone(); java_value };
        self.imagePath = { let java_value = ({ let java_arguments = (StoreFront_RESOURCES.with(Clone::clone).clone(), { let java_field = jcall!(storeFrontData).IMAGEPATH.clone(); java_field }.clone(), storeHashMap.clone(),); self.createPath3(java_arguments.0, java_arguments.1, java_arguments.2) }).clone(); java_value };
        self.staticPath = { let java_value = ({ let java_arguments = (format!("{}{}", String::from(" Products"), { let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field }), { let java_field = jcall!(storeFrontData).STATICPATH.clone(); java_field }.clone(), storeHashMap.clone(),); self.createPath3(java_arguments.0, java_arguments.1, java_arguments.2) }).clone(); java_value };
        self.categoryPath = { let java_value = ({ let java_arguments = (StoreFront_RESOURCES.with(Clone::clone).clone(), { let java_field = jcall!(storeFrontData).CATEGORYPATH.clone(); java_field }.clone(), storeHashMap.clone(),); self.createPath3(java_arguments.0, java_arguments.1, java_arguments.2) }).clone(); java_value };
        self.inventoryControl = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).INVENTORYCONTROL.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.subStores = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).SUBSTORES.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.tagLocation = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).TAGLOCATION.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.packageLocation = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).PACKAGELOCATION.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.ftp = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).FTP.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.ftpPath = { let java_value = ({ let java_arguments = ({ let java_field = jcall!(storeFrontData).FTPPATH.clone(); java_field }.clone(), storeHashMap.clone(),); self.createPath(java_arguments.0, java_arguments.1) }).clone(); java_value };
        self.ftpUserName = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).FTPUSERNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.ftpPassword = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).FTPPASSWORD.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.testFtp = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTFTP.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.testFtpPath = { let java_value = ({ let java_arguments = ({ let java_field = jcall!(storeFrontData).TESTFTPPATH.clone(); java_field }.clone(), storeHashMap.clone(),); self.createPath(java_arguments.0, java_arguments.1) }).clone(); java_value };
        self.testFtpUserName = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTFTPUSERNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.testFtpPassword = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTFTPPASSWORD.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.timeCreated = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).TIMECREATED.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.lastModified = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).LASTMODIFIED.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
    }
    

    pub fn isNameValid(&mut self) -> Option<bool> {
        if !jcall!(StringValidationUtil::getInstance()).isValidRequired(self.name.clone(), self.MINSTORENAMELENGTH.clone(), self.MAXCHAR.clone()) {
            return false;
        } else {
            let mut isEscapedCharactersContained: bool = false;
            let mut storeNameEscaped: String = StringEscapeUtils::escapeHtml3(self.name.clone());
            if (self.name.cmp(&storeNameEscaped.clone()) as i32) != 0 {
                isEscapedCharactersContained = true;
            }
            let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
            let mut isSpacesContained: bool = false;
            if jcall!(stringValidationUtil).containsSpaces(self.name.clone()) {
                isSpacesContained = true;
            }
            if isSpacesContained || isEscapedCharactersContained {
                let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = SpecialCharacterUtil::getHashMap();
                jcall!(hashMap).put(crate::java_into_object(vec![{ let java_field = jcall!(CommonSeps::getInstance()).SPACE.clone(); java_field }.clone()]), crate::java_into_object(self.ONE_EMPTY_STRING_ARRAY.clone()));
                self.name = { let java_value = (jcall!(Replace::new1(hashMap.clone())).all(self.name.clone())).clone(); java_value };
            }
        }
        return true;
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        let mut valid: Option<bool> = true;
        valid = self.isNameValid();
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        if !jcall!(stringValidationUtil).isValidRequired(self.basketName.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.homeHostName.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(jcall!(self.homeHostNamePath).toString(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.hostName.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidRequired(jcall!(self.hostNamePath).toString(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.testHomeHostName.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(jcall!(self.testHomeHostNamePath).toString(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.testHostName.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(jcall!(self.testHostNamePath).toString(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidRequired(jcall!(self.imagePath).toString(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidRequired(jcall!(self.staticPath).toString(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidRequired(jcall!(self.categoryPath).toString(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.inventoryControl.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.subStores.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.tagLocation.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidRequired(self.packageLocation.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.ftp.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(jcall!(self.ftpPath).toString(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.ftpUserName.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.ftpPassword.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.testFtp.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(jcall!(self.testFtpPath).toString(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.testFtpUserName.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if !jcall!(stringValidationUtil).isValidNotRequired(self.testFtpPassword.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            valid = { let java_value = (false).clone(); java_value }.clone();
        }
        if valid.unwrap() == true {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn nameValidationInfo(&mut self) -> String {
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        if !jcall!(stringValidationUtil).isValidRequired(self.name.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Name is invalid. Must be < "));
            jcall!(stringBuffer).appendint(self.MAXCHAR.clone());
            jcall!(stringBuffer).append(String::from(" and > 0 characters.<br>"));
            return jcall!(stringBuffer).toString();
        }
        return self.EMPTY_STRING.clone();
    }
    

    pub fn validationInfo(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(self.nameValidationInfo());
            if !jcall!(stringValidationUtil).isValidRequired(self.basketName.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(String::from("Basket name is invalid. Must be < "));
                jcall!(stringBuffer).appendint(self.MAXCHAR.clone());
                jcall!(stringBuffer).append(String::from(" and > 0 characters.<br>"));
            }
            if !jcall!(stringValidationUtil).isValidRequired(self.homeHostName.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(String::from("Home host name is invalid. Must be < "));
                jcall!(stringBuffer).appendint(self.MAXCHAR.clone());
                jcall!(stringBuffer).append(String::from(" and > 0 characters.<br>"));
            }
            if !jcall!(stringValidationUtil).isValidNotRequired(jcall!(self.homeHostNamePath).toString(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(String::from("Home host name path is invalid. Must be < "));
                jcall!(stringBuffer).appendint(self.MAXCHAR.clone());
                jcall!(stringBuffer).append(String::from(" and > 0 characters.<br>"));
            }
            if !jcall!(stringValidationUtil).isValidRequired(self.hostName.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(String::from("Host name is invalid. Must be < "));
                jcall!(stringBuffer).appendint(self.MAXCHAR.clone());
                jcall!(stringBuffer).append(String::from(" and > 0 characters.<br>"));
            }
            if !jcall!(stringValidationUtil).isValidRequired(jcall!(self.hostNamePath).toString(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(String::from("Host name path is invalid. Must be < "));
                jcall!(stringBuffer).appendint(self.MAXCHAR.clone());
                jcall!(stringBuffer).append(String::from(" and > 0 characters.<br>"));
            }
            if !jcall!(stringValidationUtil).isValidNotRequired(self.testHomeHostName.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Test home host name is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            if !jcall!(stringValidationUtil).isValidNotRequired(jcall!(self.testHomeHostNamePath).toString(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Test home host name path is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            if !jcall!(stringValidationUtil).isValidNotRequired(self.testHostName.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Test host name is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            if !jcall!(stringValidationUtil).isValidNotRequired(jcall!(self.testHostNamePath).toString(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Test host name path is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            if !jcall!(stringValidationUtil).isValidRequired(jcall!(self.imagePath).toString(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Image path is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            if !jcall!(stringValidationUtil).isValidRequired(jcall!(self.staticPath).toString(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Static path is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            if !jcall!(stringValidationUtil).isValidRequired(jcall!(self.categoryPath).toString(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Category path is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            if !jcall!(stringValidationUtil).isValidRequired(self.inventoryControl.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Inventory control is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            if !jcall!(stringValidationUtil).isValidNotRequired(self.subStores.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Sub stores is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            if !jcall!(stringValidationUtil).isValidNotRequired(self.tagLocation.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Tag location is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            if !jcall!(stringValidationUtil).isValidRequired(self.packageLocation.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Package location is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            if !jcall!(stringValidationUtil).isValidNotRequired(self.ftp.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Ftp is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            if !jcall!(stringValidationUtil).isValidNotRequired(jcall!(self.ftpPath).toString(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Ftp path is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            if !jcall!(stringValidationUtil).isValidNotRequired(self.ftpUserName.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Ftp username is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            if !jcall!(stringValidationUtil).isValidNotRequired(self.ftpPassword.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Ftp password is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            if !jcall!(stringValidationUtil).isValidNotRequired(self.testFtp.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Test ftp is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            if !jcall!(stringValidationUtil).isValidNotRequired(jcall!(self.testFtpPath).toString(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Test ftp Path is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            if !jcall!(stringValidationUtil).isValidNotRequired(self.testFtpUserName.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Test ftp username is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            if !jcall!(stringValidationUtil).isValidNotRequired(self.testFtpPassword.clone(), self.MINCHAR.clone(), self.MAXCHAR.clone()) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Test ftp password is invalid. Must be < "), self.MAXCHAR), String::from(" and > 0 characters.<br>")));
            }
            return Some(jcall!(stringBuffer).toString());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to generate validation error info"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"), crate::java_into_object(e.clone()));
            }
            return String::from("Error Validating Form");
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn getTestHtmlPath(&mut self) -> String {
        return format!("{}{}", { let java_value = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).globals.clone(); java_field }).URLGLOBALS.clone(); java_field }).getTestHtmlPath(); java_value }, self.getCurrentHostNamePath());
    }
    

    pub fn getCurrentHostName(&mut self) -> String {
        let mut location: String = self.EMPTY_STRING.clone();
        if jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).globals.clone(); java_field }).URLGLOBALS.clone(); java_field }).isTestingMode() {
            location = self.getTestHostName();
        } else {
            location = self.getHostName();
        }
        return location.clone();
    }
    

    pub fn getCurrentHostNamePath(&mut self) -> String {
        let mut location: String = self.EMPTY_STRING.clone();
        if jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).globals.clone(); java_field }).URLGLOBALS.clone(); java_field }).isTestingMode() {
            location = self.getTestHostNamePath();
        } else {
            location = self.getHostNamePath();
        }
        return location.clone();
    }
    

    pub fn getCurrentHomeHostName(&mut self) -> String {
        let mut location: String = self.EMPTY_STRING.clone();
        if jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).globals.clone(); java_field }).URLGLOBALS.clone(); java_field }).isTestingMode() {
            location = self.getTestHomeHostName();
        } else {
            location = self.getHomeHostName();
        }
        return location.clone();
    }
    

    pub fn getCurrentHomeHostNamePath(&mut self) -> String {
        let mut location: String = self.EMPTY_STRING.clone();
        if jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).globals.clone(); java_field }).URLGLOBALS.clone(); java_field }).isTestingMode() {
            location = self.getTestHomeHostNamePath();
        } else {
            location = self.getHomeHostNamePath();
        }
        return location.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getBasketName(&mut self) -> String {
        return self.basketName.clone();
    }
    

    pub fn getHomeHostName(&mut self) -> String {
        return self.homeHostName.clone();
    }
    

    pub fn getHomeHostNamePath(&mut self) -> String {
        return jcall!(self.homeHostNamePath).toString();
    }
    

    pub fn getHostName(&mut self) -> String {
        return self.hostName.clone();
    }
    

    pub fn getHostNamePath(&mut self) -> String {
        return jcall!(self.hostNamePath).toString();
    }
    

    pub fn getTestHomeHostName(&mut self) -> String {
        return self.testHomeHostName.clone();
    }
    

    pub fn getTestHomeHostNamePath(&mut self) -> String {
        return jcall!(self.testHomeHostNamePath).toString();
    }
    

    pub fn getTestHostName(&mut self) -> String {
        return self.testHostName.clone();
    }
    

    pub fn getTestHostNamePath(&mut self) -> String {
        return jcall!(self.testHostNamePath).toString();
    }
    

    pub fn getStaticPath(&mut self) -> String {
        return jcall!(self.staticPath).toString();
    }
    

    pub fn getCategoryPath(&mut self) -> String {
        return jcall!(self.categoryPath).toString();
    }
    

    pub fn getInventoryControl(&mut self) -> String {
        return self.inventoryControl.clone();
    }
    

    // throws Exception
    pub fn getSubStores(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut tokenizer: Option<Rc<RefCell<Tokenizer>>> = Tokenizer::new({ let java_field = jcall!(CommonSeps::getInstance()).SEMICOLON.clone(); java_field }.clone());
            let mut subStoreVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(tokenizer).getTokensFromString(self.subStores.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
            return Some(subStoreVector.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn getTagLocation(&mut self) -> String {
        return self.tagLocation.clone();
    }
    

    pub fn getPackageLocation(&mut self) -> String {
        return self.packageLocation.clone();
    }
    

    pub fn getFtp(&mut self) -> String {
        return self.ftp.clone();
    }
    

    pub fn getFtpUserName(&mut self) -> String {
        return self.ftpUserName.clone();
    }
    

    pub fn getFtpPassword(&mut self) -> String {
        return self.ftpPassword.clone();
    }
    

    pub fn getTestFtp(&mut self) -> String {
        return self.testFtp.clone();
    }
    

    pub fn getTestFtpUserName(&mut self) -> String {
        return self.testFtpUserName.clone();
    }
    

    pub fn getTestFtpPassword(&mut self) -> String {
        return self.testFtpPassword.clone();
    }
    

    pub fn getFtpPath(&mut self) -> String {
        return jcall!(self.ftpPath).toString();
    }
    

    pub fn getTestFtpPath(&mut self) -> String {
        return jcall!(self.testFtpPath).toString();
    }
    

    pub fn getTimeCreated(&mut self) -> String {
        return self.timeCreated.clone();
    }
    

    pub fn getLastModified(&mut self) -> String {
        return self.lastModified.clone();
    }
    

    pub fn setName(&mut self, mut value: String) {
        self.name = value.clone();
    }
    

    pub fn setBasketName(&mut self, mut value: String) {
        self.basketName = value.clone();
    }
    

    pub fn setHomeHostName(&mut self, mut value: String) {
        self.homeHostName = value.clone();
    }
    

    // throws Exception
    pub fn setHomeHostNamePath(&mut self, mut value: String) {
        self.homeHostNamePath = { let java_value = (AbPath::new(value.clone(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).clone(); java_value };
    }
    

    pub fn setHostName(&mut self, mut value: String) {
        self.hostName = value.clone();
    }
    

    // throws Exception
    pub fn setHostNamePath(&mut self, mut value: String) {
        self.hostNamePath = { let java_value = (AbPath::new(value.clone(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).clone(); java_value };
    }
    

    pub fn setTestHomeHostName(&mut self, mut value: String) {
        self.testHomeHostName = value.clone();
    }
    

    // throws Exception
    pub fn setTestHomeHostNamePath(&mut self, mut value: String) {
        self.testHomeHostNamePath = { let java_value = (AbPath::new(value.clone(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).clone(); java_value };
    }
    

    pub fn setTestHostName(&mut self, mut value: String) {
        self.testHostName = value.clone();
    }
    

    // throws Exception
    pub fn setTestHostNamePath(&mut self, mut value: String) {
        self.testHostNamePath = { let java_value = (AbPath::new(value.clone(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).clone(); java_value };
    }
    

    // throws Exception
    pub fn setImagePath(&mut self, mut value: String) {
        self.imagePath = { let java_value = (AbPath::new(value.clone(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).clone(); java_value };
    }
    

    // throws Exception
    pub fn setStaticPath(&mut self, mut value: String) {
        self.staticPath = { let java_value = (AbPath::new(value.clone(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).clone(); java_value };
    }
    

    // throws Exception
    pub fn setCategoryPath(&mut self, mut value: String) {
        self.categoryPath = { let java_value = (AbPath::new(value.clone(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).clone(); java_value };
    }
    

    pub fn setSubStores(&mut self, mut value: String) {
        self.subStores = value.clone();
    }
    

    pub fn setTagLocation(&mut self, mut value: String) {
        self.tagLocation = value.clone();
    }
    

    pub fn setPackageLocation(&mut self, mut value: String) {
        self.packageLocation = value.clone();
    }
    

    pub fn setInventoryControl(&mut self, mut value: String) {
        self.inventoryControl = value.clone();
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getPackageLocation1(&mut self, mut value: String) {
        self.packageLocation = value.clone();
    }
    

    pub fn setFtp(&mut self, mut value: String) {
        self.ftp = value.clone();
    }
    

    pub fn setFtpUserName(&mut self, mut value: String) {
        self.ftpUserName = value.clone();
    }
    

    pub fn setFtpPassword(&mut self, mut value: String) {
        self.ftpPassword = value.clone();
    }
    

    pub fn setTestFtp(&mut self, mut value: String) {
        self.testFtp = value.clone();
    }
    

    pub fn setTestFtpUserName(&mut self, mut value: String) {
        self.testFtpUserName = value.clone();
    }
    

    pub fn setTestFtpPassword(&mut self, mut value: String) {
        self.testFtpPassword = value.clone();
    }
    

    pub fn setTimeCreated(&mut self, mut value: String) {
        self.timeCreated = value.clone();
    }
    

    pub fn setLastModified(&mut self, mut value: String) {
        self.lastModified = value.clone();
    }
    

    // throws Exception
    pub fn setFtpPath(&mut self, mut value: String) {
        self.ftpPath = { let java_value = (AbPath::new(value.clone(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).clone(); java_value };
    }
    

    // throws Exception
    pub fn setTestFtpPath(&mut self, mut value: String) {
        self.testFtpPath = { let java_value = (AbPath::new(value.clone(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>)).clone(); java_value };
    }
    

    // throws Exception
    pub fn createDirectories(&mut self) -> bool {
        let mut storeAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(format!("{}{}", { let java_value = URLGLOBALS::getWebappPath(); java_value }, self.getCurrentHomeHostNamePath()), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        if !jcall!(self.directory).create(storeAbPath.clone()) {
            return false;
        }
        if !{ let java_receiver = self.directory.clone(); let java_arguments = (AbPath::new(format!("{}{}", storeAbPath, self.getCategoryPath()), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>),); let java_result = jcall!(java_receiver).create(java_arguments.0); java_result } {
            return false;
        }
        if !{ let java_receiver = self.directory.clone(); let java_arguments = (AbPath::new(format!("{}{}", storeAbPath, self.getStaticPath()), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>),); let java_result = jcall!(java_receiver).create(java_arguments.0); java_result } {
            return false;
        }
        return true;
    }
    

    // throws Exception
    pub fn install(&mut self, mut current: i32, mut total: i32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if !self.createDirectories() {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Unable to create store directories"))));
            }
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            jcall!(stringBuffer).append(URLGLOBALS::getMainPath());
            jcall!(stringBuffer).append({ let java_field = jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).XSLPATH.clone(); java_field }.clone());
            let mut viewPath: String = jcall!(stringBuffer).toString();
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            jcall!(stringBuffer).append(viewPath.clone());
            jcall!(stringBuffer).append({ let java_field = jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).INSTALLPATH.clone(); java_field }.clone());
            let mut fromDirectoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(jcall!(stringBuffer).toString(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            jcall!(stringBuffer).append(viewPath.clone());
            jcall!(stringBuffer).append(String::from("special"));
            jcall!(stringBuffer).append({ let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field }.clone());
            let mut fromSpecialDirectoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(jcall!(stringBuffer).toString(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            jcall!(stringBuffer).append(viewPath.clone());
            jcall!(stringBuffer).append(self.getName());
            jcall!(stringBuffer).append({ let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field }.clone());
            let mut toDirectoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(jcall!(stringBuffer).toString(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
            if current == 0 {
                if !jcall!(self.directory).create(toDirectoryAbPath.clone()) {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Unable to create store view directory: "), jcall!(toDirectoryAbPath).toString()))));
                }
            }
            let mut viewTotal: i32 = (total * 85) / 100;
            let mut installTotal: i32 = (total * 93) / 100;
            if current < viewTotal {
                let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(fromSpecialDirectoryAbPath.clone());
                if jcall!(file).isDirectory() {
                    let mut halfViewTotal: i32 = viewTotal / 2;
                    if current < halfViewTotal {
                        self.installViews(fromSpecialDirectoryAbPath.clone(), toDirectoryAbPath.clone(), current.clone(), halfViewTotal.clone());
                    } else if current < viewTotal {
                        self.installViews(fromDirectoryAbPath.clone(), toDirectoryAbPath.clone(), current - halfViewTotal, viewTotal - halfViewTotal);
                    }
                } else {
                    self.installViews(fromDirectoryAbPath.clone(), toDirectoryAbPath.clone(), current.clone(), viewTotal.clone());
                }
            } else if current < installTotal {
                self.installResources(fromDirectoryAbPath.clone(), current - viewTotal, installTotal - viewTotal);
            } else {
                self.installMedia(fromDirectoryAbPath.clone(), current - installTotal, total - installTotal);
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).SQLTAGSERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("install()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn installViews(&mut self, mut fromDirectoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut toDirectoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut current: i32, mut total: i32) {
        jcall!(FileUtil::getInstance()).copyDirectoryPortion(fromDirectoryAbPath.clone(), toDirectoryAbPath.clone(), true, false, current.clone(), total.clone());
    }
    

    // throws Exception
    pub fn installResources(&mut self, mut fromDirectoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut current: i32, mut total: i32) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(fromDirectoryAbPath).toString());
        jcall!(stringBuffer).append({ let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field }.clone());
        jcall!(stringBuffer).append(String::from("Resources"));
        jcall!(stringBuffer).append({ let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field }.clone());
        let mut installCategoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(jcall!(stringBuffer).toString(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(URLGLOBALS::getMainPath());
        jcall!(stringBuffer).append(self.getCurrentHomeHostNamePath());
        jcall!(stringBuffer).append({ let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field }.clone());
        jcall!(stringBuffer).append(self.getCategoryPath());
        let mut categoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(jcall!(stringBuffer).toString(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        jcall!(FileUtil::getInstance()).copyDirectoryPortion(installCategoryAbPath.clone(), categoryAbPath.clone(), true, false, current.clone(), total.clone());
    }
    

    // throws Exception
    pub fn installMedia(&mut self, mut fromDirectoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>, mut current: i32, mut total: i32) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(fromDirectoryAbPath).toString());
        jcall!(stringBuffer).append({ let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).TEMPLATEPATH.clone(); java_field }.clone());
        jcall!(stringBuffer).append(String::from("images"));
        jcall!(stringBuffer).append({ let java_field = jcall!(AbPathData::getInstance()).SEPARATOR.clone(); java_field }.clone());
        let mut viewStoreImagesDirectoryAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(jcall!(stringBuffer).toString(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        let mut storeAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(format!("{}{}", { let java_value = URLGLOBALS::getWebappPath(); java_value }, self.getCurrentHomeHostNamePath()), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        jcall!(FileUtil::getInstance()).copyDirectoryPortion(viewStoreImagesDirectoryAbPath.clone(), storeAbPath.clone(), true, false, current.clone(), total.clone());
    }
    

    // throws Exception
    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut dataVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(dataVector).add(crate::java_into_object(self.name.clone()));
        jcall!(dataVector).add(crate::java_into_object(self.homeHostName.clone()));
        jcall!(dataVector).add(crate::java_into_object(jcall!(self.homeHostNamePath).toString()));
        jcall!(dataVector).add(crate::java_into_object(self.hostName.clone()));
        jcall!(dataVector).add(crate::java_into_object(jcall!(self.hostNamePath).toString()));
        jcall!(dataVector).add(crate::java_into_object(self.testHomeHostName.clone()));
        jcall!(dataVector).add(crate::java_into_object(jcall!(self.testHomeHostNamePath).toString()));
        jcall!(dataVector).add(crate::java_into_object(self.testHostName.clone()));
        jcall!(dataVector).add(crate::java_into_object(jcall!(self.testHostNamePath).toString()));
        jcall!(dataVector).add(crate::java_into_object(jcall!(self.imagePath).toString()));
        jcall!(dataVector).add(crate::java_into_object(jcall!(self.staticPath).toString()));
        jcall!(dataVector).add(crate::java_into_object(jcall!(self.categoryPath).toString()));
        jcall!(dataVector).add(crate::java_into_object(self.inventoryControl.clone()));
        let mut contextConfigurationDomDocumentMapping: Option<Rc<RefCell<ContextConfigurationDomDocumentMapping>>> = ContextConfigurationDomDocumentMapping::new(self.getContextConfigurationInterface());
        jcall!(dataVector).add(crate::java_into_object(jcall!(contextConfigurationDomDocumentMapping).toDomDocumentString()));
        jcall!(dataVector).add(crate::java_into_object(self.subStores.clone()));
        jcall!(dataVector).add(crate::java_into_object(self.tagLocation.clone()));
        jcall!(dataVector).add(crate::java_into_object(self.packageLocation.clone()));
        jcall!(dataVector).add(crate::java_into_object(self.ftp.clone()));
        jcall!(dataVector).add(crate::java_into_object(jcall!(self.ftpPath).toString()));
        jcall!(dataVector).add(crate::java_into_object(self.ftpUserName.clone()));
        jcall!(dataVector).add(crate::java_into_object(self.ftpPassword.clone()));
        jcall!(dataVector).add(crate::java_into_object(self.testFtp.clone()));
        jcall!(dataVector).add(crate::java_into_object(jcall!(self.testFtpPath).toString()));
        jcall!(dataVector).add(crate::java_into_object(self.testFtpUserName.clone()));
        jcall!(dataVector).add(crate::java_into_object(self.testFtpPassword.clone()));
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut time: String = (Some((jcall!(calendar).getTimeInMillis()) as i64)).expect("NullPointerException").to_string().clone();
        jcall!(dataVector).add(crate::java_into_object(time.clone()));
        jcall!(dataVector).add(crate::java_into_object(time.clone()));
        return dataVector.clone();
    }
    

    // throws Exception
    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        let mut storeFrontData: Option<Rc<RefCell<StoreFrontData>>> = StoreFrontData::getInstance();
        let mut dataHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).NAME.clone(); java_field }.clone()), crate::java_into_object(self.name.clone()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).HOMEHOSTNAME.clone(); java_field }.clone()), crate::java_into_object(self.homeHostName.clone()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).HOMEHOSTNAMEPATH.clone(); java_field }.clone()), crate::java_into_object(jcall!(self.homeHostNamePath).toString()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).HOSTNAME.clone(); java_field }.clone()), crate::java_into_object(self.hostName.clone()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).HOSTNAMEPATH.clone(); java_field }.clone()), crate::java_into_object(jcall!(self.hostNamePath).toString()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTHOMEHOSTNAME.clone(); java_field }.clone()), crate::java_into_object(self.testHomeHostName.clone()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTHOMEHOSTNAMEPATH.clone(); java_field }.clone()), crate::java_into_object(jcall!(self.testHomeHostNamePath).toString()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTHOSTNAME.clone(); java_field }.clone()), crate::java_into_object(self.testHostName.clone()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTHOSTNAMEPATH.clone(); java_field }.clone()), crate::java_into_object(jcall!(self.testHostNamePath).toString()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).IMAGEPATH.clone(); java_field }.clone()), crate::java_into_object(jcall!(self.imagePath).toString()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).STATICPATH.clone(); java_field }.clone()), crate::java_into_object(jcall!(self.staticPath).toString()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).CATEGORYPATH.clone(); java_field }.clone()), crate::java_into_object(jcall!(self.categoryPath).toString()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).INVENTORYCONTROL.clone(); java_field }.clone()), crate::java_into_object(self.inventoryControl.clone()));
        let mut contextConfigurationDomDocumentMapping: Option<Rc<RefCell<ContextConfigurationDomDocumentMapping>>> = ContextConfigurationDomDocumentMapping::new(self.getContextConfigurationInterface());
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).CONFIGURATION.clone(); java_field }.clone()), crate::java_into_object(jcall!(contextConfigurationDomDocumentMapping).toDomDocumentString()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).SUBSTORES.clone(); java_field }.clone()), crate::java_into_object(self.subStores.clone()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).TAGLOCATION.clone(); java_field }.clone()), crate::java_into_object(self.tagLocation.clone()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).PACKAGELOCATION.clone(); java_field }.clone()), crate::java_into_object(self.packageLocation.clone()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).FTP.clone(); java_field }.clone()), crate::java_into_object(self.ftp.clone()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).FTPPATH.clone(); java_field }.clone()), crate::java_into_object(jcall!(self.ftpPath).toString()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).FTPUSERNAME.clone(); java_field }.clone()), crate::java_into_object(self.ftpUserName.clone()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).FTPPASSWORD.clone(); java_field }.clone()), crate::java_into_object(self.ftpPassword.clone()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTFTP.clone(); java_field }.clone()), crate::java_into_object(self.testFtp.clone()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTFTPPATH.clone(); java_field }.clone()), crate::java_into_object(jcall!(self.testFtpPath).toString()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTFTPUSERNAME.clone(); java_field }.clone()), crate::java_into_object(self.testFtpUserName.clone()));
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTFTPPASSWORD.clone(); java_field }.clone()), crate::java_into_object(self.testFtpPassword.clone()));
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut time: String = (Some((jcall!(calendar).getTimeInMillis()) as i64)).expect("NullPointerException").to_string().clone();
        jcall!(dataHashMap).put(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).LASTMODIFIED.clone(); java_field }.clone()), crate::java_into_object(time.clone()));
        return dataHashMap.clone();
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return crate::java_into_object(crate::java_into_object((self.getName()).clone()));
    }
    

    pub fn getContextConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn ContextConfigurationInterface>>> {
        return self.contextConfigurationInterface.clone();
    }
    

    pub fn setContextConfigurationInterface(&mut self, mut contextConfigurationInterface: Option<Rc<RefCell<dyn ContextConfigurationInterface>>>) {
        self.contextConfigurationInterface = contextConfigurationInterface.clone();
    }
}

