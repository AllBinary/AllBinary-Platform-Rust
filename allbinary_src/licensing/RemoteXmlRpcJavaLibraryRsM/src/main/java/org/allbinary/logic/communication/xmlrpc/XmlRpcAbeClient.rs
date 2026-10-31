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
// package org.allbinary.logic.communication.xmlrpc
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::rand::MyRandomFactory::MyRandomFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::apache::xmlrpc::NullXmlRpcHandler::NullXmlRpcHandler;
use crate::org::apache::xmlrpc::XmlRpcClient::XmlRpcClient;
use crate::org::apache::xmlrpc::XmlRpcHandler::XmlRpcHandler;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct XmlRpcAbeClient {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub remoteMethod: String,
    pub clientInfo: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub client: Option<Rc<RefCell<dyn XmlRpcHandler>>>,
    pub server: i32,
    pub start: i32,
    pub maxServers: i32,
    pub isDone: bool,
    pub START_SERVER: String,
    pub TRYING: String,
    pub SEP: String,
    pub CLIENT_INFO: String,
    pub RESULT: String,
    pub INVALID: String,
    pub EXCEPTION_IN_CLIENT: String,
    pub SERVER_REPORTED_ERROR: String,
    pub UNKNOWN_ERROR: String,
    pub TRYING_OTHER_SERVERS: String,
    pub HOST_NOT_RESOLVED_MSG: String,
    pub HOST_NOT_RESOLVED: String,
    pub myRandomFactory: Option<Rc<RefCell<MyRandomFactory>>>,
}

pub static mut XmlRpcAbeClient_isOnline: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| true);

impl XmlRpcAbeClient {
    

    pub fn new_struct(mut clientInfo: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut remoteMethod: String) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            remoteMethod: String::new(),
            clientInfo: None,
            client: None,
            server: 0,
            start: 0,
            maxServers: 0,
            isDone: false,
            START_SERVER: String::new(),
            TRYING: String::new(),
            SEP: String::new(),
            CLIENT_INFO: String::new(),
            RESULT: String::new(),
            INVALID: String::new(),
            EXCEPTION_IN_CLIENT: String::new(),
            SERVER_REPORTED_ERROR: String::new(),
            UNKNOWN_ERROR: String::new(),
            TRYING_OTHER_SERVERS: String::new(),
            HOST_NOT_RESOLVED_MSG: String::new(),
            HOST_NOT_RESOLVED: String::new(),
            myRandomFactory: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.client = crate::org::apache::xmlrpc::NullXmlRpcHandler::NullXmlRpcHandler_NULL_XML_RPC_HANDLER.with(Clone::clone).clone();
        this.START_SERVER = String::from("Start With Server #");
        this.TRYING = String::from("Trying Server #");
        this.SEP = jcall!(CommonLabels::getInstance()).COLON_SEP.clone();
        this.CLIENT_INFO = String::from("Client Info: \n");
        this.RESULT = format!("{}{}", jcall!(CommonLabels::getInstance()).RESULT_, jcall!(CommonSeps::getInstance()).NEW_LINE);
        this.INVALID = String::from("License data is Invalid Trying Other Servers");
        this.EXCEPTION_IN_CLIENT = String::from("Exception in client");
        this.SERVER_REPORTED_ERROR = String::from("Server reported error");
        this.UNKNOWN_ERROR = String::from("Unknown License Retrieval Failure");
        this.TRYING_OTHER_SERVERS = String::from("IOException Trying Other Servers");
        this.HOST_NOT_RESOLVED_MSG = String::from("Not Trying Again Since Host Unresolved");
        this.HOST_NOT_RESOLVED = String::from("Host is unresolved");
        this.myRandomFactory = MyRandomFactory::getInstance();
        this.remoteMethod = remoteMethod.clone();
        this.clientInfo = clientInfo.clone();
        if jcall!(clientInfo).getNumberOfLicenseServers() > 1 {
            this.maxServers = jcall!(clientInfo).getNumberOfLicenseServers() - 2;
            this.start = jcall!(this.myRandomFactory).getAbsoluteNextInt(this.maxServers.clone()) + 1;
        } else if jcall!(clientInfo).getNumberOfLicenseServers() == 1 {
            this.maxServers = 0;
            this.start = 0;
        }
        this.isDone = false;
        this.server = this.start.clone();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(this.START_SERVER.clone());
        jcall!(stringBuffer).appendint(this.getServer());
        jcall!(stringBuffer).append(this.SEP.clone());
        jcall!(stringBuffer).append(jcall!(clientInfo).getLicenseServer(this.getServer()));
        { let java_receiver = this.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        this
    }

