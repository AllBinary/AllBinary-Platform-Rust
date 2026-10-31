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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::logic::string::tokens::Tokenizer::Tokenizer;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
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
    

    pub fn new_struct1(mut storeHashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
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
        this.name = (jcall!(storeHashMap).get(jcall!(storeFrontData).NAME.clone())).clone() /* as String */;
        this.userName = (jcall!(storeHashMap).get(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone())).clone() /* as String */;
        this.basketName = (jcall!(storeHashMap).get(jcall!(storeFrontData).NAME.clone())).clone() /* as String */;
        this.homeHostName = (jcall!(storeHashMap).get(jcall!(storeFrontData).HOMEHOSTNAME.clone())).clone() /* as String */;
        this.homeHostNamePath = (jcall!(storeHashMap).get(jcall!(storeFrontData).HOMEHOSTNAMEPATH.clone())).clone() /* as String */;
        this.hostName = (jcall!(storeHashMap).get(jcall!(storeFrontData).HOSTNAME.clone())).clone() /* as String */;
        this.hostNamePath = (jcall!(storeHashMap).get(jcall!(storeFrontData).HOSTNAMEPATH.clone())).clone() /* as String */;
        this.testHomeHostName = (jcall!(storeHashMap).get(jcall!(storeFrontData).TESTHOMEHOSTNAME.clone())).clone() /* as String */;
        this.testHomeHostNamePath = (jcall!(storeHashMap).get(jcall!(storeFrontData).TESTHOMEHOSTNAMEPATH.clone())).clone() /* as String */;
        this.testHostName = (jcall!(storeHashMap).get(jcall!(storeFrontData).TESTHOSTNAME.clone())).clone() /* as String */;
        this.testHostNamePath = (jcall!(storeHashMap).get(jcall!(storeFrontData).TESTHOSTNAMEPATH.clone())).clone() /* as String */;
        this.imagePath = (jcall!(storeHashMap).get(jcall!(storeFrontData).IMAGEPATH.clone())).clone() /* as String */;
        this.staticPath = (jcall!(storeHashMap).get(jcall!(storeFrontData).STATICPATH.clone())).clone() /* as String */;
        this.categoryPath = (jcall!(storeHashMap).get(jcall!(storeFrontData).CATEGORYPATH.clone())).clone() /* as String */;
        this.inventoryControl = (jcall!(storeHashMap).get(jcall!(storeFrontData).INVENTORYCONTROL.clone())).clone() /* as String */;
        this.subStores = (jcall!(storeHashMap).get(jcall!(storeFrontData).SUBSTORES.clone())).clone() /* as String */;
        this.tagLocation = (jcall!(storeHashMap).get(jcall!(storeFrontData).TAGLOCATION.clone())).clone() /* as String */;
        this.packageLocation = (jcall!(storeHashMap).get(jcall!(storeFrontData).PACKAGELOCATION.clone())).clone() /* as String */;
        this.ftp = (jcall!(storeHashMap).get(jcall!(storeFrontData).FTP.clone())).clone() /* as String */;
        this.ftpPath = (jcall!(storeHashMap).get(jcall!(storeFrontData).FTPPATH.clone())).clone() /* as String */;
        this.ftpUserName = (jcall!(storeHashMap).get(jcall!(storeFrontData).FTPUSERNAME.clone())).clone() /* as String */;
        this.ftpPassword = (jcall!(storeHashMap).get(jcall!(storeFrontData).FTPPASSWORD.clone())).clone() /* as String */;
        this.testFtp = (jcall!(storeHashMap).get(jcall!(storeFrontData).TESTFTP.clone())).clone() /* as String */;
        this.testFtpPath = (jcall!(storeHashMap).get(jcall!(storeFrontData).TESTFTPPATH.clone())).clone() /* as String */;
        this.testFtpUserName = (jcall!(storeHashMap).get(jcall!(storeFrontData).TESTFTPUSERNAME.clone())).clone() /* as String */;
        this.testFtpPassword = (jcall!(storeHashMap).get(jcall!(storeFrontData).TESTFTPPASSWORD.clone())).clone() /* as String */;
        this.timeCreated = (jcall!(storeHashMap).get(jcall!(EntryData::getInstance()).TIMECREATED.clone())).clone() /* as String */;
        this.lastModified = (jcall!(storeHashMap).get(jcall!(EntryData::getInstance()).LASTMODIFIED.clone())).clone() /* as String */;
        this
    }

    pub fn new1(mut storeHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(storeHashMap))))
    }
    

    pub fn getTestHtmlPath(&mut self) -> String {
        return jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).URLGLOBALS).getTestHtmlPath() + self.getCurrentHostNamePath();
    }
    

    pub fn getCurrentHostName(&mut self) -> String {
        let mut location: String = String::new();
        if jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).URLGLOBALS).isTestingMode() {
            location = self.getTestHostName();
        } else {
            location = self.getHostName();
        }
        return location.clone();
    }
    

    pub fn getCurrentHostNamePath(&mut self) -> String {
        let mut location: String = String::new();
        if jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).URLGLOBALS).isTestingMode() {
            location = self.getTestHostNamePath();
        } else {
            location = self.getHostNamePath();
        }
        return location.clone();
    }
    

    pub fn getCurrentHomeHostName(&mut self) -> String {
        let mut location: String = String::new();
        if jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).URLGLOBALS).isTestingMode() {
            location = self.getTestHomeHostName();
        } else {
            location = self.getHomeHostName();
        }
        return location.clone();
    }
    

    pub fn getCurrentHomeHostNamePath(&mut self) -> String {
        let mut location: String = String::new();
        if jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).globals).URLGLOBALS).isTestingMode() {
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
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut tokenizer: Option<Rc<RefCell<Tokenizer>>> = Tokenizer::new(jcall!(CommonSeps::getInstance()).SEMICOLON.clone());
            let mut subStoreVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(tokenizer).getTokensFromString(self.subStores.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
            return subStoreVector.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
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

