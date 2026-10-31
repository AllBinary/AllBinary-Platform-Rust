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
// package org.allbinary.business.user.commerce.inventory.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Calendar;
use std::collections::HashMap;
use super::*;
// use crate::java::util::Calendar::Calendar;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::user::commerce::money::Money::Money;
use crate::org::allbinary::business::user::commerce::money::MoneyException::MoneyException;
use crate::org::allbinary::data::generator::ProductIdGenerator::ProductIdGenerator;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::ItemInterface::ItemInterface;
use super::BasicItemData::BasicItemData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicItem {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub basicItemData: Option<Rc<RefCell<BasicItemData>>>,
    pub itemId: String,
    pub number: String,
    pub inBaskets: String,
    pub weight: String,
    pub enabled: String,
    pub newOrUsed: String,
    pub summary: String,
    pub distributor: String,
    pub idUsedByDistributor: String,
    pub producedBy: String,
    pub productionDate: String,
    pub startProductionDate: String,
    pub description: String,
    pub keywords: String,
    pub category: String,
    pub r#type: String,
    pub smallImage: String,
    pub mediumImage: String,
    pub largeImage: String,
    pub price: Option<Rc<RefCell<Money>>>,
    pub comment: String,
    pub customs: String,
    pub downloads: String,
    pub groups: String,
    pub options: String,
    pub permissions: String,
    pub specials: String,
    pub downloadable: bool,
    pub timeEntered: String,
    pub lastModified: String,
}

impl ItemInterface for BasicItem {
    fn setId(&mut self, mut itemId: String) { Self::setId(self, itemId) }
    fn setNumber(&mut self, mut number: String) { Self::setNumber(self, number) }
    fn setInBaskets(&mut self, mut value: String) { Self::setInBaskets(self, value) }
    fn setWeight(&mut self, mut value: String) { Self::setWeight(self, value) }
    fn setEnabled(&mut self, mut value: String) { Self::setEnabled(self, value) }
    fn setNewOrUsed(&mut self, mut value: String) { Self::setNewOrUsed(self, value) }
    fn setSummary(&mut self, mut value: String) { Self::setSummary(self, value) }
    fn setDistributor(&mut self, mut value: String) { Self::setDistributor(self, value) }
    fn setIdUsedByDistributor(&mut self, mut value: String) { Self::setIdUsedByDistributor(self, value) }
    fn setProducedBy(&mut self, mut value: String) { Self::setProducedBy(self, value) }
    fn setProductionDate(&mut self, mut value: String) { Self::setProductionDate(self, value) }
    fn setStartProductionDate(&mut self, mut value: String) { Self::setStartProductionDate(self, value) }
    fn setDescription(&mut self, mut value: String) { Self::setDescription(self, value) }
    fn setKeywords(&mut self, mut value: String) { Self::setKeywords(self, value) }
    fn setCategory(&mut self, mut value: String) { Self::setCategory(self, value) }
    fn setType(&mut self, mut value: String) { Self::setType(self, value) }
    fn setSmallImage(&mut self, mut value: String) { Self::setSmallImage(self, value) }
    fn setMediumImage(&mut self, mut value: String) { Self::setMediumImage(self, value) }
    fn setLargeImage(&mut self, mut value: String) { Self::setLargeImage(self, value) }
    fn setTimeEntered(&mut self, mut value: String) { Self::setTimeEntered(self, value) }
    fn setLastModified(&mut self, mut value: String) { Self::setLastModified(self, value) }
    fn setPrice(&mut self, mut value: Option<Rc<RefCell<Money>>>) { Self::setPrice(self, value) }
    fn setComment(&mut self, mut value: String) { Self::setComment(self, value) }
    fn setCustoms(&mut self, mut value: String) { Self::setCustoms(self, value) }
    fn setDownloads(&mut self, mut value: String) { Self::setDownloads(self, value) }
    fn setGroups(&mut self, mut value: String) { Self::setGroups(self, value) }
    fn setOptions(&mut self, mut value: String) { Self::setOptions(self, value) }
    fn setPermissions(&mut self, mut value: String) { Self::setPermissions(self, value) }
    fn setSpecials(&mut self, mut value: String) { Self::setSpecials(self, value) }
    fn getId(&mut self) -> String { Self::getId(self) }
    fn getNumber(&mut self) -> String { Self::getNumber(self) }
    fn getInBaskets(&mut self) -> String { Self::getInBaskets(self) }
    fn getWeight(&mut self) -> String { Self::getWeight(self) }
    fn getEnabled(&mut self) -> String { Self::getEnabled(self) }
    fn getNewOrUsed(&mut self) -> String { Self::getNewOrUsed(self) }
    fn getSummary(&mut self) -> String { Self::getSummary(self) }
    fn getDistributor(&mut self) -> String { Self::getDistributor(self) }
    fn getIdUsedByDistributor(&mut self) -> String { Self::getIdUsedByDistributor(self) }
    fn getProducedBy(&mut self) -> String { Self::getProducedBy(self) }
    fn getProductionDate(&mut self) -> String { Self::getProductionDate(self) }
    fn getStartProductionDate(&mut self) -> String { Self::getStartProductionDate(self) }
    fn getDescription(&mut self) -> String { Self::getDescription(self) }
    fn getKeywords(&mut self) -> String { Self::getKeywords(self) }
    fn getCategory(&mut self) -> String { Self::getCategory(self) }
    fn getType(&mut self) -> String { Self::getType(self) }
    fn getSmallImage(&mut self) -> String { Self::getSmallImage(self) }
    fn getMediumImage(&mut self) -> String { Self::getMediumImage(self) }
    fn getLargeImage(&mut self) -> String { Self::getLargeImage(self) }
    fn getTimeEntered(&mut self) -> String { Self::getTimeEntered(self) }
    fn getLastModified(&mut self) -> String { Self::getLastModified(self) }
    fn getPrice(&mut self) -> Option<Rc<RefCell<Money>>> { Self::getPrice(self) }
    fn getComment(&mut self) -> String { Self::getComment(self) }
    fn getCustoms(&mut self) -> String { Self::getCustoms(self) }
    fn getDownloads(&mut self) -> String { Self::getDownloads(self) }
    fn getGroups(&mut self) -> String { Self::getGroups(self) }
    fn getOptions(&mut self) -> String { Self::getOptions(self) }
    fn getPermissions(&mut self) -> String { Self::getPermissions(self) }
    fn getSpecials(&mut self) -> String { Self::getSpecials(self) }
    fn getTotal(&mut self) -> Option<Rc<RefCell<Money>>> { Self::getTotal(self) }
}
impl crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface for BasicItem {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toVector(self) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::toHashMap(self) }
}

