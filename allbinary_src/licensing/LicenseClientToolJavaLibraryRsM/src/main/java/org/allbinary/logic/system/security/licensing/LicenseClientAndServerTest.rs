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
// package org.allbinary.logic.system.security.licensing
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::File::File;
// use crate::java::io::FileReader::FileReader;
// use crate::java::io::LineNumberReader::LineNumberReader;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::business::init::LicenseInitInfo::LicenseInitInfo;
use crate::org::allbinary::business::init::LicenseInitInfoUtil::LicenseInitInfoUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::licensing::client::XmlRpcAbeLicenseRetrievalClient::XmlRpcAbeLicenseRetrievalClient;
use crate::org::allbinary::globals::PATH_GLOBALS::PATH_GLOBALS;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::BasicArrayListD::BasicArrayListD;
use super::AbeClientInformationInterface::AbeClientInformationInterface;
use super::AbeLicenseInterface::AbeLicenseInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LicenseClientAndServerTest {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl LicenseClientAndServerTest {
    

    pub fn new_struct(mut options: Vec<String>) -> Self {
        let mut this = Self {
            logUtil: None,
            vector: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.vector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(LicenseInitInfoUtil::getInstance()).setFilePath(format!("{}{}", PrintLicenseInitInfo_PATH.with(Clone::clone), jcall!(PATH_GLOBALS::getInstance()).INIT_PATH));
        for index in (1..(options.len() as i32)) {
            jcall!(this.vector).add(crate::java_into_object(options[(index) as usize].clone()));
        }
        if (options.len() as i32) < 2 {
            let mut file: Option<Rc<RefCell<File>>> = File::new(format!("{}{}", PrintLicenseInitInfo_PATH.with(Clone::clone), String::from("testlicenseservers.txt")));
            let mut reader: Option<Rc<RefCell<FileReader>>> = FileReader::new(file.clone());
            let mut lineNumberReader: Option<Rc<RefCell<LineNumberReader>>> = LineNumberReader::new(reader.clone());
            while true {
                let mut line: String = jcall!(lineNumberReader).readLine();
                if true {
                    jcall!(this.vector).add(crate::java_into_object(line.clone()));
                } else {
                    break;
                }
            }
        }
        this
    }

    pub fn new(mut options: Vec<String>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(options))))
    }
    

    // synchronized
    pub fn test(&mut self) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut TESTING: String = String::from("Testing with new licence:<br/>");
            let mut PROCESSING: String = String::from("Processing: <br/>");
            let mut END: String = String::from("<br/>End<br/>");
            let mut RESULT: String = String::from("Test Result: ");
            let mut size: i32 = jcall!(self.vector).size();
            for index in (0..size) {
                jcall!(stringBuffer).append(TESTING.clone());
                let mut serverName: String = (jcall!(self.vector).get(index.clone())).clone() /* as String */;
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut licenseInitInfo: Option<Rc<RefCell<LicenseInitInfo>>> = LicenseInitInfo::new();
                    jcall!(licenseInitInfo).addServer(serverName.clone());
                    jcall!(LicenseInitInfoUtil::getInstance()).write(licenseInitInfo.clone());
                    let mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>> = AbeClientInformationInterfaceFactory::AbeClientInformationInterfaceFactory::getInstance();
                    let mut xmlRpcAbeLicenseClient: Option<Rc<RefCell<XmlRpcAbeLicenseRetrievalClient>>> = XmlRpcAbeLicenseRetrievalClient::new(abeClientInformation.clone());
                    let mut abeLicenseInterface: Option<Rc<RefCell<dyn AbeLicenseInterface>>> = { let java_object = jcall!(xmlRpcAbeLicenseClient).get(None); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn AbeLicenseInterface>>>>().expect("ClassCastException").clone()) };
                    if abeLicenseInterface.is_some() {
                        jcall!(stringBuffer).append(PROCESSING.clone());
                        jcall!(stringBuffer).append(jcall!(abeLicenseInterface).toString());
                    }
                    jcall!(stringBuffer).append(END.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", RESULT, jcall!(stringBuffer).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("test"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                    jcall!(list).add(crate::java_into_object(serverName.clone()));
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Test Error: "), jcall!(stringBuffer).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("test"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
                    }
                }
            }
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", format!("{}{}", String::from("Total Servers Failed: "), jcall!(list).size()), String::from(" Failed Servers: ")), jcall!(list).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("test"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Test Error: "), jcall!(stringBuffer).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("test"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
        }
    }
    

    pub fn main(mut args: Vec<String>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut licensingTest: Option<Rc<RefCell<LicenseClientAndServerTest>>> = LicenseClientAndServerTest::new(args.clone());
            jcall!(licensingTest).test();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from("main()")), String::from("main()"), crate::java_into_object(e.clone()));
            }
        }
    }
}

