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
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::globals::PATH_GLOBALS::PATH_GLOBALS;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::AbDataInputStream::AbDataInputStream;
use crate::org::allbinary::logic::io::AbDataOutputStream::AbDataOutputStream;
use crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStream;
use crate::org::allbinary::logic::io::DataOutputStreamFactory::DataOutputStreamFactory;
use crate::org::allbinary::logic::io::FileStreamFactory::FileStreamFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::security::crypt::DatabaseEncoder::DatabaseEncoder;
use crate::org::allbinary::logic::system::security::crypt::WeakCrypt::WeakCrypt;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use super::LicenseInitInfo::LicenseInitInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LicenseInitInfoUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub stringUtil: Option<Rc<RefCell<StringUtil>>>,
    pub INITFILENAME: String,
    pub ABOUT: String,
    pub PRIVACY_POLICY: String,
    pub filePath: String,
}

thread_local! { pub static LicenseInitInfoUtil_instance: Option<Rc<RefCell<LicenseInitInfoUtil>>> = LicenseInitInfoUtil::new(); }

impl LicenseInitInfoUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            stringUtil: None,
            INITFILENAME: String::new(),
            ABOUT: String::new(),
            PRIVACY_POLICY: String::new(),
            filePath: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.stringUtil = StringUtil::getInstance();
        this.INITFILENAME = String::from("licenseinitdata.dat");
        this.ABOUT = String::from("about");
        this.PRIVACY_POLICY = String::from("privacy_policy");
        this.filePath = jcall!(this.stringUtil).EMPTY_STRING.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<LicenseInitInfoUtil>>> {
        return LicenseInitInfoUtil_instance.with(Clone::clone).clone();
    }
    

    // synchronized
    pub fn setFilePath(&mut self, mut filePath: String) {
        self.filePath = filePath.clone();
    }
    

    // throws Exception
    // synchronized
    pub fn write(&mut self, mut initData: Option<Rc<RefCell<LicenseInitInfo>>>) {
        if self.filePath == jcall!(self.stringUtil).EMPTY_STRING {
            self.filePath = format!("{}{}", URLGLOBALS::getMainPath(), jcall!(PATH_GLOBALS::getInstance()).INIT_PATH);
        }
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut tsUtil: Option<Rc<RefCell<TsUtil>>> = TsUtil::getInstance();
            let mut dataOutputStream: Option<Rc<RefCell<AbDataOutputStream>>> = jcall!(DataOutputStreamFactory::getInstance()).getInstance2(self.filePath.clone(), self.INITFILENAME.clone());
            let mut licenseIdCrypted: Vec<i8> = jcall!(tsUtil).getByteArray(jcall!(WeakCrypt::new(1)).encrypt(jcall!(initData).getLicenseId()));
            jcall!(dataOutputStream).writeUTF(DatabaseEncoder::encode(licenseIdCrypted.clone()));
            let mut numberOfLicenseServers: i32 = jcall!(initData).getNumberOfServers();
            jcall!(dataOutputStream).writeInt(numberOfLicenseServers.clone());
            let mut licenseServerCrypted: Vec<i8> = Vec::new();
            for index in (0..numberOfLicenseServers) {
                licenseServerCrypted = jcall!(tsUtil).getByteArray(jcall!(WeakCrypt::new(3)).encrypt(jcall!(initData).getServer(index.clone())));
                jcall!(dataOutputStream).writeUTF(DatabaseEncoder::encode(licenseServerCrypted.clone()));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(format!("{}{}", String::from("Command Failed: "), self.INITFILENAME), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("write"), crate::java_into_object(e.clone()));
            jcall!(FileStreamFactory::getInstance()).delete(self.filePath.clone(), self.INITFILENAME.clone());
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    // synchronized
    pub fn read(&mut self) -> Option<Rc<RefCell<LicenseInitInfo>>> {
        return self.readAgain(0);
    }
    

    // throws Exception
    // synchronized
    pub fn readAgain(&mut self, mut initializeCounter: i32) -> Option<Rc<RefCell<LicenseInitInfo>>> {
        let mut METHOD_NAME: String = String::from("readAgain");
        if self.filePath == jcall!(self.stringUtil).EMPTY_STRING {
            self.filePath = format!("{}{}", URLGLOBALS::getMainPath(), jcall!(PATH_GLOBALS::getInstance()).INIT_PATH);
        }
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("LicenseInitInfo File: "), self.INITFILENAME), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone());
            let mut fileStreamFactory: Option<Rc<RefCell<FileStreamFactory>>> = FileStreamFactory::getInstance();
            let mut iFile: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::AbFileInputStream::AbFileInputStreamDyn>>> = jcall!(fileStreamFactory).getFileInputStreamInstance(self.filePath.clone(), self.INITFILENAME.clone());
            if iFile.is_some() {
                let mut iData: Option<Rc<RefCell<AbDataInputStream>>> = AbDataInputStream::new(iFile.clone());
                let mut initInfo: Option<Rc<RefCell<LicenseInitInfo>>> = LicenseInitInfo::new();
                let mut decodedByteArray: Vec<i8> = DatabaseEncoder::decode(jcall!(iData).readUTF());
                let mut licenseIdDecoded: String = String::from_utf8(decodedByteArray.clone().iter().map(|value| *value as u8).collect()).unwrap();
                jcall!(initInfo).setLicenseId(jcall!(WeakCrypt::new(1)).decrypt(licenseIdDecoded.clone()));
                let mut numberOfLicenseServers: i32 = jcall!(iData).readInt();
                let mut NEXT_FILE: String = jcall!(StringMaker::new()).append(jcall!(CommonLabels::getInstance()).NEXT.clone()).append(String::from(" License Server From File: ")).toString();
                let mut licenseServerDecoded: String = String::new();
                for index in (0..numberOfLicenseServers) {
                    decodedByteArray = DatabaseEncoder::decode(jcall!(iData).readUTF());
                    licenseServerDecoded = String::from_utf8(decodedByteArray.clone().iter().map(|value| *value as u8).collect()).unwrap();
                    jcall!(initInfo).setServer(jcall!(WeakCrypt::new(3)).decrypt(licenseServerDecoded.clone()), index.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", NEXT_FILE, jcall!(initInfo).getServer(index.clone())), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return initInfo.clone();
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Could Not Load License InitInfo: "), self.INITFILENAME))));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                jcall!(self.logUtil).put(format!("{}{}", String::from("Command Failed: "), self.INITFILENAME), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone(), crate::java_into_object(e.clone()));
            })) {
                Ok(_) => {}
                Err(e2) => {
            let e2: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e2 as Box<dyn std::any::Any>)));

                jcall!(self.logUtil).put(format!("{}{}", String::from("LicenseInitInfo Read Retry: "), self.INITFILENAME), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("readAgain()"), crate::java_into_object(e2.clone()));
                }
            }
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("LicenseInitInfo Read Error: "), self.INITFILENAME))));
            }
        }
    }
    

    pub fn getFilePath(&mut self) -> String {
        return self.filePath.clone();
    }
}

