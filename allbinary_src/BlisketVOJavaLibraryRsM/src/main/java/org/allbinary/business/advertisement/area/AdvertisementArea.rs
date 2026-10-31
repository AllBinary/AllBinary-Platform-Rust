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
// package org.allbinary.business.advertisement.area
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::advertisement::campaign::AdvertisementCampaignData::AdvertisementCampaignData;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::w3c::dom::Document::Document;
use super::AdvertisementAreaInterface::AdvertisementAreaInterface;
use super::AdvertisementAreaConstraintsInterface::AdvertisementAreaConstraintsInterface;
use super::AdvertisementAreaConstraints::AdvertisementAreaConstraints;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdvertisementArea {
    pub name: String,
    pub storeName: String,
    pub description: String,
    pub advertisementAreaConstraintsInterface: Option<Rc<RefCell<dyn AdvertisementAreaConstraintsInterface>>>,
    pub campaign: String,
}

impl AdvertisementAreaInterface for AdvertisementArea {
    fn getName(&mut self) -> String { Self::getName(self) }
    fn getStoreName(&mut self) -> String { Self::getStoreName(self) }
    fn getDescription(&mut self) -> String { Self::getDescription(self) }
    fn getConstraints(&mut self) -> Option<Rc<RefCell<dyn AdvertisementAreaConstraintsInterface>>> { Self::getConstraints(self) }
    fn getCampaign(&mut self) -> String { Self::getCampaign(self) }
    fn setName(&mut self, mut name: String) { Self::setName(self, name) }
    fn setStoreName(&mut self, mut storeName: String) { Self::setStoreName(self, storeName) }
    fn setDescription(&mut self, mut description: String) { Self::setDescription(self, description) }
    fn setConstraints(&mut self, mut advertisementConstraintsInterface: Option<Rc<RefCell<dyn AdvertisementAreaConstraintsInterface>>>) { Self::setConstraints(self, advertisementConstraintsInterface) }
    fn setCampaign(&mut self, mut campaign: String) { Self::setCampaign(self, campaign) }
}

impl AdvertisementArea {
    

    pub fn new_struct(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            name: String::new(),
            storeName: String::new(),
            description: String::new(),
            advertisementAreaConstraintsInterface: None,
            campaign: String::new(),
        };
        this.name = (jcall!(hashMap).get(jcall!(AdvertisementAreaData::AdvertisementAreaData::getInstance()).NAME.clone())).clone() /* as String */;
        this.storeName = (jcall!(hashMap).get(jcall!(StoreFrontData::getInstance()).NAME.clone())).clone() /* as String */;
        this.description = (jcall!(hashMap).get(jcall!(AdvertisementAreaData::AdvertisementAreaData::getInstance()).DESCRIPTION.clone())).clone() /* as String */;
        let mut constraintsDomString: String = (jcall!(hashMap).get(jcall!(AdvertisementAreaData::AdvertisementAreaData::getInstance()).CONSTRAINTS.clone())).clone() /* as String */;
        let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create1(constraintsDomString.clone());
        this.advertisementAreaConstraintsInterface = AdvertisementAreaConstraints::new(document.clone()).map(|reference| reference as Rc<RefCell<dyn AdvertisementAreaConstraintsInterface>>);
        this.campaign = (jcall!(hashMap).get(jcall!(AdvertisementCampaignData::getInstance()).NAME.clone())).clone() /* as String */;
        this
    }

    pub fn new(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashMap))))
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getStoreName(&mut self) -> String {
        return self.storeName.clone();
    }
    

    pub fn getDescription(&mut self) -> String {
        return self.description.clone();
    }
    

    pub fn getConstraints(&mut self) -> Option<Rc<RefCell<dyn AdvertisementAreaConstraintsInterface>>> {
        return self.advertisementAreaConstraintsInterface.clone();
    }
    

    pub fn getCampaign(&mut self) -> String {
        return self.campaign.clone();
    }
    

    pub fn setName(&mut self, mut name: String) {
        self.name = name.clone();
    }
    

    pub fn setStoreName(&mut self, mut storeName: String) {
        self.storeName = storeName.clone();
    }
    

    pub fn setDescription(&mut self, mut description: String) {
        self.description = description.clone();
    }
    

    pub fn setConstraints(&mut self, mut advertisementConstraintsInterface: Option<Rc<RefCell<dyn AdvertisementAreaConstraintsInterface>>>) {
        self.advertisementAreaConstraintsInterface = self.advertisementAreaConstraintsInterface.clone();
    }
    

    pub fn setCampaign(&mut self, mut campaign: String) {
        self.campaign = campaign.clone();
    }
}

