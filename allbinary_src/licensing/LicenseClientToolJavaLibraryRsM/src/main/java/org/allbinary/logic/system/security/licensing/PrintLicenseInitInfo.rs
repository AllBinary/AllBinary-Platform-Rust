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
use crate::org::allbinary::business::init::LicenseInitInfo::LicenseInitInfo;
use crate::org::allbinary::business::init::LicenseInitInfoUtil::LicenseInitInfoUtil;
use crate::org::allbinary::logic::io::file::FileFactory::FileFactory;
use crate::org::allbinary::globals::PATH_GLOBALS::PATH_GLOBALS;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PrintLicenseInitInfo {
}

thread_local! { pub static PrintLicenseInitInfo_PATH: String = String::from("G:\\mnt\\bc\\mydev\\work\\allbinary_src\\licensing\\data\\"); }

impl PrintLicenseInitInfo {
    

    pub fn new_struct(mut options: Vec<String>) -> Self {
        let mut this = Self {
        };
        jcall!(LicenseInitInfoUtil::getInstance()).setFilePath(format!("{}{}", PrintLicenseInitInfo_PATH.with(Clone::clone), jcall!(PATH_GLOBALS::getInstance()).INIT_PATH));
        this
    }

    pub fn new(mut options: Vec<String>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(options))))
    }
    

    // throws Exception
    pub fn getLicenseInitInfo(&mut self) -> Option<Rc<RefCell<LicenseInitInfo>>> {
        let mut licenseInitInfoUtil: Option<Rc<RefCell<LicenseInitInfoUtil>>> = LicenseInitInfoUtil::getInstance();
        if jcall!(FileFactory::getInstance()).isFile(jcall!(licenseInitInfoUtil).getFilePath()) {
            return jcall!(licenseInitInfoUtil).read();
        } else {
            println!("{}", format!("{}{}", String::from("No Licence File at: "), jcall!(licenseInitInfoUtil).getFilePath()));
            return LicenseInitInfo::new();
        }
    }
    

    pub fn printInitInfo(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut licenseInitInfo: Option<Rc<RefCell<LicenseInitInfo>>> = self.getLicenseInitInfo();
            println!("{}", format!("{}{}", String::from("License ID: "), jcall!(licenseInitInfo).getLicenseId()));
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(licenseInitInfo).getServerList();
            let mut size: i32 = jcall!(list).size();
            for index in (0..size) {
                println!("{}", format!("{}{}", String::from("Lic Serv: "), jcall!(list).get(index.clone())));
            }
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
        let mut initInfo: Option<Rc<RefCell<PrintLicenseInitInfo>>> = PrintLicenseInitInfo::new(args.clone());
        jcall!(initInfo).printInitInfo();
    }
}

