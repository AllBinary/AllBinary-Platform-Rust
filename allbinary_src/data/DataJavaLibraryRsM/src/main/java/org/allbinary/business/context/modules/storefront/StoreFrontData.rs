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
// package org.allbinary.business.context.modules.storefront
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreFrontData {
    pub CANCEL: String,
    pub SELECT: String,
    pub INSTALL: String,
    pub INSTALL_COMPLETE: String,
    pub NAME: String,
    pub CURRENTHOSTNAME: String,
    pub CURRENTHOSTNAMEPATH: String,
    pub CURRENTHOMEHOSTNAME: String,
    pub CURRENTHOMEHOSTNAMEPATH: String,
    pub HOMEHOSTNAME: String,
    pub HOMEHOSTNAMEPATH: String,
    pub HOSTNAME: String,
    pub HOSTNAMEPATH: String,
    pub TESTHOMEHOSTNAME: String,
    pub TESTHOMEHOSTNAMEPATH: String,
    pub TESTHOSTNAME: String,
    pub TESTHOSTNAMEPATH: String,
    pub IMAGEPATH: String,
    pub STATICPATH: String,
    pub CATEGORYPATH: String,
    pub INVENTORYCONTROL: String,
    pub CONFIGURATION: String,
    pub SUBSTORES: String,
    pub TAGLOCATION: String,
    pub PACKAGELOCATION: String,
    pub FTP: String,
    pub FTPPATH: String,
    pub FTPUSERNAME: String,
    pub FTPPASSWORD: String,
    pub TESTFTP: String,
    pub TESTFTPPATH: String,
    pub TESTFTPUSERNAME: String,
    pub TESTFTPPASSWORD: String,
    pub SELECTSTORENAME: String,
}

thread_local! { pub static StoreFrontData_instance: Option<Rc<RefCell<StoreFrontData>>> = StoreFrontData::new(); }

impl StoreFrontData {

    pub fn new_struct() -> Self {
        let mut this = Self {
            CANCEL: String::new(),
            SELECT: String::new(),
            INSTALL: String::new(),
            INSTALL_COMPLETE: String::new(),
            NAME: String::new(),
            CURRENTHOSTNAME: String::new(),
            CURRENTHOSTNAMEPATH: String::new(),
            CURRENTHOMEHOSTNAME: String::new(),
            CURRENTHOMEHOSTNAMEPATH: String::new(),
            HOMEHOSTNAME: String::new(),
            HOMEHOSTNAMEPATH: String::new(),
            HOSTNAME: String::new(),
            HOSTNAMEPATH: String::new(),
            TESTHOMEHOSTNAME: String::new(),
            TESTHOMEHOSTNAMEPATH: String::new(),
            TESTHOSTNAME: String::new(),
            TESTHOSTNAMEPATH: String::new(),
            IMAGEPATH: String::new(),
            STATICPATH: String::new(),
            CATEGORYPATH: String::new(),
            INVENTORYCONTROL: String::new(),
            CONFIGURATION: String::new(),
            SUBSTORES: String::new(),
            TAGLOCATION: String::new(),
            PACKAGELOCATION: String::new(),
            FTP: String::new(),
            FTPPATH: String::new(),
            FTPUSERNAME: String::new(),
            FTPPASSWORD: String::new(),
            TESTFTP: String::new(),
            TESTFTPPATH: String::new(),
            TESTFTPUSERNAME: String::new(),
            TESTFTPPASSWORD: String::new(),
            SELECTSTORENAME: String::new(),
        };
        this.CANCEL = String::from("Cancel Store Front");
        this.SELECT = String::from("Select Store Front");
        this.INSTALL = String::from("Install Store Front");
        this.INSTALL_COMPLETE = String::from("Install Store Front Complete");
        this.NAME = String::from("STOREFRONT_NAME");
        this.CURRENTHOSTNAME = String::from("STOREFRONT_CURRENTHOSTNAME");
        this.CURRENTHOSTNAMEPATH = String::from("STOREFRONT_CURRENTHOSTNAMEPATH");
        this.CURRENTHOMEHOSTNAME = String::from("STOREFRONT_CURRENTHOMEHOSTNAME");
        this.CURRENTHOMEHOSTNAMEPATH = String::from("STOREFRONT_CURRENTHOMEHOSTNAMEPATH");
        this.HOMEHOSTNAME = String::from("STOREFRONT_HOMEHOSTNAME");
        this.HOMEHOSTNAMEPATH = String::from("STOREFRONT_HOMEHOSTNAMEPATH");
        this.HOSTNAME = String::from("STOREFRONT_HOSTNAME");
        this.HOSTNAMEPATH = String::from("STOREFRONT_HOSTNAMEPATH");
        this.TESTHOMEHOSTNAME = String::from("STOREFRONT_TESTHOMEHOSTNAME");
        this.TESTHOMEHOSTNAMEPATH = String::from("STOREFRONT_TESTHOMEHOSTNAMEPATH");
        this.TESTHOSTNAME = String::from("STOREFRONT_TESTHOSTNAME");
        this.TESTHOSTNAMEPATH = String::from("STOREFRONT_TESTHOSTNAMEPATH");
        this.IMAGEPATH = String::from("STOREFRONT_IMAGEPATH");
        this.STATICPATH = String::from("STOREFRONT_STATICPATH");
        this.CATEGORYPATH = String::from("STOREFRONT_CATEGORYPATH");
        this.INVENTORYCONTROL = String::from("STOREFRONT_INVENTORYCONTROL");
        this.CONFIGURATION = String::from("STOREFRONT_CONFIGURATION");
        this.SUBSTORES = String::from("STOREFRONT_SUBSTORES");
        this.TAGLOCATION = String::from("STOREFRONT_TAGLOCATION");
        this.PACKAGELOCATION = String::from("STOREFRONT_PACKAGELOCATION");
        this.FTP = String::from("STOREFRONT_FTP");
        this.FTPPATH = String::from("STOREFRONT_FTPPATH");
        this.FTPUSERNAME = String::from("STOREFRONT_FTPUSERNAME");
        this.FTPPASSWORD = String::from("STOREFRONT_FTPPASSWORD");
        this.TESTFTP = String::from("STOREFRONT_TESTFTP");
        this.TESTFTPPATH = String::from("STOREFRONT_TESTFTPPATH");
        this.TESTFTPUSERNAME = String::from("STOREFRONT_TESTFTPUSERNAME");
        this.TESTFTPPASSWORD = String::from("STOREFRONT_TESTFTPPASSWORD");
        this.SELECTSTORENAME = String::from("STOREFRONT_SELECTSTORENAME");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<StoreFrontData>>> {
        return StoreFrontData_instance.with(Clone::clone).clone();
    }
}

