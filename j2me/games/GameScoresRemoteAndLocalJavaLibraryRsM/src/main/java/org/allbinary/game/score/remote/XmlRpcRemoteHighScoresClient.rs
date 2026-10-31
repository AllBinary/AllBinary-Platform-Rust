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
// package org.allbinary.game.score.remote
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
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::apache::xmlrpc::XmlRpcClient::XmlRpcClient;
use crate::org::apache::xmlrpc::XmlRpcException::XmlRpcException;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct XmlRpcRemoteHighScoresClient {
    pub base: XmlRpcAbeClient,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub page: String,
}


impl XmlRpcRemoteHighScoresClient {
    

    pub fn new_struct(mut clientInfo: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut page: String, mut remoteMethod: String) -> Self {
        let mut this = Self {
            base: XmlRpcAbeClient::new_struct(clientInfo.clone(), remoteMethod.clone()),
            logUtil: None,
            page: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.page = page.clone();
        this.setServer(0);
        this
    }

    pub fn new(mut clientInfo: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut page: String, mut remoteMethod: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(clientInfo, page, remoteMethod))))
    }
    

    // throws Exception
    pub fn get(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut cryptInterface: Option<Rc<RefCell<dyn CryptInterface>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut param: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut serverUrl: String = jcall!(self.getClientInfo()).getLicenseServer(self.getServer());
            let mut index: i32 = serverUrl.rfind('/').map(|index| index as i32).unwrap_or(-1);
            serverUrl = format!("{}{}", serverUrl[(0) as usize..(index + 1) as usize].to_string(), self.page);
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(TRYING.clone());
            jcall!(stringBuffer).appendint(((self.getServer()) as i32));
            jcall!(stringBuffer).append(SEP.clone());
            jcall!(stringBuffer).append(serverUrl.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            let mut xmlRpcClient: Option<Rc<RefCell<dyn crate::org::apache::xmlrpc::XmlRpcClient::XmlRpcClientDyn>>> = XmlRpcClient::new(serverUrl.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::apache::xmlrpc::XmlRpcClient::XmlRpcClientDyn>>);
            self.setClient(xmlRpcClient.clone());
            jcall!(xmlRpcClient).setBasicAuthentication(None, None);
            let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = (anyType).clone() /* as ABHashtable */;
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", CLIENT_INFO, jcall!(hashtable).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            jcall!(param).add(crate::java_into_object(hashtable.clone()));
            let mut result: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(xmlRpcClient).execute3(self.getRemoteMethod(), param.clone(), cryptInterface.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", RESULT, { let value = jcall!(result); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) }), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            self.base.isOnline = true;
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", TRYING_OTHER_SERVERS, jcall!(ExceptionUtil::getInstance()).getStackTrace(e.clone())), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            if !jcall!(e.to_string()).startsWith(HOST_NOT_RESOLVED.clone()) {
                return self.tryAnother(anyType.clone());
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (HOST_NOT_RESOLVED_MSG.clone())));
            }
            }
        }
    }
}
impl std::ops::Deref for XmlRpcRemoteHighScoresClient {
    type Target = XmlRpcAbeClient;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for XmlRpcRemoteHighScoresClient {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::xmlrpc::XmlRpcAbeClient::XmlRpcAbeClientDyn for XmlRpcRemoteHighScoresClient {
    fn as_XmlRpcAbeClient(&self) -> &crate::org::allbinary::logic::communication::xmlrpc::XmlRpcAbeClient::XmlRpcAbeClient { &self.base }
    fn as_XmlRpcAbeClient_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::xmlrpc::XmlRpcAbeClient::XmlRpcAbeClient { &mut self.base }
}


