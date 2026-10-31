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
use crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait PaymentGatewayInterface: TableMappingInterface {
    

    fn getEnable(&mut self) -> String;
    

    fn getStoreName(&mut self) -> String;
    

    fn getName(&mut self) -> String;
    

    fn getPaymentMethod(&mut self) -> String;
    

    fn getMode(&mut self) -> String;
    

    fn getTestProtocol(&mut self) -> String;
    

    fn getTestServer(&mut self) -> String;
    

    fn getTestPort(&mut self) -> String;
    

    fn getTestPath(&mut self) -> String;
    

    fn getServerProtocol(&mut self) -> String;
    

    fn getServer(&mut self) -> String;
    

    fn getServerPort(&mut self) -> String;
    

    fn getServerPath(&mut self) -> String;
    

    fn getUserName(&mut self) -> String;
    

    fn getPassword(&mut self) -> String;
    

    fn getProxyProtocol(&mut self) -> String;
    

    fn getProxyServer(&mut self) -> String;
    

    fn getProxyPath(&mut self) -> String;
    

    fn getProxyPort(&mut self) -> String;
    

    fn getProxyUserName(&mut self) -> String;
    

    fn getProxyPassword(&mut self) -> String;
    

    fn getTimeout(&mut self) -> String;
    

    fn getProxyTimeout(&mut self) -> String;
    

    fn getSpecial1(&mut self) -> String;
    

    fn getSpecial2(&mut self) -> String;
    

    fn getSpecial3(&mut self) -> String;
    

    fn getSpecial4(&mut self) -> String;
    

    fn getSpecial5(&mut self) -> String;
    

    fn getSpecial6(&mut self) -> String;
    

    fn getSpecial7(&mut self) -> String;
    

    fn getSpecial8(&mut self) -> String;
    

    fn getSpecial9(&mut self) -> String;
    

    fn getLastModified(&mut self) -> String;
    

    fn getTimeEntered(&mut self) -> String;
    

    fn setEnable(&mut self, value: String);
    

    fn setStoreName(&mut self, value: String);
    

    fn setName(&mut self, value: String);
    

    fn setPaymentMethod(&mut self, value: String);
    

    fn setMode(&mut self, value: String);
    

    fn setTestProtocol(&mut self, value: String);
    

    fn setTestServer(&mut self, value: String);
    

    fn setTestPort(&mut self, value: String);
    

    fn setTestPath(&mut self, value: String);
    

    fn setServerProtocol(&mut self, value: String);
    

    fn setServer(&mut self, value: String);
    

    fn setServerPort(&mut self, value: String);
    

    fn setServerPath(&mut self, value: String);
    

    fn setUserName(&mut self, value: String);
    

    fn setPassword(&mut self, value: String);
    

    fn setProxyProtocol(&mut self, value: String);
    

    fn setProxyServer(&mut self, value: String);
    

    fn setProxyPath(&mut self, value: String);
    

    fn setProxyPort(&mut self, value: String);
    

    fn setProxyUserName(&mut self, value: String);
    

    fn setProxyPassword(&mut self, value: String);
    

    fn setTimeout(&mut self, value: String);
    

    fn setProxyTimeout(&mut self, value: String);
    

    fn setSpecial1(&mut self, value: String);
    

    fn setSpecial2(&mut self, value: String);
    

    fn setSpecial3(&mut self, value: String);
    

    fn setSpecial4(&mut self, value: String);
    

    fn setSpecial5(&mut self, value: String);
    

    fn setSpecial6(&mut self, value: String);
    

    fn setSpecial7(&mut self, value: String);
    

    fn setSpecial8(&mut self, value: String);
    

    fn setSpecial9(&mut self, value: String);
    

    fn setLastModified(&mut self, value: String);
    

    fn setTimeEntered(&mut self, value: String);
    

    // throws Exception
    fn toHashMap(&mut self, isEncrypted: bool) -> Option<Rc<RefCell<HashMap>>>;
    

    fn toVector(&mut self, isEncrypted: bool) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
}


