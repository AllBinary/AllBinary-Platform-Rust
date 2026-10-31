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
// use crate::java::io::IOException::IOException;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::init::crypt::jcehelper::CryptInterface::CryptInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::xmlrpc::XmlRpcAbeClient::XmlRpcAbeClient;
use crate::org::allbinary::logic::java::exception::ExceptionUtil::ExceptionUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::AbeClientLicense::AbeClientLicense;
use crate::org::allbinary::logic::system::security::licensing::AbeLicenseInterface::AbeLicenseInterface;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::apache::xmlrpc::XmlRpcClient::XmlRpcClient;
use crate::org::apache::xmlrpc::XmlRpcException::XmlRpcException;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct XmlRpcAbeLicenseRetrievalClient {
    pub base: XmlRpcAbeClient,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl XmlRpcAbeLicenseRetrievalClient {
    

    pub fn new_struct(mut clientInfo: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) -> Self {
        let mut this = Self {
            base: XmlRpcAbeClient::new_struct(clientInfo.clone(), String::from("LicServ.getLicense")),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut clientInfo: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(clientInfo))))
    }
    

    // throws Exception
    pub fn get(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut cryptInterface: Option<Rc<RefCell<dyn CryptInterface>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut server: String = jcall!(self.getClientInfo()).getLicenseServer(self.getServer());
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(TRYING.clone());
            jcall!(stringBuffer).appendint(((self.getServer()) as i32));
            jcall!(stringBuffer).append(SEP.clone());
            jcall!(stringBuffer).append(server.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", jcall!(CommonLabels::getInstance()).START_LABEL, jcall!(stringBuffer).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            let mut param: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut xmlRpcClient: Option<Rc<RefCell<dyn crate::org::apache::xmlrpc::XmlRpcClient::XmlRpcClientDyn>>> = XmlRpcClient::new(server.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::apache::xmlrpc::XmlRpcClient::XmlRpcClientDyn>>);
            self.setClient(xmlRpcClient.clone());
            jcall!(xmlRpcClient).setBasicAuthentication(None, None);
            let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(self.getClientInfo()).toHashtable();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", CLIENT_INFO, jcall!(hashtable).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            jcall!(param).add(crate::java_into_object(hashtable.clone()));
            let mut result: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(xmlRpcClient).execute3(self.getRemoteMethod(), param.clone(), cryptInterface.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", RESULT, { let value = jcall!(result); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) }), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            let mut resultHashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = (result).clone() /* as ABHashtable */;
            if !AbeClientLicense::hasRequiredKeys(resultHashtable.clone()) {
                jcall!(self.logUtil).putF(INVALID.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone());
                return self.tryAnother(anyType.clone());
            }
            let mut abeLicenseInterface: Option<Rc<RefCell<dyn AbeLicenseInterface>>> = AbeClientLicense::new(resultHashtable.clone()).map(|reference| reference as Rc<RefCell<dyn AbeLicenseInterface>>);
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", jcall!(self.commonStrings).END, jcall!(stringBuffer).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            self.base.isOnline = true;
            return (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(abeLicenseInterface.clone());
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(EXCEPTION_IN_CLIENT.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(), crate::java_into_object(e.clone()));
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", TRYING_OTHER_SERVERS, jcall!(ExceptionUtil::getInstance()).getStackTrace(e.clone())), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            if !jcall!(e.to_string()).startsWith(HOST_NOT_RESOLVED.clone()) {
                return self.tryAnother(anyType.clone());
            } else {
                self.base.isOnline = false;
                panic!("{:?}", format!("{}: {:?}", "Exception", (HOST_NOT_RESOLVED_MSG.clone())));
            }
            }
        }
    }
}
impl std::ops::Deref for XmlRpcAbeLicenseRetrievalClient {
    type Target = XmlRpcAbeClient;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for XmlRpcAbeLicenseRetrievalClient {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::xmlrpc::XmlRpcAbeClient::XmlRpcAbeClientDyn for XmlRpcAbeLicenseRetrievalClient {
    fn as_XmlRpcAbeClient(&self) -> &crate::org::allbinary::logic::communication::xmlrpc::XmlRpcAbeClient::XmlRpcAbeClient { &self.base }
    fn as_XmlRpcAbeClient_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::xmlrpc::XmlRpcAbeClient::XmlRpcAbeClient { &mut self.base }
}


