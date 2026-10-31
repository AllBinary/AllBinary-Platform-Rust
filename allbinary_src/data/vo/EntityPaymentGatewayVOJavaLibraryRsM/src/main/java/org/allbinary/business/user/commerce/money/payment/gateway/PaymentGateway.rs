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
// package org.allbinary.business.user.commerce.money.payment.gateway
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::control::crypt::SuperCrypt::SuperCrypt;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::PaymentGatewayInterface::PaymentGatewayInterface;
use super::PaymentGatewayPrimaryKey::PaymentGatewayPrimaryKey;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaymentGateway {
    pub storeName: String,
    pub name: String,
    pub enable: String,
    pub paymentMethod: String,
    pub mode: String,
    pub testProtocol: String,
    pub testServer: String,
    pub testPort: String,
    pub testPath: String,
    pub serverProtocol: String,
    pub server: String,
    pub serverPort: String,
    pub serverPath: String,
    pub timeout: String,
    pub userName: String,
    pub password: String,
    pub proxyProtocol: String,
    pub proxyServer: String,
    pub proxyPort: String,
    pub proxyPath: String,
    pub proxyUserName: String,
    pub proxyPassword: String,
    pub proxyTimeout: String,
    pub special1: String,
    pub special2: String,
    pub special3: String,
    pub special4: String,
    pub special5: String,
    pub special6: String,
    pub special7: String,
    pub special8: String,
    pub special9: String,
    pub lastModified: String,
    pub timeEntered: String,
}

