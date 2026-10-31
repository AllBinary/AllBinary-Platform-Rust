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
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::ThirdPartyInApplicationPurchaseInterface::ThirdPartyInApplicationPurchaseInterface;
use super::LockableFeature::LockableFeature;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ThirdPartyInApplicationPurchase {
}

impl ThirdPartyInApplicationPurchaseInterface for ThirdPartyInApplicationPurchase {
    fn isPurchased(&mut self, mut lockableFeature: Option<Rc<RefCell<LockableFeature>>>) -> bool { Self::isPurchased(self, lockableFeature) }
    fn onCreate(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut stateObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { Self::onCreate(self, anyType, stateObject) }
    fn onDestroy(&mut self) { Self::onDestroy(self) }
    fn onStart(&mut self) { Self::onStart(self) }
    fn onActivityResult(&mut self, mut requestCode: i32, mut resultCode: i32, mut dataObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { Self::onActivityResult(self, requestCode, resultCode, dataObject) }
    fn onSaveInstanceState(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { Self::onSaveInstanceState(self, anyType) }
    fn onStop(&mut self) { Self::onStop(self) }
    fn requestPurchase(&mut self, mut lockableFeature: Option<Rc<RefCell<LockableFeature>>>) { Self::requestPurchase(self, lockableFeature) }
    fn requestReceipts(&mut self) { Self::requestReceipts(self) }
    fn add(&mut self, mut lockableFeature: Option<Rc<RefCell<LockableFeature>>>) { Self::add(self, lockableFeature) }
    fn getUserName(&mut self) -> String { Self::getUserName(self) }
    fn isEnabled(&mut self) -> bool { Self::isEnabled(self) }
}

impl ThirdPartyInApplicationPurchase {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isPurchased(&mut self, mut lockableFeature: Option<Rc<RefCell<LockableFeature>>>) -> bool {
        return false;
    }
    

    pub fn onCreate(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut stateObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn onDestroy(&mut self) {
    }
    

    pub fn onStart(&mut self) {
    }
    

    pub fn onActivityResult(&mut self, mut requestCode: i32, mut resultCode: i32, mut dataObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn onSaveInstanceState(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn onStop(&mut self) {
    }
    

    pub fn requestPurchase(&mut self, mut lockableFeature: Option<Rc<RefCell<LockableFeature>>>) {
    }
    

    pub fn requestReceipts(&mut self) {
    }
    

    pub fn add(&mut self, mut lockableFeature: Option<Rc<RefCell<LockableFeature>>>) {
    }
    

    pub fn getUserName(&mut self) -> String {
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
    

    pub fn isEnabled(&mut self) -> bool {
        return false;
    }
    

    pub fn requestProducts(&mut self) {
    }
}
pub trait ThirdPartyInApplicationPurchaseDyn: crate::org::allbinary::logic::system::security::licensing::ThirdPartyInApplicationPurchaseInterface::ThirdPartyInApplicationPurchaseInterface {
    fn as_ThirdPartyInApplicationPurchase(&self) -> &ThirdPartyInApplicationPurchase;
    fn as_ThirdPartyInApplicationPurchase_mut(&mut self) -> &mut ThirdPartyInApplicationPurchase;
    fn requestProducts(&mut self) { ThirdPartyInApplicationPurchase::requestProducts(self.as_ThirdPartyInApplicationPurchase_mut()) }
}
impl ThirdPartyInApplicationPurchaseDyn for ThirdPartyInApplicationPurchase {
    fn as_ThirdPartyInApplicationPurchase(&self) -> &ThirdPartyInApplicationPurchase { self }
    fn as_ThirdPartyInApplicationPurchase_mut(&mut self) -> &mut ThirdPartyInApplicationPurchase { self }
}


