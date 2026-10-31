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
// use crate::java::io::BufferedReader::BufferedReader;
// use crate::java::io::InputStreamReader::InputStreamReader;
use crate::org::allbinary::business::init::LicenseInitInfo::LicenseInitInfo;
use crate::org::allbinary::business::init::LicenseInitInfoUtil::LicenseInitInfoUtil;
use crate::org::allbinary::logic::io::file::FileFactory::FileFactory;
use crate::org::allbinary::globals::PATH_GLOBALS::PATH_GLOBALS;
// use crate::java::io::File::File;
// use crate::java::io::FileReader::FileReader;
// use crate::java::io::LineNumberReader::LineNumberReader;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SetLicenseInitInfo {
    pub vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

pub static mut SetLicenseInitInfo_stdinput: std::cell::LazyCell<Option<Rc<RefCell<BufferedReader>>>> = std::cell::LazyCell::new(|| BufferedReader::new(InputStreamReader::new(unsafe { System_in.clone() }.clone())));

impl SetLicenseInitInfo {
    

    pub fn new_struct(mut options: Vec<String>) -> Self {
        let mut this = Self {
            vector: None,
        };
        this.vector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(LicenseInitInfoUtil::getInstance()).setFilePath(format!("{}{}", PrintLicenseInitInfo_PATH.with(Clone::clone), jcall!(PATH_GLOBALS::getInstance()).INIT_PATH));
        for index in (1..(options.len() as i32)) {
            jcall!(this.vector).add(crate::java_into_object(options[(index) as usize].clone()));
        }
        this
    }

    pub fn new(mut options: Vec<String>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(options))))
    }
    

    // throws Exception
    pub fn getLicenseInitInfo(&mut self) -> Option<Rc<RefCell<LicenseInitInfo>>> {
        let mut licenseInitInfoUtil: Option<Rc<RefCell<LicenseInitInfoUtil>>> = LicenseInitInfoUtil::getInstance();
        if jcall!(FileFactory::getInstance()).isFile(jcall!(licenseInitInfoUtil).INITFILENAME.clone()) {
            return jcall!(licenseInitInfoUtil).read();
        } else {
            return LicenseInitInfo::new();
        }
    }
    

    pub fn createNewLicenseInitInfo(&mut self, mut licenceServerVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if licenceServerVector.is_none() {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("null BasicArrayList Passed - Just Make Empty if you want"))));
            }
            let mut licenseInitInfo: Option<Rc<RefCell<LicenseInitInfo>>> = self.getLicenseInitInfo();
            println!("{}", String::from("License Serv Example: http://www.dartstatistics.com/LicServ/serverssl.php"));
            jcall!(licenseInitInfo).clearServers();
            let mut file: Option<Rc<RefCell<File>>> = File::new(format!("{}{}", PrintLicenseInitInfo_PATH.with(Clone::clone), String::from("licenseservers.txt")));
            let mut reader: Option<Rc<RefCell<FileReader>>> = FileReader::new(file.clone());
            let mut lineNumberReader: Option<Rc<RefCell<LineNumberReader>>> = LineNumberReader::new(reader.clone());
            while true {
                let mut line: String = jcall!(lineNumberReader).readLine();
                if true {
                    jcall!(licenseInitInfo).addServer(line.clone());
                } else {
                    break;
                }
            }
            jcall!(LicenseInitInfoUtil::getInstance()).write(licenseInitInfo.clone());
            jcall!(LicenseInitInfoUtil::getInstance()).read();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            print!("{}", format!("{}{}", String::from("Error: "), e));
            eprintln!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn main(mut args: Vec<String>) {
        let mut initInfo: Option<Rc<RefCell<SetLicenseInitInfo>>> = SetLicenseInitInfo::new(args.clone());
        jcall!(initInfo).createNewLicenseInitInfo(BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
    }
}