impl PaymentGatewayInterface for PaymentGateway {
    fn getEnable(&mut self) -> String { Self::getEnable(self) }
    fn getStoreName(&mut self) -> String { Self::getStoreName(self) }
    fn getName(&mut self) -> String { Self::getName(self) }
    fn getPaymentMethod(&mut self) -> String { Self::getPaymentMethod(self) }
    fn getMode(&mut self) -> String { Self::getMode(self) }
    fn getTestProtocol(&mut self) -> String { Self::getTestProtocol(self) }
    fn getTestServer(&mut self) -> String { Self::getTestServer(self) }
    fn getTestPort(&mut self) -> String { Self::getTestPort(self) }
    fn getTestPath(&mut self) -> String { Self::getTestPath(self) }
    fn getServerProtocol(&mut self) -> String { Self::getServerProtocol(self) }
    fn getServer(&mut self) -> String { Self::getServer(self) }
    fn getServerPort(&mut self) -> String { Self::getServerPort(self) }
    fn getServerPath(&mut self) -> String { Self::getServerPath(self) }
    fn getUserName(&mut self) -> String { Self::getUserName(self) }
    fn getPassword(&mut self) -> String { Self::getPassword(self) }
    fn getProxyProtocol(&mut self) -> String { Self::getProxyProtocol(self) }
    fn getProxyServer(&mut self) -> String { Self::getProxyServer(self) }
    fn getProxyPath(&mut self) -> String { Self::getProxyPath(self) }
    fn getProxyPort(&mut self) -> String { Self::getProxyPort(self) }
    fn getProxyUserName(&mut self) -> String { Self::getProxyUserName(self) }
    fn getProxyPassword(&mut self) -> String { Self::getProxyPassword(self) }
    fn getTimeout(&mut self) -> String { Self::getTimeout(self) }
    fn getProxyTimeout(&mut self) -> String { Self::getProxyTimeout(self) }
    fn getSpecial1(&mut self) -> String { Self::getSpecial1(self) }
    fn getSpecial2(&mut self) -> String { Self::getSpecial2(self) }
    fn getSpecial3(&mut self) -> String { Self::getSpecial3(self) }
    fn getSpecial4(&mut self) -> String { Self::getSpecial4(self) }
    fn getSpecial5(&mut self) -> String { Self::getSpecial5(self) }
    fn getSpecial6(&mut self) -> String { Self::getSpecial6(self) }
    fn getSpecial7(&mut self) -> String { Self::getSpecial7(self) }
    fn getSpecial8(&mut self) -> String { Self::getSpecial8(self) }
    fn getSpecial9(&mut self) -> String { Self::getSpecial9(self) }
    fn getLastModified(&mut self) -> String { Self::getLastModified(self) }
    fn getTimeEntered(&mut self) -> String { Self::getTimeEntered(self) }
    fn setEnable(&mut self, mut value: String) { Self::setEnable(self, value) }
    fn setStoreName(&mut self, mut value: String) { Self::setStoreName(self, value) }
    fn setName(&mut self, mut value: String) { Self::setName(self, value) }
    fn setPaymentMethod(&mut self, mut value: String) { Self::setPaymentMethod(self, value) }
    fn setMode(&mut self, mut value: String) { Self::setMode(self, value) }
    fn setTestProtocol(&mut self, mut value: String) { Self::setTestProtocol(self, value) }
    fn setTestServer(&mut self, mut value: String) { Self::setTestServer(self, value) }
    fn setTestPort(&mut self, mut value: String) { Self::setTestPort(self, value) }
    fn setTestPath(&mut self, mut value: String) { Self::setTestPath(self, value) }
    fn setServerProtocol(&mut self, mut value: String) { Self::setServerProtocol(self, value) }
    fn setServer(&mut self, mut value: String) { Self::setServer(self, value) }
    fn setServerPort(&mut self, mut value: String) { Self::setServerPort(self, value) }
    fn setServerPath(&mut self, mut value: String) { Self::setServerPath(self, value) }
    fn setUserName(&mut self, mut value: String) { Self::setUserName(self, value) }
    fn setPassword(&mut self, mut value: String) { Self::setPassword(self, value) }
    fn setProxyProtocol(&mut self, mut value: String) { Self::setProxyProtocol(self, value) }
    fn setProxyServer(&mut self, mut value: String) { Self::setProxyServer(self, value) }
    fn setProxyPath(&mut self, mut value: String) { Self::setProxyPath(self, value) }
    fn setProxyPort(&mut self, mut value: String) { Self::setProxyPort(self, value) }
    fn setProxyUserName(&mut self, mut value: String) { Self::setProxyUserName(self, value) }
    fn setProxyPassword(&mut self, mut value: String) { Self::setProxyPassword(self, value) }
    fn setTimeout(&mut self, mut value: String) { Self::setTimeout(self, value) }
    fn setProxyTimeout(&mut self, mut value: String) { Self::setProxyTimeout(self, value) }
    fn setSpecial1(&mut self, mut value: String) { Self::setSpecial1(self, value) }
    fn setSpecial2(&mut self, mut value: String) { Self::setSpecial2(self, value) }
    fn setSpecial3(&mut self, mut value: String) { Self::setSpecial3(self, value) }
    fn setSpecial4(&mut self, mut value: String) { Self::setSpecial4(self, value) }
    fn setSpecial5(&mut self, mut value: String) { Self::setSpecial5(self, value) }
    fn setSpecial6(&mut self, mut value: String) { Self::setSpecial6(self, value) }
    fn setSpecial7(&mut self, mut value: String) { Self::setSpecial7(self, value) }
    fn setSpecial8(&mut self, mut value: String) { Self::setSpecial8(self, value) }
    fn setSpecial9(&mut self, mut value: String) { Self::setSpecial9(self, value) }
    fn setLastModified(&mut self, mut value: String) { Self::setLastModified(self, value) }
    fn setTimeEntered(&mut self, mut value: String) { Self::setTimeEntered(self, value) }
    fn toHashMap(&mut self, mut isEncrypted: bool) -> Option<Rc<RefCell<HashMap>>> { Self::toHashMap1(self, isEncrypted) }
    fn toVector(&mut self, mut isEncrypted: bool) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toVector1(self, isEncrypted) }
}
impl crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface for PaymentGateway {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toVector(self) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::toHashMap(self) }
}

impl PaymentGateway {
    