    pub fn new(mut clientInfo: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut remoteMethod: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(clientInfo, remoteMethod))))
    }
    

    // throws Exception
    pub fn get(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        ForcedLogUtil::log(jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
        return jcall!(NullUtil::getInstance()).NULL_OBJECT.clone();
    }
    

    // throws Exception
    pub fn tryAnother(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        if self.getServer() < self.getMaxServers() {
            { let java_arguments = (self.getServer() + 1,); self.setServer(java_arguments.0) };
        } else {
            self.setServer(0);
        }
        if self.getServer() != self.getStart() && !self.isIsDone() {
            return self.get(anyType.clone());
        } else {
            self.setIsDone(true);
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Tried All Servers But Still Failed"))));
    }
    

    pub fn getClientInfo(&mut self) -> Option<Rc<RefCell<dyn AbeClientInformationInterface>>> {
        return self.clientInfo.clone();
    }
    

    pub fn getClient(&mut self) -> Option<Rc<RefCell<dyn XmlRpcHandler>>> {
        return self.client.clone();
    }
    

    pub fn setClient(&mut self, mut client: Option<Rc<RefCell<dyn crate::org::apache::xmlrpc::XmlRpcClient::XmlRpcClientDyn>>>) {
        self.client = client.clone();
    }
    

    pub fn getServer(&mut self) -> i32 {
        return self.server.clone();
    }
    

    pub fn setServer(&mut self, mut server: i32) {
        self.server = server.clone();
    }
    

    pub fn getStart(&mut self) -> i32 {
        return self.start.clone();
    }
    

    pub fn setStart(&mut self, mut start: i32) {
        self.start = start.clone();
    }
    

    pub fn getMaxServers(&mut self) -> i32 {
        return self.maxServers.clone();
    }
    

    pub fn setMaxServers(&mut self, mut maxServers: i32) {
        self.maxServers = maxServers.clone();
    }
    

    pub fn isIsDone(&mut self) -> bool {
        return self.isDone.clone();
    }
    

    pub fn setIsDone(&mut self, mut isDone: bool) {
        self.isDone = isDone.clone();
    }
    

    pub fn getRemoteMethod(&mut self) -> String {
        return self.remoteMethod.clone();
    }
}
pub trait XmlRpcAbeClientDyn {
    fn as_XmlRpcAbeClient(&self) -> &XmlRpcAbeClient;
    fn as_XmlRpcAbeClient_mut(&mut self) -> &mut XmlRpcAbeClient;
    fn get(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { XmlRpcAbeClient::get(self.as_XmlRpcAbeClient_mut(), anyType) }
    fn tryAnother(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { XmlRpcAbeClient::tryAnother(self.as_XmlRpcAbeClient_mut(), anyType) }
    fn getClientInfo(&mut self) -> Option<Rc<RefCell<dyn AbeClientInformationInterface>>> { XmlRpcAbeClient::getClientInfo(self.as_XmlRpcAbeClient_mut()) }
    fn getClient(&mut self) -> Option<Rc<RefCell<dyn XmlRpcHandler>>> { XmlRpcAbeClient::getClient(self.as_XmlRpcAbeClient_mut()) }
    fn setClient(&mut self, mut client: Option<Rc<RefCell<dyn crate::org::apache::xmlrpc::XmlRpcClient::XmlRpcClientDyn>>>) { XmlRpcAbeClient::setClient(self.as_XmlRpcAbeClient_mut(), client) }
    fn getServer(&mut self) -> i32 { XmlRpcAbeClient::getServer(self.as_XmlRpcAbeClient_mut()) }
    fn setServer(&mut self, mut server: i32) { XmlRpcAbeClient::setServer(self.as_XmlRpcAbeClient_mut(), server) }
    fn getStart(&mut self) -> i32 { XmlRpcAbeClient::getStart(self.as_XmlRpcAbeClient_mut()) }
    fn setStart(&mut self, mut start: i32) { XmlRpcAbeClient::setStart(self.as_XmlRpcAbeClient_mut(), start) }
    fn getMaxServers(&mut self) -> i32 { XmlRpcAbeClient::getMaxServers(self.as_XmlRpcAbeClient_mut()) }
    fn setMaxServers(&mut self, mut maxServers: i32) { XmlRpcAbeClient::setMaxServers(self.as_XmlRpcAbeClient_mut(), maxServers) }
    fn isIsDone(&mut self) -> bool { XmlRpcAbeClient::isIsDone(self.as_XmlRpcAbeClient_mut()) }
    fn setIsDone(&mut self, mut isDone: bool) { XmlRpcAbeClient::setIsDone(self.as_XmlRpcAbeClient_mut(), isDone) }
    fn getRemoteMethod(&mut self) -> String { XmlRpcAbeClient::getRemoteMethod(self.as_XmlRpcAbeClient_mut()) }
}
impl XmlRpcAbeClientDyn for XmlRpcAbeClient {
    fn as_XmlRpcAbeClient(&self) -> &XmlRpcAbeClient { self }
    fn as_XmlRpcAbeClient_mut(&mut self) -> &mut XmlRpcAbeClient { self }
}


