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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait BasicStoreFrontInterface {
    

    fn getTestHtmlPath(&mut self) -> String;
    

    fn getCurrentHostName(&mut self) -> String;
    

    fn getCurrentHostNamePath(&mut self) -> String;
    

    fn getCurrentHomeHostName(&mut self) -> String;
    

    fn getCurrentHomeHostNamePath(&mut self) -> String;
    

    fn getName(&mut self) -> String;
    

    fn getUserName(&mut self) -> String;
    

    fn getBasketName(&mut self) -> String;
    

    fn getHomeHostName(&mut self) -> String;
    

    fn getHomeHostNamePath(&mut self) -> String;
    

    fn getHostName(&mut self) -> String;
    

    fn getHostNamePath(&mut self) -> String;
    

    fn getTestHomeHostName(&mut self) -> String;
    

    fn getTestHomeHostNamePath(&mut self) -> String;
    

    fn getTestHostName(&mut self) -> String;
    

    fn getTestHostNamePath(&mut self) -> String;
    

    fn getStaticPath(&mut self) -> String;
    

    fn getCategoryPath(&mut self) -> String;
    

    // throws Exception
    fn getSubStores(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    fn getTagLocation(&mut self) -> String;
    

    fn getPackageLocation(&mut self) -> String;
    

    fn getInventoryControl(&mut self) -> String;
    

    fn getFtp(&mut self) -> String;
    

    fn getFtpPath(&mut self) -> String;
    

    fn getFtpUserName(&mut self) -> String;
    

    fn getFtpPassword(&mut self) -> String;
    

    fn getTestFtp(&mut self) -> String;
    

    fn getTestFtpPath(&mut self) -> String;
    

    fn getTestFtpUserName(&mut self) -> String;
    

    fn getTestFtpPassword(&mut self) -> String;
    

    fn getTimeCreated(&mut self) -> String;
    

    fn getLastModified(&mut self) -> String;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