impl BasicItem {
    

    pub fn new_struct1(mut itemHashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            basicItemData: None,
            itemId: String::new(),
            number: String::new(),
            inBaskets: String::new(),
            weight: String::new(),
            enabled: String::new(),
            newOrUsed: String::new(),
            summary: String::new(),
            distributor: String::new(),
            idUsedByDistributor: String::new(),
            producedBy: String::new(),
            productionDate: String::new(),
            startProductionDate: String::new(),
            description: String::new(),
            keywords: String::new(),
            category: String::new(),
            r#type: String::new(),
            smallImage: String::new(),
            mediumImage: String::new(),
            largeImage: String::new(),
            price: None,
            comment: String::new(),
            customs: String::new(),
            downloads: String::new(),
            groups: String::new(),
            options: String::new(),
            permissions: String::new(),
            specials: String::new(),
            downloadable: false,
            timeEntered: String::new(),
            lastModified: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.basicItemData = BasicItemData::getInstance();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRODUCTSEARCHLOGGING.clone()) {
            jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("Constructor(HashMap)"));
        }
        this.itemId = (jcall!(itemHashMap).get(jcall!(this.basicItemData).ID.clone())).clone() /* as String */;
        this.number = (jcall!(itemHashMap).get(jcall!(this.basicItemData).NUMBER.clone())).clone() /* as String */;
        this.inBaskets = (jcall!(itemHashMap).get(jcall!(this.basicItemData).INBASKETS.clone())).clone() /* as String */;
        this.weight = (jcall!(itemHashMap).get(jcall!(this.basicItemData).WEIGHT.clone())).clone() /* as String */;
        this.enabled = (jcall!(itemHashMap).get(jcall!(EntryData::getInstance()).ENABLE.clone())).clone() /* as String */;
        this.newOrUsed = (jcall!(itemHashMap).get(jcall!(this.basicItemData).NEWORUSED.clone())).clone() /* as String */;
        this.summary = (jcall!(itemHashMap).get(jcall!(this.basicItemData).SUMMARY.clone())).clone() /* as String */;
        this.distributor = (jcall!(itemHashMap).get(jcall!(this.basicItemData).DISTRIBUTOR.clone())).clone() /* as String */;
        this.idUsedByDistributor = (jcall!(itemHashMap).get(jcall!(this.basicItemData).IDUSEDBYDISTRIBUTOR.clone())).clone() /* as String */;
        this.producedBy = (jcall!(itemHashMap).get(jcall!(this.basicItemData).PRODUCEDBY.clone())).clone() /* as String */;
        this.productionDate = (jcall!(itemHashMap).get(jcall!(this.basicItemData).PRODUCTIONDATE.clone())).clone() /* as String */;
        this.startProductionDate = (jcall!(itemHashMap).get(jcall!(this.basicItemData).STARTPRODUCTIONDATE.clone())).clone() /* as String */;
        this.description = (jcall!(itemHashMap).get(jcall!(this.basicItemData).DESCRIPTION.clone())).clone() /* as String */;
        this.keywords = (jcall!(itemHashMap).get(jcall!(this.basicItemData).KEYWORDS.clone())).clone() /* as String */;
        this.category = (jcall!(itemHashMap).get(jcall!(this.basicItemData).CATEGORY.clone())).clone() /* as String */;
        this.r#type = (jcall!(itemHashMap).get(jcall!(this.basicItemData).TYPE.clone())).clone() /* as String */;
        this.smallImage = (jcall!(itemHashMap).get(jcall!(this.basicItemData).SMALLIMAGE.clone())).clone() /* as String */;
        this.mediumImage = (jcall!(itemHashMap).get(jcall!(this.basicItemData).MEDIUMIMAGE.clone())).clone() /* as String */;
        this.largeImage = (jcall!(itemHashMap).get(jcall!(this.basicItemData).LARGEIMAGE.clone())).clone() /* as String */;
        this.timeEntered = (jcall!(itemHashMap).get(jcall!(EntryData::getInstance()).TIMECREATED.clone())).clone() /* as String */;
        this.lastModified = (jcall!(itemHashMap).get(jcall!(EntryData::getInstance()).LASTMODIFIED.clone())).clone() /* as String */;
        this.price = Money::new1((jcall!(itemHashMap).get(jcall!(this.basicItemData).PRICE.clone())).clone() /* as String */);
        this.comment = (jcall!(itemHashMap).get(jcall!(this.basicItemData).COMMENT.clone())).clone() /* as String */;
        this.customs = (jcall!(itemHashMap).get(jcall!(this.basicItemData).CUSTOMS.clone())).clone() /* as String */;
        { let java_arguments = ((jcall!(itemHashMap).get(jcall!(this.basicItemData).DOWNLOADS.clone())).clone() /* as String */,); this.setDownloads(java_arguments.0) };
        this.groups = (jcall!(itemHashMap).get(jcall!(this.basicItemData).GROUPS.clone())).clone() /* as String */;
        this.options = (jcall!(itemHashMap).get(jcall!(this.basicItemData).OPTIONS.clone())).clone() /* as String */;
        this.permissions = (jcall!(itemHashMap).get(jcall!(this.basicItemData).PERMISSIONS.clone())).clone() /* as String */;
        this.specials = (jcall!(itemHashMap).get(jcall!(this.basicItemData).SPECIALS.clone())).clone() /* as String */;
        this
    }

    pub fn new1(mut itemHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(itemHashMap))))
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            basicItemData: None,
            itemId: String::new(),
            number: String::new(),
            inBaskets: String::new(),
            weight: String::new(),
            enabled: String::new(),
            newOrUsed: String::new(),
            summary: String::new(),
            distributor: String::new(),
            idUsedByDistributor: String::new(),
            producedBy: String::new(),
            productionDate: String::new(),
            startProductionDate: String::new(),
            description: String::new(),
            keywords: String::new(),
            category: String::new(),
            r#type: String::new(),
            smallImage: String::new(),
            mediumImage: String::new(),
            largeImage: String::new(),
            price: None,
            comment: String::new(),
            customs: String::new(),
            downloads: String::new(),
            groups: String::new(),
            options: String::new(),
            permissions: String::new(),
            specials: String::new(),
            downloadable: false,
            timeEntered: String::new(),
            lastModified: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.basicItemData = BasicItemData::getInstance();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRODUCTSEARCHLOGGING.clone()) {
            jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        }
        let mut EMPTY: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.itemId = jcall!(ProductIdGenerator::new()).getNext();
        this.number = jcall!(CommonPhoneStrings::getInstance()).ZERO.clone();
        this.inBaskets = EMPTY.clone();
        this.weight = EMPTY.clone();
        this.enabled = EMPTY.clone();
        this.newOrUsed = EMPTY.clone();
        this.summary = EMPTY.clone();
        this.distributor = EMPTY.clone();
        this.idUsedByDistributor = EMPTY.clone();
        this.producedBy = EMPTY.clone();
        this.productionDate = EMPTY.clone();
        this.startProductionDate = EMPTY.clone();
        this.description = EMPTY.clone();
        this.keywords = EMPTY.clone();
        this.category = EMPTY.clone();
        this.r#type = EMPTY.clone();
        this.smallImage = EMPTY.clone();
        this.mediumImage = EMPTY.clone();
        this.largeImage = EMPTY.clone();
        this.timeEntered = EMPTY.clone();
        this.lastModified = EMPTY.clone();
        this.price = Money::new();
        this.comment = EMPTY.clone();
        this.customs = EMPTY.clone();
        this.setDownloads(EMPTY.clone());
        this.groups = EMPTY.clone();
        this.options = EMPTY.clone();
        this.permissions = EMPTY.clone();
        this.specials = EMPTY.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setId(&mut self, mut itemId: String) {
        self.itemId = itemId.clone();
    }
    

    pub fn setNumber(&mut self, mut number: String) {
        self.number = number.clone();
    }
    

    pub fn setInBaskets(&mut self, mut value: String) {
        self.inBaskets = value.clone();
    }
    

    pub fn setWeight(&mut self, mut value: String) {
        self.weight = value.clone();
    }
    

    pub fn setNewOrUsed(&mut self, mut value: String) {
        self.newOrUsed = value.clone();
    }
    

    pub fn setSummary(&mut self, mut value: String) {
        self.summary = value.clone();
    }
    

    pub fn setDistributor(&mut self, mut value: String) {
        self.distributor = value.clone();
    }
    

    pub fn setIdUsedByDistributor(&mut self, mut value: String) {
        self.idUsedByDistributor = value.clone();
    }
    

    pub fn setProducedBy(&mut self, mut value: String) {
        self.producedBy = value.clone();
    }
    

    pub fn setProductionDate(&mut self, mut value: String) {
        self.productionDate = value.clone();
    }
    

    pub fn setStartProductionDate(&mut self, mut value: String) {
        self.startProductionDate = value.clone();
    }
    

    pub fn setDescription(&mut self, mut value: String) {
        self.description = value.clone();
    }
    

    pub fn setKeywords(&mut self, mut value: String) {
        self.keywords = value.clone();
    }
    

    pub fn setCategory(&mut self, mut value: String) {
        self.category = value.clone();
    }
    

    pub fn setType(&mut self, mut value: String) {
        self.r#type = value.clone();
    }
    

    pub fn setSmallImage(&mut self, mut value: String) {
        self.smallImage = value.clone();
    }
    

    pub fn setMediumImage(&mut self, mut value: String) {
        self.mediumImage = value.clone();
    }
    

    pub fn setLargeImage(&mut self, mut value: String) {
        self.largeImage = value.clone();
    }
    

    pub fn setTimeEntered(&mut self, mut value: String) {
        self.timeEntered = value.clone();
    }
    

    pub fn setLastModified(&mut self, mut value: String) {
        self.lastModified = value.clone();
    }
    

    pub fn setPrice(&mut self, mut value: Option<Rc<RefCell<Money>>>) {
        self.price = value.clone();
    }
    

    pub fn setComment(&mut self, mut value: String) {
        self.comment = value.clone();
    }
    

    pub fn setCustoms(&mut self, mut value: String) {
        self.customs = value.clone();
    }
    

    pub fn setDownloads(&mut self, mut value: String) {
        self.downloads = value.clone();
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(self.downloads.clone()) {
            let mut downloadInteger: Option<i32> = (self.downloads.clone()).to_string();
            if ((downloadInteger).unwrap() as i32) != 0 {
                self.setDownloadable(true);
            }
        }
    }
    

    pub fn setGroups(&mut self, mut value: String) {
        self.groups = value.clone();
    }
    

    pub fn setOptions(&mut self, mut value: String) {
        self.options = value.clone();
    }
    

    pub fn setPermissions(&mut self, mut value: String) {
        self.permissions = value.clone();
    }
    

    pub fn setSpecials(&mut self, mut value: String) {
        self.specials = value.clone();
    }
    

    pub fn setEnabled(&mut self, mut value: String) {
        self.enabled = value.clone();
    }
    

    pub fn getId(&mut self) -> String {
        return self.itemId.clone();
    }
    

    pub fn getNumber(&mut self) -> String {
        return self.number.clone();
    }
    

    pub fn getInBaskets(&mut self) -> String {
        return self.inBaskets.clone();
    }
    

    pub fn getWeight(&mut self) -> String {
        return self.weight.clone();
    }
    

    pub fn getNewOrUsed(&mut self) -> String {
        return self.newOrUsed.clone();
    }
    

    pub fn getSummary(&mut self) -> String {
        return self.summary.clone();
    }
    

    pub fn getDistributor(&mut self) -> String {
        return self.distributor.clone();
    }
    

    pub fn getIdUsedByDistributor(&mut self) -> String {
        return self.idUsedByDistributor.clone();
    }
    

    pub fn getProducedBy(&mut self) -> String {
        return self.producedBy.clone();
    }
    

    pub fn getProductionDate(&mut self) -> String {
        return self.productionDate.clone();
    }
    

    pub fn getStartProductionDate(&mut self) -> String {
        return self.startProductionDate.clone();
    }
    

    pub fn getDescription(&mut self) -> String {
        return self.description.clone();
    }
    

    pub fn getKeywords(&mut self) -> String {
        return self.keywords.clone();
    }
    

    pub fn getCategory(&mut self) -> String {
        return self.category.clone();
    }
    

    pub fn getType(&mut self) -> String {
        return self.r#type.clone();
    }
    

    pub fn getSmallImage(&mut self) -> String {
        return self.smallImage.clone();
    }
    

    pub fn getMediumImage(&mut self) -> String {
        return self.mediumImage.clone();
    }
    

    pub fn getLargeImage(&mut self) -> String {
        return self.largeImage.clone();
    }
    

    pub fn getTimeEntered(&mut self) -> String {
        return self.timeEntered.clone();
    }
    

    pub fn getLastModified(&mut self) -> String {
        return self.lastModified.clone();
    }
    

    pub fn getPrice(&mut self) -> Option<Rc<RefCell<Money>>> {
        return self.price.clone();
    }
    

    pub fn getComment(&mut self) -> String {
        return self.comment.clone();
    }
    

    pub fn getCustoms(&mut self) -> String {
        return self.customs.clone();
    }
    

    pub fn getDownloads(&mut self) -> String {
        return self.downloads.clone();
    }
    

    pub fn getGroups(&mut self) -> String {
        return self.groups.clone();
    }
    

    pub fn getOptions(&mut self) -> String {
        return self.options.clone();
    }
    

    pub fn getPermissions(&mut self) -> String {
        return self.permissions.clone();
    }
    

    pub fn getSpecials(&mut self) -> String {
        return self.specials.clone();
    }
    

    pub fn setDownloadable(&mut self, mut downloadable: bool) {
        self.downloadable = downloadable.clone();
    }
    

    pub fn isDownloadable(&mut self) -> bool {
        return self.downloadable.clone();
    }
    

    pub fn getEnabled(&mut self) -> String {
        return self.enabled.clone();
    }
    

    pub fn getTotal(&mut self) -> Option<Rc<RefCell<Money>>> {
        let mut itemTotal: Option<Rc<RefCell<Money>>> = Money::new1(self.price.clone());
        jcall!(itemTotal).multiply(((Some((self.number.clone()) as i32)).unwrap() as i32));
        return itemTotal.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRODUCTSEARCHLOGGING.clone()) {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toHashMap"));
        }
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(jcall!(self.basicItemData).ID.clone(), self.itemId.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).NUMBER.clone(), self.number.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).INBASKETS.clone(), self.inBaskets.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).WEIGHT.clone(), self.weight.clone());
        jcall!(hashMap).put(jcall!(EntryData::getInstance()).ENABLE.clone(), self.enabled.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).NEWORUSED.clone(), self.newOrUsed.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).SUMMARY.clone(), self.summary.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).DISTRIBUTOR.clone(), self.distributor.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).IDUSEDBYDISTRIBUTOR.clone(), self.idUsedByDistributor.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).PRODUCEDBY.clone(), self.producedBy.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).PRODUCTIONDATE.clone(), self.productionDate.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).STARTPRODUCTIONDATE.clone(), self.startProductionDate.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).DESCRIPTION.clone(), self.description.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).KEYWORDS.clone(), self.keywords.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).CATEGORY.clone(), self.category.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).TYPE.clone(), self.r#type.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).SMALLIMAGE.clone(), self.smallImage.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).MEDIUMIMAGE.clone(), self.mediumImage.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).LARGEIMAGE.clone(), self.largeImage.clone());
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString();
        jcall!(hashMap).put(jcall!(EntryData::getInstance()).LASTMODIFIED.clone(), time.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).PRICE.clone(), jcall!(self.price).toString());
        jcall!(hashMap).put(jcall!(self.basicItemData).COMMENT.clone(), self.comment.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).CUSTOMS.clone(), self.customs.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).DOWNLOADS.clone(), self.getDownloads());
        jcall!(hashMap).put(jcall!(self.basicItemData).GROUPS.clone(), self.groups.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).OPTIONS.clone(), self.options.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).PERMISSIONS.clone(), self.permissions.clone());
        jcall!(hashMap).put(jcall!(self.basicItemData).SPECIALS.clone(), self.specials.clone());
        return hashMap.clone();
    }
    

    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRODUCTSEARCHLOGGING.clone()) {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toVector"));
        }
        let mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(values).add(crate::java_into_object(self.itemId.clone()));
        jcall!(values).add(crate::java_into_object(self.number.clone()));
        jcall!(values).add(crate::java_into_object(self.inBaskets.clone()));
        jcall!(values).add(crate::java_into_object(self.weight.clone()));
        jcall!(values).add(crate::java_into_object(self.enabled.clone()));
        jcall!(values).add(crate::java_into_object(self.newOrUsed.clone()));
        jcall!(values).add(crate::java_into_object(self.summary.clone()));
        jcall!(values).add(crate::java_into_object(self.distributor.clone()));
        jcall!(values).add(crate::java_into_object(self.idUsedByDistributor.clone()));
        jcall!(values).add(crate::java_into_object(self.producedBy.clone()));
        jcall!(values).add(crate::java_into_object(self.productionDate.clone()));
        jcall!(values).add(crate::java_into_object(self.startProductionDate.clone()));
        jcall!(values).add(crate::java_into_object(self.description.clone()));
        jcall!(values).add(crate::java_into_object(self.keywords.clone()));
        jcall!(values).add(crate::java_into_object(self.category.clone()));
        jcall!(values).add(crate::java_into_object(self.r#type.clone()));
        jcall!(values).add(crate::java_into_object(self.getSmallImage()));
        jcall!(values).add(crate::java_into_object(self.getMediumImage()));
        jcall!(values).add(crate::java_into_object(self.getLargeImage()));
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString();
        jcall!(values).add(crate::java_into_object(time.clone()));
        jcall!(values).add(crate::java_into_object(time.clone()));
        jcall!(values).add(crate::java_into_object(jcall!(self.price).toString()));
        jcall!(values).add(crate::java_into_object(self.comment.clone()));
        jcall!(values).add(crate::java_into_object(self.customs.clone()));
        jcall!(values).add(crate::java_into_object(self.getDownloads()));
        jcall!(values).add(crate::java_into_object(self.groups.clone()));
        jcall!(values).add(crate::java_into_object(self.options.clone()));
        jcall!(values).add(crate::java_into_object(self.permissions.clone()));
        jcall!(values).add(crate::java_into_object(self.specials.clone()));
        return values.clone();
    }
    

    // throws Exception
    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return (self.getId()).clone() /* as Any */;
    }
}
pub trait BasicItemDyn {
    fn as_BasicItem(&self) -> &BasicItem;
    fn as_BasicItem_mut(&mut self) -> &mut BasicItem;
    fn setId(&mut self, mut itemId: String) { BasicItem::setId(self.as_BasicItem_mut(), itemId) }
    fn setNumber(&mut self, mut number: String) { BasicItem::setNumber(self.as_BasicItem_mut(), number) }
    fn setInBaskets(&mut self, mut value: String) { BasicItem::setInBaskets(self.as_BasicItem_mut(), value) }
    fn setWeight(&mut self, mut value: String) { BasicItem::setWeight(self.as_BasicItem_mut(), value) }
    fn setNewOrUsed(&mut self, mut value: String) { BasicItem::setNewOrUsed(self.as_BasicItem_mut(), value) }
    fn setSummary(&mut self, mut value: String) { BasicItem::setSummary(self.as_BasicItem_mut(), value) }
    fn setDistributor(&mut self, mut value: String) { BasicItem::setDistributor(self.as_BasicItem_mut(), value) }
    fn setIdUsedByDistributor(&mut self, mut value: String) { BasicItem::setIdUsedByDistributor(self.as_BasicItem_mut(), value) }
    fn setProducedBy(&mut self, mut value: String) { BasicItem::setProducedBy(self.as_BasicItem_mut(), value) }
    fn setProductionDate(&mut self, mut value: String) { BasicItem::setProductionDate(self.as_BasicItem_mut(), value) }
    fn setStartProductionDate(&mut self, mut value: String) { BasicItem::setStartProductionDate(self.as_BasicItem_mut(), value) }
    fn setDescription(&mut self, mut value: String) { BasicItem::setDescription(self.as_BasicItem_mut(), value) }
    fn setKeywords(&mut self, mut value: String) { BasicItem::setKeywords(self.as_BasicItem_mut(), value) }
    fn setCategory(&mut self, mut value: String) { BasicItem::setCategory(self.as_BasicItem_mut(), value) }
    fn setType(&mut self, mut value: String) { BasicItem::setType(self.as_BasicItem_mut(), value) }
    fn setSmallImage(&mut self, mut value: String) { BasicItem::setSmallImage(self.as_BasicItem_mut(), value) }
    fn setMediumImage(&mut self, mut value: String) { BasicItem::setMediumImage(self.as_BasicItem_mut(), value) }
    fn setLargeImage(&mut self, mut value: String) { BasicItem::setLargeImage(self.as_BasicItem_mut(), value) }
    fn setTimeEntered(&mut self, mut value: String) { BasicItem::setTimeEntered(self.as_BasicItem_mut(), value) }
    fn setLastModified(&mut self, mut value: String) { BasicItem::setLastModified(self.as_BasicItem_mut(), value) }
    fn setPrice(&mut self, mut value: Option<Rc<RefCell<Money>>>) { BasicItem::setPrice(self.as_BasicItem_mut(), value) }
    fn setComment(&mut self, mut value: String) { BasicItem::setComment(self.as_BasicItem_mut(), value) }
    fn setCustoms(&mut self, mut value: String) { BasicItem::setCustoms(self.as_BasicItem_mut(), value) }
    fn setDownloads(&mut self, mut value: String) { BasicItem::setDownloads(self.as_BasicItem_mut(), value) }
    fn setGroups(&mut self, mut value: String) { BasicItem::setGroups(self.as_BasicItem_mut(), value) }
    fn setOptions(&mut self, mut value: String) { BasicItem::setOptions(self.as_BasicItem_mut(), value) }
    fn setPermissions(&mut self, mut value: String) { BasicItem::setPermissions(self.as_BasicItem_mut(), value) }
    fn setSpecials(&mut self, mut value: String) { BasicItem::setSpecials(self.as_BasicItem_mut(), value) }
    fn setEnabled(&mut self, mut value: String) { BasicItem::setEnabled(self.as_BasicItem_mut(), value) }
    fn getId(&mut self) -> String { BasicItem::getId(self.as_BasicItem_mut()) }
    fn getNumber(&mut self) -> String { BasicItem::getNumber(self.as_BasicItem_mut()) }
    fn getInBaskets(&mut self) -> String { BasicItem::getInBaskets(self.as_BasicItem_mut()) }
    fn getWeight(&mut self) -> String { BasicItem::getWeight(self.as_BasicItem_mut()) }
    fn getNewOrUsed(&mut self) -> String { BasicItem::getNewOrUsed(self.as_BasicItem_mut()) }
    fn getSummary(&mut self) -> String { BasicItem::getSummary(self.as_BasicItem_mut()) }
    fn getDistributor(&mut self) -> String { BasicItem::getDistributor(self.as_BasicItem_mut()) }
    fn getIdUsedByDistributor(&mut self) -> String { BasicItem::getIdUsedByDistributor(self.as_BasicItem_mut()) }
    fn getProducedBy(&mut self) -> String { BasicItem::getProducedBy(self.as_BasicItem_mut()) }
    fn getProductionDate(&mut self) -> String { BasicItem::getProductionDate(self.as_BasicItem_mut()) }
    fn getStartProductionDate(&mut self) -> String { BasicItem::getStartProductionDate(self.as_BasicItem_mut()) }
    fn getDescription(&mut self) -> String { BasicItem::getDescription(self.as_BasicItem_mut()) }
    fn getKeywords(&mut self) -> String { BasicItem::getKeywords(self.as_BasicItem_mut()) }
    fn getCategory(&mut self) -> String { BasicItem::getCategory(self.as_BasicItem_mut()) }
    fn getType(&mut self) -> String { BasicItem::getType(self.as_BasicItem_mut()) }
    fn getSmallImage(&mut self) -> String { BasicItem::getSmallImage(self.as_BasicItem_mut()) }
    fn getMediumImage(&mut self) -> String { BasicItem::getMediumImage(self.as_BasicItem_mut()) }
    fn getLargeImage(&mut self) -> String { BasicItem::getLargeImage(self.as_BasicItem_mut()) }
    fn getTimeEntered(&mut self) -> String { BasicItem::getTimeEntered(self.as_BasicItem_mut()) }
    fn getLastModified(&mut self) -> String { BasicItem::getLastModified(self.as_BasicItem_mut()) }
    fn getPrice(&mut self) -> Option<Rc<RefCell<Money>>> { BasicItem::getPrice(self.as_BasicItem_mut()) }
    fn getComment(&mut self) -> String { BasicItem::getComment(self.as_BasicItem_mut()) }
    fn getCustoms(&mut self) -> String { BasicItem::getCustoms(self.as_BasicItem_mut()) }
    fn getDownloads(&mut self) -> String { BasicItem::getDownloads(self.as_BasicItem_mut()) }
    fn getGroups(&mut self) -> String { BasicItem::getGroups(self.as_BasicItem_mut()) }
    fn getOptions(&mut self) -> String { BasicItem::getOptions(self.as_BasicItem_mut()) }
    fn getPermissions(&mut self) -> String { BasicItem::getPermissions(self.as_BasicItem_mut()) }
    fn getSpecials(&mut self) -> String { BasicItem::getSpecials(self.as_BasicItem_mut()) }
    fn setDownloadable(&mut self, mut downloadable: bool) { BasicItem::setDownloadable(self.as_BasicItem_mut(), downloadable) }
    fn isDownloadable(&mut self) -> bool { BasicItem::isDownloadable(self.as_BasicItem_mut()) }
    fn getEnabled(&mut self) -> String { BasicItem::getEnabled(self.as_BasicItem_mut()) }
    fn getTotal(&mut self) -> Option<Rc<RefCell<Money>>> { BasicItem::getTotal(self.as_BasicItem_mut()) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { BasicItem::toHashMap(self.as_BasicItem_mut()) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { BasicItem::toVector(self.as_BasicItem_mut()) }
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { BasicItem::getKey(self.as_BasicItem_mut()) }
}
impl BasicItemDyn for BasicItem {
    fn as_BasicItem(&self) -> &BasicItem { self }
    fn as_BasicItem_mut(&mut self) -> &mut BasicItem { self }
}


