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
// package org.allbinary.bundle.logic.system.loader
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::ByteArrayInputStream::ByteArrayInputStream;
// use crate::java::io::InputStream::InputStream;
use crate::org::allbinary::globals::Globals::Globals;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::gui::dialog::BasicTextJDialog::BasicTextJDialog;
use crate::org::allbinary::gui::dialog::ExitCloseListener::ExitCloseListener;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::security::AbCryptUtil::AbCryptUtil;
use crate::org::allbinary::logic::system::security::AbKeys::AbKeys;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::AbeLicenseInterface::AbeLicenseInterface;
use crate::org::allbinary::logic::system::security::licensing::AbeLicenseInterfaceFactory::AbeLicenseInterfaceFactory;
use crate::org::allbinary::logic::system::security::licensing::AbeNoLicense::AbeNoLicense;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CryptService {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub abCryptUtil: Option<Rc<RefCell<AbCryptUtil>>>,
}


impl CryptService {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            abCryptUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.abCryptUtil = AbCryptUtil::getInstance();
        jcall!(Globals::getInstance()).init(jcall!(std::any::type_name_of_val(&this)).getClassLoader(), String::from("./"));
        { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Set Globals: "), URLGLOBALS::getWebappPath()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getDecryptedInputStream(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut name: String, mut inputStream: Option<Rc<RefCell<InputStream>>>) -> Option<Rc<RefCell<InputStream>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut key: String = jcall!(AbKeys::getInstance()).getKey(abeClientInformation.clone(), name.clone());
            let mut decrypted: Vec<i8> = jcall!(self.abCryptUtil).decrypt(inputStream.clone(), key.clone());
            return ByteArrayInputStream::new(decrypted.clone());
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            self.showLicenseDialog(abeClientInformation.clone(), e.clone());
            }
        }
        return None;
    }
    

    pub fn showLicenseDialog(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut e: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(), crate::java_into_object(e.clone()));
            let mut basicTextJDialog: Option<Rc<RefCell<BasicTextJDialog>>> = BasicTextJDialog::new1(e.to_string());
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut abeLicenseInterface: Option<Rc<RefCell<dyn AbeLicenseInterface>>> = jcall!(AbeLicenseInterfaceFactory::getInstance()).getLicenseInstance(abeClientInformation.clone());
                if !match (&abeLicenseInterface, &AbeNoLicense::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    if jcall!(abeLicenseInterface).isValid() {
                        jcall!(basicTextJDialog).setText(String::from("Subscription Invalid"));
                    }
                }
            })) {
                Ok(_) => {}
                Err(e2) => {
            let e2: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e2 as Box<dyn std::any::Any>)));

                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(), crate::java_into_object(e2.clone()));
                }
            }
            jcall!(basicTextJDialog).addCloseListener(ExitCloseListener::new());
            jcall!(basicTextJDialog).setVisible(true);
        })) {
            Ok(_) => {}
            Err(e3) => {
            let e3: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e3 as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(), crate::java_into_object(e3.clone()));
            }
        }
    }
}

