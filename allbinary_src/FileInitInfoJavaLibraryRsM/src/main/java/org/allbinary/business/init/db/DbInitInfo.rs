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
// package org.allbinary.business.init.db
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::globals::FREEBLISKET_PATH_GLOBALS::FREEBLISKET_PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::communication::http::HttpData::HttpData;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypeFactory::LogConfigTypeFactory;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes;
use crate::org::allbinary::logic::io::AbDataInputStream::AbDataInputStream;
use crate::org::allbinary::logic::io::AbDataOutputStream::AbDataOutputStream;
use crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStream;
use crate::org::allbinary::logic::io::DataOutputStreamFactory::DataOutputStreamFactory;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::directory::Directory::Directory;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::security::crypt::DatabaseEncoder::DatabaseEncoder;
use crate::org::allbinary::logic::system::security::crypt::WeakCrypt::WeakCrypt;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::DbConnectionInfo::DbConnectionInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DbInitInfo {
    pub base: DbConnectionInfo,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub directory: Option<Rc<RefCell<Directory>>>,
    pub httpData: Option<Rc<RefCell<HttpData>>>,
    pub GET_URL: String,
    pub PACKAGE: String,
    pub initFileName: String,
    pub hasRead: bool,
}


impl DbInitInfo {
    