    pub fn new_struct(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            storeName: String::new(),
            name: String::new(),
            enable: String::new(),
            paymentMethod: String::new(),
            mode: String::new(),
            testProtocol: String::new(),
            testServer: String::new(),
            testPort: String::new(),
            testPath: String::new(),
            serverProtocol: String::new(),
            server: String::new(),
            serverPort: String::new(),
            serverPath: String::new(),
            timeout: String::new(),
            userName: String::new(),
            password: String::new(),
            proxyProtocol: String::new(),
            proxyServer: String::new(),
            proxyPort: String::new(),
            proxyPath: String::new(),
            proxyUserName: String::new(),
            proxyPassword: String::new(),
            proxyTimeout: String::new(),
            special1: String::new(),
            special2: String::new(),
            special3: String::new(),
            special4: String::new(),
            special5: String::new(),
            special6: String::new(),
            special7: String::new(),
            special8: String::new(),
            special9: String::new(),
            lastModified: String::new(),
            timeEntered: String::new(),
        };
        this.getFormData(hashMap.clone());
        this
    }

    pub fn new(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashMap))))
    }
    

    pub fn getFormData(&mut self, mut hashMap: Option<Rc<RefCell<HashMap>>>) {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut empty: String = jcall!(stringUtil).EMPTY_STRING.clone();
        self.storeName = (jcall!(hashMap).get(jcall!(StoreFrontData::getInstance()).NAME.clone())).clone() /* as String */;
        self.name = (jcall!(hashMap).get(jcall!(PaymentGatewayData_NAME.with(Clone::clone)).toString())).clone() /* as String */;
        self.enable = jcall!(stringUtil).getNonNull((jcall!(hashMap).get(jcall!(EntryData::getInstance()).ENABLE.clone())).clone() /* as String */);
        self.paymentMethod = jcall!(stringUtil).getNonNull((jcall!(hashMap).get(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_METHOD.with(Clone::clone).clone())).clone() /* as String */);
        self.mode = jcall!(stringUtil).getNonNull((jcall!(hashMap).get(jcall!(PaymentGatewayData_MODE.with(Clone::clone)).toString())).clone() /* as String */);
        self.testProtocol = jcall!(stringUtil).getNonNull((jcall!(hashMap).get(jcall!(PaymentGatewayData_TESTPROTOCOL.with(Clone::clone)).toString())).clone() /* as String */);
        self.testServer = jcall!(stringUtil).getNonNull((jcall!(hashMap).get(jcall!(PaymentGatewayData_TESTSERVER.with(Clone::clone)).toString())).clone() /* as String */);
        self.testPort = jcall!(stringUtil).getNonNull((jcall!(hashMap).get(jcall!(PaymentGatewayData_TESTPORT.with(Clone::clone)).toString())).clone() /* as String */);
        self.testPath = (jcall!(hashMap).get(jcall!(PaymentGatewayData_TESTPATH.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.testPath = empty.clone();
        }
        self.serverProtocol = (jcall!(hashMap).get(jcall!(PaymentGatewayData_SERVERPROTOCOL.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.serverProtocol = empty.clone();
        }
        self.server = (jcall!(hashMap).get(jcall!(PaymentGatewayData_SERVER.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.server = empty.clone();
        }
        self.serverPort = (jcall!(hashMap).get(jcall!(PaymentGatewayData_SERVERPORT.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.serverPort = empty.clone();
        }
        self.serverPath = (jcall!(hashMap).get(jcall!(PaymentGatewayData_SERVERPATH.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.serverPath = empty.clone();
        }
        self.userName = (jcall!(hashMap).get(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone())).clone() /* as String */;
        if false {
            self.userName = empty.clone();
        }
        self.password = (jcall!(hashMap).get(crate::org::allbinary::business::user::UserData::UserData_PASSWORD.with(Clone::clone).clone())).clone() /* as String */;
        if false {
            self.password = empty.clone();
        }
        self.timeout = (jcall!(hashMap).get(jcall!(PaymentGatewayData_TIMEOUT.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.timeout = empty.clone();
        }
        self.proxyProtocol = (jcall!(hashMap).get(jcall!(PaymentGatewayData_PROXYPROTOCOL.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.proxyProtocol = empty.clone();
        }
        self.proxyServer = (jcall!(hashMap).get(jcall!(PaymentGatewayData_PROXYSERVER.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.proxyServer = empty.clone();
        }
        self.proxyPort = (jcall!(hashMap).get(jcall!(PaymentGatewayData_PROXYPORT.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.proxyPort = empty.clone();
        }
        self.proxyPath = (jcall!(hashMap).get(jcall!(PaymentGatewayData_PROXYPATH.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.proxyPath = empty.clone();
        }
        self.proxyUserName = (jcall!(hashMap).get(jcall!(PaymentGatewayData_PROXYUSERNAME.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.proxyUserName = empty.clone();
        }
        self.proxyPassword = (jcall!(hashMap).get(jcall!(PaymentGatewayData_PROXYPASSWORD.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.proxyPassword = empty.clone();
        }
        self.proxyTimeout = (jcall!(hashMap).get(jcall!(PaymentGatewayData_PROXYTIMEOUT.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.proxyTimeout = empty.clone();
        }
        self.special1 = (jcall!(hashMap).get(jcall!(PaymentGatewayData_SPECIAL1.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.special1 = empty.clone();
        }
        self.special2 = (jcall!(hashMap).get(jcall!(PaymentGatewayData_SPECIAL2.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.special2 = empty.clone();
        }
        self.special3 = (jcall!(hashMap).get(jcall!(PaymentGatewayData_SPECIAL3.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.special3 = empty.clone();
        }
        self.special4 = (jcall!(hashMap).get(jcall!(PaymentGatewayData_SPECIAL4.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.special4 = empty.clone();
        }
        self.special5 = (jcall!(hashMap).get(jcall!(PaymentGatewayData_SPECIAL5.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.special5 = empty.clone();
        }
        self.special6 = (jcall!(hashMap).get(jcall!(PaymentGatewayData_SPECIAL6.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.special6 = empty.clone();
        }
        self.special7 = (jcall!(hashMap).get(jcall!(PaymentGatewayData_SPECIAL7.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.special7 = empty.clone();
        }
        self.special8 = (jcall!(hashMap).get(jcall!(PaymentGatewayData_SPECIAL8.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.special8 = empty.clone();
        }
        self.special9 = (jcall!(hashMap).get(jcall!(PaymentGatewayData_SPECIAL9.with(Clone::clone)).toString())).clone() /* as String */;
        if false {
            self.special9 = empty.clone();
        }
        self.lastModified = (jcall!(hashMap).get(jcall!(EntryData::getInstance()).LASTMODIFIED.clone())).clone() /* as String */;
        if false {
            self.lastModified = empty.clone();
        }
        self.timeEntered = (jcall!(hashMap).get(jcall!(EntryData::getInstance()).TIMECREATED.clone())).clone() /* as String */;
        if false {
            self.timeEntered = empty.clone();
        }
    }
    

    pub fn getStoreName(&mut self) -> String {
        return self.storeName.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getEnable(&mut self) -> String {
        return self.enable.clone();
    }
    

    pub fn getPaymentMethod(&mut self) -> String {
        return self.paymentMethod.clone();
    }
    

    pub fn getMode(&mut self) -> String {
        return self.mode.clone();
    }
    

    pub fn getTestProtocol(&mut self) -> String {
        return self.testProtocol.clone();
    }
    

    pub fn getTestServer(&mut self) -> String {
        return self.testServer.clone();
    }
    

    pub fn getTestPort(&mut self) -> String {
        return self.testPort.clone();
    }
    

    pub fn getTestPath(&mut self) -> String {
        return self.testPath.clone();
    }
    

    pub fn getServerProtocol(&mut self) -> String {
        return self.serverProtocol.clone();
    }
    

    pub fn getServer(&mut self) -> String {
        return self.server.clone();
    }
    

    pub fn getServerPort(&mut self) -> String {
        return self.serverPort.clone();
    }
    

    pub fn getServerPath(&mut self) -> String {
        return self.serverPath.clone();
    }
    

    pub fn getUserName(&mut self) -> String {
        return self.userName.clone();
    }
    

    pub fn getPassword(&mut self) -> String {
        return self.password.clone();
    }
    

    pub fn getProxyProtocol(&mut self) -> String {
        return self.proxyProtocol.clone();
    }
    

    pub fn getProxyServer(&mut self) -> String {
        return self.proxyServer.clone();
    }
    

    pub fn getProxyPath(&mut self) -> String {
        return self.proxyPath.clone();
    }
    

    pub fn getProxyPort(&mut self) -> String {
        return self.proxyPort.clone();
    }
    

    pub fn getProxyUserName(&mut self) -> String {
        return self.proxyUserName.clone();
    }
    

    pub fn getProxyPassword(&mut self) -> String {
        return self.proxyPassword.clone();
    }
    

    pub fn getTimeout(&mut self) -> String {
        return self.timeout.clone();
    }
    

    pub fn getProxyTimeout(&mut self) -> String {
        return self.proxyTimeout.clone();
    }
    

    pub fn getSpecial1(&mut self) -> String {
        return self.special1.clone();
    }
    

    pub fn getSpecial2(&mut self) -> String {
        return self.special2.clone();
    }
    

    pub fn getSpecial3(&mut self) -> String {
        return self.special3.clone();
    }
    

    pub fn getSpecial4(&mut self) -> String {
        return self.special4.clone();
    }
    

    pub fn getSpecial5(&mut self) -> String {
        return self.special5.clone();
    }
    

    pub fn getSpecial6(&mut self) -> String {
        return self.special6.clone();
    }
    

    pub fn getSpecial7(&mut self) -> String {
        return self.special7.clone();
    }
    

    pub fn getSpecial8(&mut self) -> String {
        return self.special8.clone();
    }
    

    pub fn getSpecial9(&mut self) -> String {
        return self.special9.clone();
    }
    

    pub fn getLastModified(&mut self) -> String {
        return self.lastModified.clone();
    }
    

    pub fn getTimeEntered(&mut self) -> String {
        return self.timeEntered.clone();
    }
    

    pub fn setEnable(&mut self, mut value: String) {
        self.enable = value.clone();
    }
    

    pub fn setStoreName(&mut self, mut value: String) {
        self.storeName = value.clone();
    }
    

    pub fn setName(&mut self, mut value: String) {
        self.name = value.clone();
    }
    

    pub fn setPaymentMethod(&mut self, mut value: String) {
        self.paymentMethod = value.clone();
    }
    

    pub fn setMode(&mut self, mut value: String) {
        self.mode = value.clone();
    }
    

    pub fn setTestProtocol(&mut self, mut value: String) {
        self.testProtocol = value.clone();
    }
    

    pub fn setTestServer(&mut self, mut value: String) {
        self.testServer = value.clone();
    }
    

    pub fn setTestPort(&mut self, mut value: String) {
        self.testPort = value.clone();
    }
    

    pub fn setTestPath(&mut self, mut value: String) {
        self.testPath = value.clone();
    }
    

    pub fn setServerProtocol(&mut self, mut value: String) {
        self.serverProtocol = value.clone();
    }
    

    pub fn setServer(&mut self, mut value: String) {
        self.server = value.clone();
    }
    

    pub fn setServerPort(&mut self, mut value: String) {
        self.serverPort = value.clone();
    }
    

    pub fn setServerPath(&mut self, mut value: String) {
        self.serverPath = value.clone();
    }
    

    pub fn setTimeout(&mut self, mut value: String) {
        self.timeout = value.clone();
    }
    

    pub fn setUserName(&mut self, mut value: String) {
        self.userName = value.clone();
    }
    

    pub fn setPassword(&mut self, mut value: String) {
        self.password = value.clone();
    }
    

    pub fn setProxyProtocol(&mut self, mut value: String) {
        self.proxyProtocol = value.clone();
    }
    

    pub fn setProxyServer(&mut self, mut value: String) {
        self.proxyServer = value.clone();
    }
    

    pub fn setProxyPort(&mut self, mut value: String) {
        self.proxyPort = value.clone();
    }
    

    pub fn setProxyPath(&mut self, mut value: String) {
        self.proxyPath = value.clone();
    }
    

    pub fn setProxyUserName(&mut self, mut value: String) {
        self.proxyUserName = value.clone();
    }
    

    pub fn setProxyPassword(&mut self, mut value: String) {
        self.proxyPassword = value.clone();
    }
    

    pub fn setProxyTimeout(&mut self, mut value: String) {
        self.proxyTimeout = value.clone();
    }
    

    pub fn setSpecial1(&mut self, mut value: String) {
        self.special1 = value.clone();
    }
    

    pub fn setSpecial2(&mut self, mut value: String) {
        self.special2 = value.clone();
    }
    

    pub fn setSpecial3(&mut self, mut value: String) {
        self.special3 = value.clone();
    }
    

    pub fn setSpecial4(&mut self, mut value: String) {
        self.special4 = value.clone();
    }
    

    pub fn setSpecial5(&mut self, mut value: String) {
        self.special5 = value.clone();
    }
    

    pub fn setSpecial6(&mut self, mut value: String) {
        self.special6 = value.clone();
    }
    

    pub fn setSpecial7(&mut self, mut value: String) {
        self.special7 = value.clone();
    }
    

    pub fn setSpecial8(&mut self, mut value: String) {
        self.special8 = value.clone();
    }
    

    pub fn setSpecial9(&mut self, mut value: String) {
        self.special9 = value.clone();
    }
    

    pub fn setLastModified(&mut self, mut value: String) {
        self.lastModified = value.clone();
    }
    

    pub fn setTimeEntered(&mut self, mut value: String) {
        self.timeEntered = value.clone();
    }
    

    // throws Exception
    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        return self.toHashMap1(true);
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn toHashMap1(&mut self, mut isEncrypted: bool) -> Option<Rc<RefCell<HashMap>>> {
        let mut paymentGatewayHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(paymentGatewayHashMap).put(jcall!(EntryData::getInstance()).ENABLE.clone(), self.enable.clone());
        jcall!(paymentGatewayHashMap).put(jcall!(StoreFrontData::getInstance()).NAME.clone(), self.storeName.clone());
        jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_NAME.with(Clone::clone)).toString(), self.getName());
        jcall!(paymentGatewayHashMap).put(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_METHOD.with(Clone::clone).clone(), self.paymentMethod.clone());
        jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_MODE.with(Clone::clone)).toString(), self.mode.clone());
        jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_TESTPROTOCOL.with(Clone::clone)).toString(), self.testProtocol.clone());
        jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_TESTSERVER.with(Clone::clone)).toString(), self.testServer.clone());
        jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_TESTPORT.with(Clone::clone)).toString(), self.testPort.clone());
        jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_TESTPATH.with(Clone::clone)).toString(), self.testPath.clone());
        jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SERVERPROTOCOL.with(Clone::clone)).toString(), self.serverProtocol.clone());
        jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SERVER.with(Clone::clone)).toString(), self.server.clone());
        jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SERVERPORT.with(Clone::clone)).toString(), self.serverPort.clone());
        jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SERVERPATH.with(Clone::clone)).toString(), self.serverPath.clone());
        if isEncrypted {
            let mut superCrypt: Option<Rc<RefCell<SuperCrypt>>> = SuperCrypt::new(PaymentGatewayData_CRYPTNUM.with(Clone::clone).clone());
            jcall!(paymentGatewayHashMap).put(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), jcall!(superCrypt).encrypt(self.userName.clone()));
            jcall!(paymentGatewayHashMap).put(crate::org::allbinary::business::user::UserData::UserData_PASSWORD.with(Clone::clone).clone(), jcall!(superCrypt).encrypt(self.password.clone()));
        } else {
            jcall!(paymentGatewayHashMap).put(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), self.userName.clone());
            jcall!(paymentGatewayHashMap).put(crate::org::allbinary::business::user::UserData::UserData_PASSWORD.with(Clone::clone).clone(), self.password.clone());
        }
        jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_TIMEOUT.with(Clone::clone)).toString(), self.timeout.clone());
        jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_PROXYPROTOCOL.with(Clone::clone)).toString(), self.proxyProtocol.clone());
        jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_PROXYSERVER.with(Clone::clone)).toString(), self.proxyServer.clone());
        jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_PROXYPORT.with(Clone::clone)).toString(), self.proxyPort.clone());
        jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_PROXYPATH.with(Clone::clone)).toString(), self.proxyPath.clone());
        jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_PROXYUSERNAME.with(Clone::clone)).toString(), self.proxyUserName.clone());
        jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_PROXYPASSWORD.with(Clone::clone)).toString(), self.proxyPassword.clone());
        jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_PROXYTIMEOUT.with(Clone::clone)).toString(), self.proxyTimeout.clone());
        if isEncrypted {
            let mut superCrypt: Option<Rc<RefCell<SuperCrypt>>> = SuperCrypt::new(PaymentGatewayData_CRYPTNUM.with(Clone::clone).clone());
            jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SPECIAL1.with(Clone::clone)).toString(), jcall!(superCrypt).encrypt(self.special1.clone()));
            jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SPECIAL2.with(Clone::clone)).toString(), jcall!(superCrypt).encrypt(self.special2.clone()));
            jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SPECIAL3.with(Clone::clone)).toString(), jcall!(superCrypt).encrypt(self.special3.clone()));
            jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SPECIAL4.with(Clone::clone)).toString(), jcall!(superCrypt).encrypt(self.special4.clone()));
            jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SPECIAL5.with(Clone::clone)).toString(), jcall!(superCrypt).encrypt(self.special5.clone()));
            jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SPECIAL6.with(Clone::clone)).toString(), jcall!(superCrypt).encrypt(self.special6.clone()));
            jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SPECIAL7.with(Clone::clone)).toString(), jcall!(superCrypt).encrypt(self.special7.clone()));
            jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SPECIAL8.with(Clone::clone)).toString(), jcall!(superCrypt).encrypt(self.special8.clone()));
        } else {
            jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SPECIAL1.with(Clone::clone)).toString(), self.special1.clone());
            jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SPECIAL2.with(Clone::clone)).toString(), self.special2.clone());
            jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SPECIAL3.with(Clone::clone)).toString(), self.special3.clone());
            jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SPECIAL4.with(Clone::clone)).toString(), self.special4.clone());
            jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SPECIAL5.with(Clone::clone)).toString(), self.special5.clone());
            jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SPECIAL6.with(Clone::clone)).toString(), self.special6.clone());
            jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SPECIAL7.with(Clone::clone)).toString(), self.special7.clone());
            jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SPECIAL8.with(Clone::clone)).toString(), self.special8.clone());
            jcall!(paymentGatewayHashMap).put(jcall!(PaymentGatewayData_SPECIAL9.with(Clone::clone)).toString(), self.special9.clone());
        }
        jcall!(paymentGatewayHashMap).put(jcall!(EntryData::getInstance()).LASTMODIFIED.clone(), self.lastModified.clone());
        return paymentGatewayHashMap.clone();
    }
    

    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.toVector1(true);
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn toVector1(&mut self, mut isEncrypted: bool) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut updateVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(updateVector).add(crate::java_into_object(self.enable.clone()));
        jcall!(updateVector).add(crate::java_into_object(self.storeName.clone()));
        jcall!(updateVector).add(crate::java_into_object(self.getName()));
        jcall!(updateVector).add(crate::java_into_object(self.paymentMethod.clone()));
        jcall!(updateVector).add(crate::java_into_object(self.mode.clone()));
        jcall!(updateVector).add(crate::java_into_object(self.testProtocol.clone()));
        jcall!(updateVector).add(crate::java_into_object(self.testServer.clone()));
        jcall!(updateVector).add(crate::java_into_object(self.testPort.clone()));
        jcall!(updateVector).add(crate::java_into_object(self.testPath.clone()));
        jcall!(updateVector).add(crate::java_into_object(self.serverProtocol.clone()));
        jcall!(updateVector).add(crate::java_into_object(self.server.clone()));
        jcall!(updateVector).add(crate::java_into_object(self.serverPort.clone()));
        jcall!(updateVector).add(crate::java_into_object(self.serverPath.clone()));
        if isEncrypted {
            let mut superCrypt: Option<Rc<RefCell<SuperCrypt>>> = SuperCrypt::new(PaymentGatewayData_CRYPTNUM.with(Clone::clone).clone());
            jcall!(updateVector).add(crate::java_into_object(jcall!(superCrypt).encrypt(self.userName.clone())));
            jcall!(updateVector).add(crate::java_into_object(jcall!(superCrypt).encrypt(self.password.clone())));
        } else {
            jcall!(updateVector).add(crate::java_into_object(self.userName.clone()));
            jcall!(updateVector).add(crate::java_into_object(self.password.clone()));
        }
        jcall!(updateVector).add(crate::java_into_object(self.timeout.clone()));
        jcall!(updateVector).add(crate::java_into_object(self.proxyProtocol.clone()));
        jcall!(updateVector).add(crate::java_into_object(self.proxyServer.clone()));
        jcall!(updateVector).add(crate::java_into_object(self.proxyPort.clone()));
        jcall!(updateVector).add(crate::java_into_object(self.proxyPath.clone()));
        jcall!(updateVector).add(crate::java_into_object(self.proxyUserName.clone()));
        jcall!(updateVector).add(crate::java_into_object(self.proxyPassword.clone()));
        jcall!(updateVector).add(crate::java_into_object(self.proxyTimeout.clone()));
        if isEncrypted {
            let mut superCrypt: Option<Rc<RefCell<SuperCrypt>>> = SuperCrypt::new(PaymentGatewayData_CRYPTNUM.with(Clone::clone).clone());
            jcall!(updateVector).add(crate::java_into_object(jcall!(superCrypt).encrypt(self.special1.clone())));
            jcall!(updateVector).add(crate::java_into_object(jcall!(superCrypt).encrypt(self.special2.clone())));
            jcall!(updateVector).add(crate::java_into_object(jcall!(superCrypt).encrypt(self.special3.clone())));
            jcall!(updateVector).add(crate::java_into_object(jcall!(superCrypt).encrypt(self.special4.clone())));
            jcall!(updateVector).add(crate::java_into_object(jcall!(superCrypt).encrypt(self.special5.clone())));
            jcall!(updateVector).add(crate::java_into_object(jcall!(superCrypt).encrypt(self.special6.clone())));
            jcall!(updateVector).add(crate::java_into_object(jcall!(superCrypt).encrypt(self.special7.clone())));
            jcall!(updateVector).add(crate::java_into_object(jcall!(superCrypt).encrypt(self.special8.clone())));
            jcall!(updateVector).add(crate::java_into_object(jcall!(superCrypt).encrypt(self.special9.clone())));
        } else {
            jcall!(updateVector).add(crate::java_into_object(self.special1.clone()));
            jcall!(updateVector).add(crate::java_into_object(self.special2.clone()));
            jcall!(updateVector).add(crate::java_into_object(self.special3.clone()));
            jcall!(updateVector).add(crate::java_into_object(self.special4.clone()));
            jcall!(updateVector).add(crate::java_into_object(self.special5.clone()));
            jcall!(updateVector).add(crate::java_into_object(self.special6.clone()));
            jcall!(updateVector).add(crate::java_into_object(self.special7.clone()));
            jcall!(updateVector).add(crate::java_into_object(self.special8.clone()));
            jcall!(updateVector).add(crate::java_into_object(self.special9.clone()));
        }
        return updateVector.clone();
    }
    

    // throws Exception
    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return (PaymentGatewayPrimaryKey::new(self.getStoreName(), self.getName())).clone() /* as Any */;
    }
}

