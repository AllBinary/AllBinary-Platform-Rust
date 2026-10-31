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
// package org.allbinary.business.advertisement.campaign
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::DynamicObjectData::DynamicObjectData;
use crate::org::allbinary::business::advertisement::AdvertisementsInterface::AdvertisementsInterface;
use crate::org::allbinary::business::advertisement::search::AdvertisementSearchInterface::AdvertisementSearchInterface;
use super::AdvertisementCampaignInterface::AdvertisementCampaignInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdvertisementCampaign {
    pub hashMap: Option<Rc<RefCell<HashMap>>>,
}

impl AdvertisementCampaignInterface for AdvertisementCampaign {
    fn getName(&mut self) -> String { Self::getName(self) }
    fn getComponent(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getComponent(self) }
    fn setName(&mut self, mut name: String) { Self::setName(self, name) }
    fn setComponentName(&mut self, mut name: String) { Self::setComponentName(self, name) }
    fn search(&mut self, mut advertisementSearchInterface: Option<Rc<RefCell<dyn AdvertisementSearchInterface>>>) -> Option<Rc<RefCell<dyn AdvertisementsInterface>>> { Self::search(self, advertisementSearchInterface) }
}

impl AdvertisementCampaign {
    

    pub fn new_struct(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            hashMap: None,
        };
        this.hashMap = hashMap.clone();
        this
    }

    pub fn new(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashMap))))
    }
    

    pub fn getComponent(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return None;
    }
    

    pub fn getName(&mut self) -> String {
        return ({ let java_receiver = self.hashMap.clone(); let java_arguments = (jcall!(AdvertisementCampaignData::AdvertisementCampaignData::getInstance()).NAME.clone(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn setComponentName(&mut self, mut name: String) {
        jcall!(self.hashMap).put(crate::org::allbinary::business::DynamicObjectData::DynamicObjectData_NAME.with(Clone::clone).clone(), name.clone());
    }
    

    pub fn setName(&mut self, mut name: String) {
        { let java_receiver = self.hashMap.clone(); let java_arguments = (jcall!(AdvertisementCampaignData::AdvertisementCampaignData::getInstance()).NAME.clone(), name.clone(),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
    }
    

    pub fn search(&mut self, mut advertisementSearchInterface: Option<Rc<RefCell<dyn AdvertisementSearchInterface>>>) -> Option<Rc<RefCell<dyn AdvertisementsInterface>>> {
        return None;
    }
}