    pub fn new_struct(mut initFileName: String, mut read: bool) -> Self {
        let mut this = Self {
            base: DbConnectionInfo::new_struct(),
            commonStrings: None,
            directory: None,
            httpData: None,
            GET_URL: String::new(),
            PACKAGE: String::new(),
            initFileName: String::new(),
            hasRead: false,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.directory = Directory::getInstance();
        this.httpData = HttpData::getInstance();
        this.GET_URL = String::from("getUrl");
        this.PACKAGE = jcall!(FREEBLISKET_PATH_GLOBALS::getInstance()).DBINITPATH.clone();
        this.hasRead = false;
        // super(...) is in the struct literal above
        this.initFileName = initFileName.clone();
        if read {
            this.updateIfNeeded();
        } else {
            this.setHasRead(true);
        }
        this
    }

    pub fn new(mut initFileName: String, mut read: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(initFileName, read))))
    }
    

    pub fn getUrl(&mut self) -> String {
        let mut url: String = self.base.getUrl();
        if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).PRELOADER.clone())) {
            PreLogUtil::put(format!("{}{}", jcall!(self.httpData).URL_LABEL, url), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.GET_URL.clone());
        }
        return url.clone();
    }
    

    // throws Exception
    pub fn write(&mut self) {
        let mut PATH: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(format!("{}{}", URLGLOBALS::getWebappPath(), self.PACKAGE), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        jcall!(self.directory).create(PATH.clone());
        let mut FILEABPATH: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(format!("{}{}", URLGLOBALS::getWebappPath(), self.PACKAGE), self.initFileName.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut newFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(FILEABPATH.clone());
            jcall!(newFile).createNewFile();
            let mut dataOutputStream: Option<Rc<RefCell<AbDataOutputStream>>> = jcall!(DataOutputStreamFactory::getInstance()).getInstanceForAbFile(newFile.clone());
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut cryptedJdbcDriver: Vec<i8> = jcall!(jcall!(WeakCrypt::new(1)).encrypt(self.getJdbcDriver())).getBytes();
                let mut cryptedName: Vec<i8> = jcall!(jcall!(WeakCrypt::new(2)).encrypt(self.getName())).getBytes();
                let mut cryptedUserName: Vec<i8> = jcall!(jcall!(WeakCrypt::new(3)).encrypt(self.getUserName())).getBytes();
                let mut cryptedPassword: Vec<i8> = jcall!(jcall!(WeakCrypt::new(4)).encrypt(self.getPassword())).getBytes();
                let mut cryptedSchema: Vec<i8> = jcall!(jcall!(WeakCrypt::new(5)).encrypt(self.getSchema())).getBytes();
                let mut cryptedServer: Vec<i8> = jcall!(jcall!(WeakCrypt::new(6)).encrypt(self.getServer())).getBytes();
                let mut cryptedPort: Vec<i8> = jcall!(jcall!(WeakCrypt::new(7)).encrypt(self.getPort())).getBytes();
                jcall!(dataOutputStream).writeUTF(DatabaseEncoder::encode(cryptedJdbcDriver.clone()));
                jcall!(dataOutputStream).writeUTF(DatabaseEncoder::encode(cryptedName.clone()));
                jcall!(dataOutputStream).writeUTF(DatabaseEncoder::encode(cryptedUserName.clone()));
                jcall!(dataOutputStream).writeUTF(DatabaseEncoder::encode(cryptedPassword.clone()));
                jcall!(dataOutputStream).writeUTF(DatabaseEncoder::encode(cryptedSchema.clone()));
                jcall!(dataOutputStream).writeUTF(DatabaseEncoder::encode(cryptedServer.clone()));
                jcall!(dataOutputStream).writeUTF(DatabaseEncoder::encode(cryptedPort.clone()));
                self.hasRead = false;
            })) {
                Ok(_) => {}
                Err(error) => std::panic::resume_unwind(error),
            }
            { // finally
                jcall!(StreamUtil::getInstance()).close(dataOutputStream.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).PRELOADERERROR.clone())) {
                PreLogUtil::putOE(format!("{}{}", String::from("Failed Write: "), jcall!(FILEABPATH).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("write"), e.clone());
            }
            }
        }
    }
    

    // throws Exception
    pub fn load(&mut self) {
        let mut FILEABPATH: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(format!("{}{}", URLGLOBALS::getWebappPath(), self.PACKAGE), self.initFileName.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(FILEABPATH.clone());
            if jcall!(file).isFile() {
                let mut iFile: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>> = AbFileInputStream::new1(file.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>);
                let mut iData: Option<Rc<RefCell<AbDataInputStream>>> = AbDataInputStream::new(iFile.clone());
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut decryptedJdbcDriver: String = DatabaseEncoder::decode(jcall!(iData).readUTF()).clone();
                    let mut decryptedName: String = DatabaseEncoder::decode(jcall!(iData).readUTF()).clone();
                    let mut decryptedUserName: String = DatabaseEncoder::decode(jcall!(iData).readUTF()).clone();
                    let mut decryptedPassword: String = DatabaseEncoder::decode(jcall!(iData).readUTF()).clone();
                    let mut decryptedSchema: String = DatabaseEncoder::decode(jcall!(iData).readUTF()).clone();
                    let mut decryptedServer: String = DatabaseEncoder::decode(jcall!(iData).readUTF()).clone();
                    let mut decryptedPort: String = DatabaseEncoder::decode(jcall!(iData).readUTF()).clone();
                    { let java_arguments = (jcall!(WeakCrypt::new(1)).decrypt(decryptedJdbcDriver.clone()),); self.setJdbcDriver(java_arguments.0) };
                    { let java_arguments = (jcall!(WeakCrypt::new(2)).decrypt(decryptedName.clone()),); self.setName(java_arguments.0) };
                    { let java_arguments = (jcall!(WeakCrypt::new(3)).decrypt(decryptedUserName.clone()),); self.setUserName(java_arguments.0) };
                    { let java_arguments = (jcall!(WeakCrypt::new(4)).decrypt(decryptedPassword.clone()),); self.setPassword(java_arguments.0) };
                    { let java_arguments = (jcall!(WeakCrypt::new(5)).decrypt(decryptedSchema.clone()),); self.setSchema(java_arguments.0) };
                    { let java_arguments = (jcall!(WeakCrypt::new(6)).decrypt(decryptedServer.clone()),); self.setServer(java_arguments.0) };
                    { let java_arguments = (jcall!(WeakCrypt::new(7)).decrypt(decryptedPort.clone()),); self.setPort(java_arguments.0) };
                    let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
                })) {
                    Ok(_) => {}
                    Err(error) => std::panic::resume_unwind(error),
                }
                { // finally
                    jcall!(StreamUtil::getInstance()).close(iData.clone());
                }
            } else {
                self.hasRead = false;
                if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).PRELOADER.clone())) {
                    PreLogUtil::put(format!("{}{}", String::from("Not a File - Failed Loading: "), jcall!(FILEABPATH).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).LOAD.clone());
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).PRELOADERERROR.clone())) {
                PreLogUtil::putOE(format!("{}{}", String::from("Failed Loading: "), jcall!(FILEABPATH).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).LOAD.clone(), e.clone());
            }
            }
        }
    }
    

    pub fn setHasRead(&mut self, mut value: bool) {
        self.hasRead = value.clone();
    }
    

    pub fn updateIfNeeded(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if !self.hasRead {
                self.hasRead = true;
                self.load();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).PRELOADERERROR.clone())) {
                PreLogUtil::putOE(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateIfNeeded"), e.clone());
            }
            }
        }
    }
    

    pub fn getName(&mut self) -> String {
        self.updateIfNeeded();
        if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).PRELOADER.clone())) {
            PreLogUtil::put(format!("{}{}", String::from("Name: "), self.base.getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getName"));
        }
        return self.base.getName();
    }
    

    pub fn getUserName(&mut self) -> String {
        self.updateIfNeeded();
        if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object(jcall!(LogConfigTypeFactory::getInstance()).PRELOADER.clone())) {
            PreLogUtil::put(format!("{}{}", String::from("Name: "), self.base.getUserName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getUserName"));
        }
        return self.base.getUserName();
    }
    

    pub fn getPassword(&mut self) -> String {
        self.updateIfNeeded();
        return self.base.getPassword();
    }
    

    pub fn getSchema(&mut self) -> String {
        self.updateIfNeeded();
        return self.base.getSchema();
    }
    

    pub fn getServer(&mut self) -> String {
        self.updateIfNeeded();
        return self.base.getServer();
    }
    

    pub fn getPort(&mut self) -> String {
        self.updateIfNeeded();
        return self.base.getPort();
    }
}
impl std::ops::Deref for DbInitInfo {
    type Target = DbConnectionInfo;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DbInitInfo {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait DbInitInfoDyn: crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn {
    fn as_DbInitInfo(&self) -> &DbInitInfo;
    fn as_DbInitInfo_mut(&mut self) -> &mut DbInitInfo;
    fn write(&mut self) { DbInitInfo::write(self.as_DbInitInfo_mut()) }
    fn load(&mut self) { DbInitInfo::load(self.as_DbInitInfo_mut()) }
    fn setHasRead(&mut self, mut value: bool) { DbInitInfo::setHasRead(self.as_DbInitInfo_mut(), value) }
    fn updateIfNeeded(&mut self) { DbInitInfo::updateIfNeeded(self.as_DbInitInfo_mut()) }
    fn write(&mut self) { DbInitInfo::write(self.as_DbInitInfo_mut()) }
    fn load(&mut self) { DbInitInfo::load(self.as_DbInitInfo_mut()) }
    fn setHasRead(&mut self, mut value: bool) { DbInitInfo::setHasRead(self.as_DbInitInfo_mut(), value) }
}
impl DbInitInfoDyn for DbInitInfo {
    fn as_DbInitInfo(&self) -> &DbInitInfo { self }
    fn as_DbInitInfo_mut(&mut self) -> &mut DbInitInfo { self }
}

impl crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn for DbInitInfo {
    fn as_DbConnectionInfo(&self) -> &crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo { &self.base }
    fn as_DbConnectionInfo_mut(&mut self) -> &mut crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo { &mut self.base }
}
impl crate::org::allbinary::business::init::db::DatabaseConnectionInfoInterface::DatabaseConnectionInfoInterface for DbInitInfo {
    fn getUrl(&mut self) -> String { Self::getUrl(self) }
    fn getJdbcDriver(&mut self) -> String { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::getJdbcDriver(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self)) }
    fn getName(&mut self) -> String { Self::getName(self) }
    fn getUserName(&mut self) -> String { Self::getUserName(self) }
    fn getPassword(&mut self) -> String { Self::getPassword(self) }
    fn getHost(&mut self) -> String { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::getHost(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self)) }
    fn getSchema(&mut self) -> String { Self::getSchema(self) }
    fn getServer(&mut self) -> String { Self::getServer(self) }
    fn getPort(&mut self) -> String { Self::getPort(self) }
    fn setJdbcDriver(&mut self, p1: String) { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::setJdbcDriver(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self), p1) }
    fn setName(&mut self, p1: String) { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::setName(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self), p1) }
    fn setUserName(&mut self, p1: String) { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::setUserName(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self), p1) }
    fn setPassword(&mut self, p1: String) { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::setPassword(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self), p1) }
    fn setSchema(&mut self, p1: String) { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::setSchema(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self), p1) }
    fn setServer(&mut self, p1: String) { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::setServer(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self), p1) }
    fn setPort(&mut self, p1: String) { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::setPort(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self), p1) }
    fn getUserNameKey(&mut self) -> String { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::getUserNameKey(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self)) }
    fn getPasswordKey(&mut self) -> String { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::getPasswordKey(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self)) }
}


