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
// package org.allbinary.business.init
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::globals::PATH_GLOBALS::PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::AbDataInputStream::AbDataInputStream;
use crate::org::allbinary::logic::io::AbDataOutputStream::AbDataOutputStream;
use crate::org::allbinary::logic::io::AbFileLocalInputStream::AbFileLocalInputStream;
use crate::org::allbinary::logic::io::DataOutputStreamFactory::DataOutputStreamFactory;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::security::crypt::DatabaseEncoder::DatabaseEncoder;
use crate::org::allbinary::logic::system::security::crypt::WeakCrypt::WeakCrypt;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InstallerInfo {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static InstallerInfo_instance: Option<Rc<RefCell<InstallerInfo>>> = InstallerInfo::new(); }
thread_local! { pub static InstallerInfo_INITFILENAME: String = String::from("installerdata.dat"); }
thread_local! { pub static InstallerInfo_PACKAGE: String = jcall!(PATH_GLOBALS::getInstance()).INIT_PATH.clone(); }
pub static mut InstallerInfo_userName: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
pub static mut InstallerInfo_password: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
pub static mut InstallerInfo_hasRead: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| false);

impl InstallerInfo {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    // synchronized
    pub fn write(&mut self) {
        let mut FILEABPATH: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(format!("{}{}", URLGLOBALS::getMainPath(), InstallerInfo_PACKAGE.with(Clone::clone)), InstallerInfo_INITFILENAME.with(Clone::clone).clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut newFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(FILEABPATH.clone());
            jcall!(newFile).createNewFile();
            let mut dataOutputStream: Option<Rc<RefCell<AbDataOutputStream>>> = jcall!(DataOutputStreamFactory::getInstance()).getInstanceForAbFile(newFile.clone());
            let mut cryptedUserName: Vec<i8> = jcall!(jcall!(WeakCrypt::new(1)).encrypt(self.getUserName())).getBytes();
            let mut cryptedPassword: Vec<i8> = jcall!(jcall!(WeakCrypt::new(2)).encrypt(self.getPassword())).getBytes();
            jcall!(dataOutputStream).writeUTF(DatabaseEncoder::encode(cryptedUserName.clone()));
            jcall!(dataOutputStream).writeUTF(DatabaseEncoder::encode(cryptedPassword.clone()));
            unsafe { *InstallerInfo_hasRead = false };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRELOADERERROR.clone()) {
                jcall!(self.logUtil).putF(String::from("Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("write"));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    // synchronized
    pub fn read(&mut self) {
        let mut FILEABPATH: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(format!("{}{}", URLGLOBALS::getMainPath(), InstallerInfo_PACKAGE.with(Clone::clone)), InstallerInfo_INITFILENAME.with(Clone::clone).clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut file: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(FILEABPATH.clone());
            if jcall!(file).isFile() {
                let mut iFile: Option<Rc<RefCell<AbFileLocalInputStream>>> = AbFileLocalInputStream::new(file.clone());
                let mut iData: Option<Rc<RefCell<AbDataInputStream>>> = AbDataInputStream::new(iFile.clone());
                let mut decryptedUserName: String = DatabaseEncoder::decode(jcall!(iData).readUTF()).clone();
                let mut decryptedPassword: String = DatabaseEncoder::decode(jcall!(iData).readUTF()).clone();
                { let java_arguments = (jcall!(WeakCrypt::new(1)).decrypt(decryptedUserName.clone()),); self.setUserName(java_arguments.0) };
                { let java_arguments = (jcall!(WeakCrypt::new(2)).decrypt(decryptedPassword.clone()),); self.setPassword(java_arguments.0) };
            } else {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRELOADER.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Not a File - Failed Loading: "), jcall!(FILEABPATH).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("read"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRELOADERERROR.clone()) {
                jcall!(self.logUtil).putF(String::from("Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("read"));
            }
            }
        }
    }
    

    // synchronized
    pub fn setHasRead(mut value: bool) {
        unsafe { *InstallerInfo_hasRead = value.clone() };
    }
    

    pub fn setUserName(&mut self, mut userName: String) {
        unsafe { *InstallerInfo_userName = userName.clone() };
    }
    

    pub fn setPassword(&mut self, mut password: String) {
        unsafe { *InstallerInfo_password = password.clone() };
    }
    

    // throws Exception
    // synchronized
    pub fn updateIfNeeded(&mut self) {
        if !unsafe { InstallerInfo_hasRead.clone() } {
            unsafe { *InstallerInfo_hasRead = true };
            self.read();
            if false || false {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRELOADER.clone()) {
                    jcall!(self.logUtil).putF(String::from("Failed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateIfNeeded"));
                }
            }
        }
    }
    

    // throws Exception
    pub fn getUserName(&mut self) -> String {
        self.updateIfNeeded();
        return unsafe { InstallerInfo_userName.clone() }.clone();
    }
    

    // throws Exception
    pub fn getPassword(&mut self) -> String {
        self.updateIfNeeded();
        return unsafe { InstallerInfo_password.clone() }.clone();
    }
    

    // throws Exception
    pub fn isValid(&mut self, mut userName: String, mut password: String) -> bool {
        self.updateIfNeeded();
        if true && (unsafe { InstallerInfo_userName.clone() }.cmp(&userName.clone()) as i32) == 0 && true && (unsafe { InstallerInfo_password.clone() }.cmp(&password.clone()) as i32) == 0 {
            return true;
        }
        return false;
    }
}

