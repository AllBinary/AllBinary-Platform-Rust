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
// package org.allbinary.logic.system.security.licensing.client
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::init::LicenseInitInfo::LicenseInitInfo;
use crate::org::allbinary::business::init::LicenseInitInfoUtil::LicenseInitInfoUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::xmlrpc::XmlRpcAbeClient::XmlRpcAbeClient;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::AbeLicenseInterface::AbeLicenseInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::XmlRpcAbeLicenseRetrievalClient::XmlRpcAbeLicenseRetrievalClient;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AbeLicenseClient {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub MINSERVERS: i32,
}


impl AbeLicenseClient {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            MINSERVERS: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.MINSERVERS = 3;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    // synchronized
    pub fn get(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) -> Option<Rc<RefCell<dyn AbeLicenseInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone());
            let mut xmlRpcAbeLicenseClient: Option<Rc<RefCell<dyn crate::org::allbinary::logic::communication::xmlrpc::XmlRpcAbeClient::XmlRpcAbeClientDyn>>> = XmlRpcAbeLicenseRetrievalClient::new(abeClientInformation.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::communication::xmlrpc::XmlRpcAbeClient::XmlRpcAbeClientDyn>>);
            let mut abeLicenseInterface: Option<Rc<RefCell<dyn AbeLicenseInterface>>> = { let java_object = jcall!(xmlRpcAbeLicenseClient).get(None); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn AbeLicenseInterface>>>>().expect("ClassCastException").clone()) };
            let mut licenseId: String = jcall!(abeLicenseInterface).getLicenseId();
            let mut servers: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(abeLicenseInterface).getServers();
            let mut isNewLicenseId: bool = false;
            let mut isBetterServerList: bool = false;
            let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
            if jcall!(stringValidationUtil).isEmpty(jcall!(abeClientInformation).getLicenseId()) && !jcall!(abeClientInformation).isSameId(licenseId.clone()) {
                isNewLicenseId = true;
            }
            if jcall!(servers).size() >= self.MINSERVERS && jcall!(abeClientInformation).isLargerOrDifferentServerList(servers.clone()) {
                isBetterServerList = true;
            }
            if isBetterServerList || isNewLicenseId {
                let mut initInfo: Option<Rc<RefCell<LicenseInitInfo>>> = jcall!(LicenseInitInfoUtil::getInstance()).read();
                jcall!(initInfo).setLicenseId(licenseId.clone());
                jcall!(initInfo).setServerList(servers.clone());
                jcall!(LicenseInitInfoUtil::getInstance()).write(initInfo.clone());
                jcall!(abeClientInformation).init();
            }
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).END.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone());
            return abeLicenseInterface.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
}

