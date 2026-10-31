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
// package org.allbinary.logic.system.security.licensing.registration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::DataInputStream::DataInputStream;
// use crate::java::io::InputStream::InputStream;
// use crate::java::io::OutputStream::OutputStream;
use crate::org::allbinary::logic::io::AbDataOutputStream::AbDataOutputStream;
use crate::org::allbinary::logic::io::FileStreamFactory::FileStreamFactory;
use crate::org::allbinary::logic::io::StreamUtil::StreamUtil;
use crate::org::allbinary::logic::io::file::FileFactory::FileFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RegistrationConfiguration {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub registrationCode: String,
    pub NAME: String,
    pub FILE: String,
}

thread_local! { pub static RegistrationConfiguration_SINGLETON: Option<Rc<RefCell<RegistrationConfiguration>>> = RegistrationConfiguration::new(); }

impl RegistrationConfiguration {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            registrationCode: String::new(),
            NAME: String::new(),
            FILE: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.registrationCode = String::from("No Registration Code");
        this.NAME = String::from("registrationid");
        this.FILE = String::from("Registration.dat");
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(FileFactory::getInstance()).isFile(this.FILE.clone()) {
                this.read();
            } else {
                this.write();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            }
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<RegistrationConfiguration>>> {
        return RegistrationConfiguration_SINGLETON.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn read(&mut self) {
        let mut fileInputStreamFactory: Option<Rc<RefCell<FileStreamFactory>>> = FileStreamFactory::getInstance();
        let mut fileInputStream: Option<Rc<RefCell<InputStream>>> = jcall!(fileInputStreamFactory).getFileInputStreamInstance(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), self.FILE.clone());
        let mut dataInputStream: Option<Rc<RefCell<DataInputStream>>> = DataInputStream::new(fileInputStream.clone());
        { let java_arguments = (jcall!(dataInputStream).readUTF(),); self.setRegistrationCode(java_arguments.0) };
        PreLogUtil::put(jcall!(StringMaker::new()).append(String::from("Read Configuration: ")).append(self.toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("read"));
    }
    

    // throws Exception
    pub fn write(&mut self) {
        let mut dataOutputStream: Option<Rc<RefCell<AbDataOutputStream>>> = None;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Write Configuration: ")).append(self.toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("write"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            let mut fileInputStreamFactory: Option<Rc<RefCell<FileStreamFactory>>> = FileStreamFactory::getInstance();
            let mut fileOutputStream: Option<Rc<RefCell<OutputStream>>> = jcall!(fileInputStreamFactory).getFileOutputStreamInstance(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), self.FILE.clone());
            dataOutputStream = AbDataOutputStream::new(fileOutputStream.clone());
            jcall!(dataOutputStream).writeUTF(self.getRegistrationCode());
            jcall!(dataOutputStream).flush();
        })) {
            Ok(_) => {}
            Err(error) => std::panic::resume_unwind(error),
        }
        { // finally
            jcall!(StreamUtil::getInstance()).close(dataOutputStream.clone());
        }
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringMaker::new()).append(String::from("Registration Code: ")).append(self.getRegistrationCode()).toString();
    }
    

    pub fn setRegistrationCode(&mut self, mut registrationCode: String) {
        self.registrationCode = registrationCode.clone();
    }
    

    pub fn getRegistrationCode(&mut self) -> String {
        return self.registrationCode.clone();
    }
}

