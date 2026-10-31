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
// package org.allbinary.logic.system.security.licensing
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::LockableFeature::LockableFeature;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait ThirdPartyInApplicationPurchaseInterface {
    

    fn isPurchased(&mut self, lockableFeature: Option<Rc<RefCell<LockableFeature>>>) -> bool;
    

    fn onCreate(&mut self, anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, stateObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>>);
    

    fn onDestroy(&mut self);
    

    fn onStart(&mut self);
    

    fn onActivityResult(&mut self, requestCode: i32, resultCode: i32, dataObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>>);
    

    fn onSaveInstanceState(&mut self, anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>);
    

    fn onStop(&mut self);
    

    fn requestPurchase(&mut self, lockableFeature: Option<Rc<RefCell<LockableFeature>>>);
    

    fn requestReceipts(&mut self);
    

    fn add(&mut self, lockableFeature: Option<Rc<RefCell<LockableFeature>>>);
    

    fn getUserName(&mut self) -> String;
    

    fn isEnabled(&mut self) -> bool;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


