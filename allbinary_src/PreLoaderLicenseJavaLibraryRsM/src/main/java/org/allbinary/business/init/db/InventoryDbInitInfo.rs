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
// package org.allbinary.business.init.db
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::DbInitInfo::DbInitInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InventoryDbInitInfo {
    pub base: DbInitInfo,
}

thread_local! { pub static InventoryDbInitInfo_INITFILE: String = String::from("inventorydbinitdata.dat"); }

impl InventoryDbInitInfo {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: DbInitInfo::new_struct(InventoryDbInitInfo_INITFILE.with(Clone::clone).clone(), true),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut read: bool) -> Self {
        let mut this = Self {
            base: DbInitInfo::new_struct(InventoryDbInitInfo_INITFILE.with(Clone::clone).clone(), read.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut read: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(read))))
    }
}
impl std::ops::Deref for InventoryDbInitInfo {
    type Target = DbInitInfo;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InventoryDbInitInfo {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn for InventoryDbInitInfo {
    fn as_DbConnectionInfo(&self) -> &crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo { &self.base.base }
    fn as_DbConnectionInfo_mut(&mut self) -> &mut crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo { &mut self.base.base }
}
impl crate::org::allbinary::business::init::db::DbInitInfo::DbInitInfoDyn for InventoryDbInitInfo {
    fn as_DbInitInfo(&self) -> &crate::org::allbinary::business::init::db::DbInitInfo::DbInitInfo { &self.base }
    fn as_DbInitInfo_mut(&mut self) -> &mut crate::org::allbinary::business::init::db::DbInitInfo::DbInitInfo { &mut self.base }
}
impl crate::org::allbinary::business::init::db::DatabaseConnectionInfoInterface::DatabaseConnectionInfoInterface for InventoryDbInitInfo {
    fn getUrl(&mut self) -> String { crate::org::allbinary::business::init::db::DbInitInfo::DbInitInfo::getUrl(crate::org::allbinary::business::init::db::DbInitInfo::DbInitInfoDyn::as_DbInitInfo_mut(self)) }
    fn getJdbcDriver(&mut self) -> String { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::getJdbcDriver(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self)) }
    fn getName(&mut self) -> String { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::getName(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self)) }
    fn getUserName(&mut self) -> String { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::getUserName(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self)) }
    fn getPassword(&mut self) -> String { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::getPassword(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self)) }
    fn getHost(&mut self) -> String { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::getHost(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self)) }
    fn getSchema(&mut self) -> String { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::getSchema(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self)) }
    fn getServer(&mut self) -> String { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::getServer(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self)) }
    fn getPort(&mut self) -> String { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::getPort(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self)) }
    fn setJdbcDriver(&mut self, p1: String) { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::setJdbcDriver(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self), p1) }
    fn setName(&mut self, p1: String) { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::setName(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self), p1) }
    fn setUserName(&mut self, p1: String) { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::setUserName(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self), p1) }
    fn setPassword(&mut self, p1: String) { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::setPassword(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self), p1) }
    fn setSchema(&mut self, p1: String) { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::setSchema(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self), p1) }
    fn setServer(&mut self, p1: String) { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::setServer(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self), p1) }
    fn setPort(&mut self, p1: String) { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::setPort(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self), p1) }
    fn getUserNameKey(&mut self) -> String { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::getUserNameKey(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self)) }
    fn getPasswordKey(&mut self) -> String { crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfo::getPasswordKey(crate::org::allbinary::business::init::db::DbConnectionInfo::DbConnectionInfoDyn::as_DbConnectionInfo_mut(self)) }
}


