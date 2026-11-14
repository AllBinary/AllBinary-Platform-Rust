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
// package org.allbinary.business.context.modules.storefronts
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::logic::string::tokens::Tokenizer::Tokenizer;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::java::lang::Exception::*;
use super::BasicStoreFrontInterface::BasicStoreFrontInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicStoreFront {
    pub name: String,
    pub userName: String,
    pub basketName: String,
    pub homeHostName: String,
    pub homeHostNamePath: String,
    pub hostName: String,
    pub hostNamePath: String,
    pub testHomeHostName: String,
    pub testHomeHostNamePath: String,
    pub testHostName: String,
    pub testHostNamePath: String,
    pub imagePath: String,
    pub staticPath: String,
    pub categoryPath: String,
    pub inventoryControl: String,
    pub subStores: String,
    pub tagLocation: String,
    pub packageLocation: String,
    pub ftp: String,
    pub ftpPath: String,
    pub ftpUserName: String,
    pub ftpPassword: String,
    pub testFtp: String,
    pub testFtpPath: String,
    pub testFtpUserName: String,
    pub testFtpPassword: String,
    pub timeCreated: String,
    pub lastModified: String,
}

impl BasicStoreFrontInterface for BasicStoreFront {
    fn getTestHtmlPath(&mut self) -> String { Self::getTestHtmlPath(self) }
    fn getCurrentHostName(&mut self) -> String { Self::getCurrentHostName(self) }
    fn getCurrentHostNamePath(&mut self) -> String { Self::getCurrentHostNamePath(self) }
    fn getCurrentHomeHostName(&mut self) -> String { Self::getCurrentHomeHostName(self) }
    fn getCurrentHomeHostNamePath(&mut self) -> String { Self::getCurrentHomeHostNamePath(self) }
    fn getName(&mut self) -> String { Self::getName(self) }
    fn getUserName(&mut self) -> String { Self::getUserName(self) }
    fn getBasketName(&mut self) -> String { Self::getBasketName(self) }
    fn getHomeHostName(&mut self) -> String { Self::getHomeHostName(self) }
    fn getHomeHostNamePath(&mut self) -> String { Self::getHomeHostNamePath(self) }
    fn getHostName(&mut self) -> String { Self::getHostName(self) }
    fn getHostNamePath(&mut self) -> String { Self::getHostNamePath(self) }
    fn getTestHomeHostName(&mut self) -> String { Self::getTestHomeHostName(self) }
    fn getTestHomeHostNamePath(&mut self) -> String { Self::getTestHomeHostNamePath(self) }
    fn getTestHostName(&mut self) -> String { Self::getTestHostName(self) }
    fn getTestHostNamePath(&mut self) -> String { Self::getTestHostNamePath(self) }
    fn getStaticPath(&mut self) -> String { Self::getStaticPath(self) }
    fn getCategoryPath(&mut self) -> String { Self::getCategoryPath(self) }
    fn getSubStores(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getSubStores(self) }
    fn getTagLocation(&mut self) -> String { Self::getTagLocation(self) }
    fn getPackageLocation(&mut self) -> String { Self::getPackageLocation(self) }
    fn getInventoryControl(&mut self) -> String { Self::getInventoryControl(self) }
    fn getFtp(&mut self) -> String { Self::getFtp(self) }
    fn getFtpPath(&mut self) -> String { Self::getFtpPath(self) }
    fn getFtpUserName(&mut self) -> String { Self::getFtpUserName(self) }
    fn getFtpPassword(&mut self) -> String { Self::getFtpPassword(self) }
    fn getTestFtp(&mut self) -> String { Self::getTestFtp(self) }
    fn getTestFtpPath(&mut self) -> String { Self::getTestFtpPath(self) }
    fn getTestFtpUserName(&mut self) -> String { Self::getTestFtpUserName(self) }
    fn getTestFtpPassword(&mut self) -> String { Self::getTestFtpPassword(self) }
    fn getTimeCreated(&mut self) -> String { Self::getTimeCreated(self) }
    fn getLastModified(&mut self) -> String { Self::getLastModified(self) }
}

