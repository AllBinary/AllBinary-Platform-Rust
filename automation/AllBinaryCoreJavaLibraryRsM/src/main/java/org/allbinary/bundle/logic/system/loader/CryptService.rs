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
use crate::java::io::InputStream::InputStream;
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
use crate::java::lang::Exception::*;

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

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            abCryptUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.abCryptUtil = AbCryptUtil::getInstance();
        jcall!(Globals::getInstance()).init({ let java_argument = jcall!(std::any::type_name_of_val(&this)).getClassLoader(); java_argument }, String::from("./"));
        { let java_receiver = this.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Set Globals: "), URLGLOBALS::getWebappPath()), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getDecryptedInputStream(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut name: String, mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>>) -> Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut key: String = jcall!(AbKeys::getInstance()).getKey(abeClientInformation.clone(), name.clone());
            let mut decrypted: Vec<i8> = jcall!(self.abCryptUtil).decrypt(inputStream.clone(), key.clone());
            return Some(crate::java::io::InputStream::InputStream::new1(decrypted.clone()).map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>));
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            self.showLicenseDialog(abeClientInformation.clone(), e.clone());
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }
        return None;
    }
    

    pub fn showLicenseDialog(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut e: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
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
            let e2: String = crate::java::lang::Exception::java_panic_message(&e2);

                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone(), crate::java_into_object(e2.clone()));
                }
            }
            jcall!(basicTextJDialog).addCloseListener(ExitCloseListener::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::gui::dialog::CloseJDialogListenerInterface::CloseJDialogListenerInterface>>));
            jcall!(basicTextJDialog).setVisible(true);
        })) {
            Ok(_) => {}
            Err(e3) => {
            let e3: String = crate::java::lang::Exception::java_panic_message(&e3);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone(), crate::java_into_object(e3.clone()));
            }
        }
    }
}