impl BasicStoreFront {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            name: String::new(),
            userName: String::new(),
            basketName: String::new(),
            homeHostName: String::new(),
            homeHostNamePath: String::new(),
            hostName: String::new(),
            hostNamePath: String::new(),
            testHomeHostName: String::new(),
            testHomeHostNamePath: String::new(),
            testHostName: String::new(),
            testHostNamePath: String::new(),
            imagePath: String::new(),
            staticPath: String::new(),
            categoryPath: String::new(),
            inventoryControl: String::new(),
            subStores: String::new(),
            tagLocation: String::new(),
            packageLocation: String::new(),
            ftp: String::new(),
            ftpPath: String::new(),
            ftpUserName: String::new(),
            ftpPassword: String::new(),
            testFtp: String::new(),
            testFtpPath: String::new(),
            testFtpUserName: String::new(),
            testFtpPassword: String::new(),
            timeCreated: String::new(),
            lastModified: String::new(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut storeHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Self {
        let mut this = Self {
            name: String::new(),
            userName: String::new(),
            basketName: String::new(),
            homeHostName: String::new(),
            homeHostNamePath: String::new(),
            hostName: String::new(),
            hostNamePath: String::new(),
            testHomeHostName: String::new(),
            testHomeHostNamePath: String::new(),
            testHostName: String::new(),
            testHostNamePath: String::new(),
            imagePath: String::new(),
            staticPath: String::new(),
            categoryPath: String::new(),
            inventoryControl: String::new(),
            subStores: String::new(),
            tagLocation: String::new(),
            packageLocation: String::new(),
            ftp: String::new(),
            ftpPath: String::new(),
            ftpUserName: String::new(),
            ftpPassword: String::new(),
            testFtp: String::new(),
            testFtpPath: String::new(),
            testFtpUserName: String::new(),
            testFtpPassword: String::new(),
            timeCreated: String::new(),
            lastModified: String::new(),
        };
        let mut storeFrontData: Option<Rc<RefCell<StoreFrontData>>> = StoreFrontData::getInstance();
        this.name = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).NAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.userName = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.basketName = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).NAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.homeHostName = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).HOMEHOSTNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.homeHostNamePath = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).HOMEHOSTNAMEPATH.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.hostName = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).HOSTNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.hostNamePath = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).HOSTNAMEPATH.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.testHomeHostName = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTHOMEHOSTNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.testHomeHostNamePath = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTHOMEHOSTNAMEPATH.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.testHostName = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTHOSTNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.testHostNamePath = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTHOSTNAMEPATH.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.imagePath = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).IMAGEPATH.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.staticPath = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).STATICPATH.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.categoryPath = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).CATEGORYPATH.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.inventoryControl = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).INVENTORYCONTROL.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.subStores = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).SUBSTORES.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.tagLocation = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).TAGLOCATION.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.packageLocation = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).PACKAGELOCATION.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.ftp = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).FTP.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.ftpPath = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).FTPPATH.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.ftpUserName = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).FTPUSERNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.ftpPassword = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).FTPPASSWORD.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.testFtp = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTFTP.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.testFtpPath = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTFTPPATH.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.testFtpUserName = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTFTPUSERNAME.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.testFtpPassword = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(storeFrontData).TESTFTPPASSWORD.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.timeCreated = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).TIMECREATED.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.lastModified = { let java_value = ({ let java_object = (jcall!(storeHashMap).get(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).LASTMODIFIED.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this
    }

    pub fn new1(mut storeHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(storeHashMap))))
    }
    

    pub fn getTestHtmlPath(&mut self) -> String {
        return format!("{}{}", { let java_value = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).globals.clone(); java_field }).URLGLOBALS.clone(); java_field }).getTestHtmlPath(); java_value }, self.getCurrentHostNamePath());
    }
    

    pub fn getCurrentHostName(&mut self) -> String {
        let mut location: String = String::new();
        if jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).globals.clone(); java_field }).URLGLOBALS.clone(); java_field }).isTestingMode() {
            location = self.getTestHostName();
        } else {
            location = self.getHostName();
        }
        return location.clone();
    }
    

    pub fn getCurrentHostNamePath(&mut self) -> String {
        let mut location: String = String::new();
        if jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).globals.clone(); java_field }).URLGLOBALS.clone(); java_field }).isTestingMode() {
            location = self.getTestHostNamePath();
        } else {
            location = self.getHostNamePath();
        }
        return location.clone();
    }
    

    pub fn getCurrentHomeHostName(&mut self) -> String {
        let mut location: String = String::new();
        if jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).globals.clone(); java_field }).URLGLOBALS.clone(); java_field }).isTestingMode() {
            location = self.getTestHomeHostName();
        } else {
            location = self.getHomeHostName();
        }
        return location.clone();
    }
    

    pub fn getCurrentHomeHostNamePath(&mut self) -> String {
        let mut location: String = String::new();
        if jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).globals.clone(); java_field }).URLGLOBALS.clone(); java_field }).isTestingMode() {
            location = self.getTestHomeHostNamePath();
        } else {
            location = self.getHomeHostNamePath();
        }
        return location.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getUserName(&mut self) -> String {
        return self.userName.clone();
    }
    

    pub fn getBasketName(&mut self) -> String {
        return self.basketName.clone();
    }
    

    pub fn getHomeHostName(&mut self) -> String {
        return self.homeHostName.clone();
    }
    

    pub fn getHomeHostNamePath(&mut self) -> String {
        return self.homeHostNamePath.clone();
    }
    

    pub fn getHostName(&mut self) -> String {
        return self.hostName.clone();
    }
    

    pub fn getHostNamePath(&mut self) -> String {
        return self.hostNamePath.clone();
    }
    

    pub fn getTestHomeHostName(&mut self) -> String {
        return self.testHomeHostName.clone();
    }
    

    pub fn getTestHomeHostNamePath(&mut self) -> String {
        return self.testHomeHostNamePath.clone();
    }
    

    pub fn getTestHostName(&mut self) -> String {
        return self.testHostName.clone();
    }
    

    pub fn getTestHostNamePath(&mut self) -> String {
        return self.testHostNamePath.clone();
    }
    

    pub fn getStaticPath(&mut self) -> String {
        return self.staticPath.clone();
    }
    

    pub fn getCategoryPath(&mut self) -> String {
        return self.categoryPath.clone();
    }
    

    pub fn getInventoryControl(&mut self) -> String {
        return self.inventoryControl.clone();
    }
    

    // throws Exception
    pub fn getSubStores(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut tokenizer: Option<Rc<RefCell<Tokenizer>>> = Tokenizer::new({ let java_field = jcall!(CommonSeps::getInstance()).SEMICOLON.clone(); java_field }.clone());
            let mut subStoreVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(tokenizer).getTokensFromString(self.subStores.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
            return Some(subStoreVector.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn getTagLocation(&mut self) -> String {
        return self.tagLocation.clone();
    }
    

    pub fn getPackageLocation(&mut self) -> String {
        return self.packageLocation.clone();
    }
    

    pub fn getFtp(&mut self) -> String {
        return self.ftp.clone();
    }
    

    pub fn getFtpUserName(&mut self) -> String {
        return self.ftpUserName.clone();
    }
    

    pub fn getFtpPassword(&mut self) -> String {
        return self.ftpPassword.clone();
    }
    

    pub fn getTestFtp(&mut self) -> String {
        return self.testFtp.clone();
    }
    

    pub fn getTestFtpUserName(&mut self) -> String {
        return self.testFtpUserName.clone();
    }
    

    pub fn getTestFtpPassword(&mut self) -> String {
        return self.testFtpPassword.clone();
    }
    

    pub fn getFtpPath(&mut self) -> String {
        return self.ftpPath.clone();
    }
    

    pub fn getTestFtpPath(&mut self) -> String {
        return self.testFtpPath.clone();
    }
    

    pub fn getTimeCreated(&mut self) -> String {
        return self.timeCreated.clone();
    }
    

    pub fn getLastModified(&mut self) -> String {
        return self.lastModified.clone();
    }
}

