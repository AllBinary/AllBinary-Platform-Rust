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
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData;
use crate::org::allbinary::business::user::address::StreetAddress::StreetAddress;
use crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData;
use crate::org::allbinary::business::user::commerce::money::MoneyException::MoneyException;
use crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::java::lang::Exception::*;
use super::BasicItem::BasicItem;
use super::OrderItemInterface::OrderItemInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Item {
    pub base: BasicItem,
    pub group: String,
    pub tax: String,
    pub shippingType: String,
    pub shippingCost: String,
    pub shippedDate: String,
    pub shippingAddress: Option<Rc<RefCell<StreetAddress>>>,
    pub special: String,
}

// impl OrderItemInterface for Item with the methods of the trait
impl crate::org::allbinary::business::user::commerce::inventory::item::ItemInterface::ItemInterface for Item {
    fn setId(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setId(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setNumber(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setNumber(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setInBaskets(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setInBaskets(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setWeight(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setWeight(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setEnabled(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setEnabled(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setNewOrUsed(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setNewOrUsed(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setSummary(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setSummary(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setDistributor(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setDistributor(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setIdUsedByDistributor(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setIdUsedByDistributor(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setProducedBy(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setProducedBy(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setProductionDate(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setProductionDate(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setStartProductionDate(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setStartProductionDate(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setDescription(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setDescription(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setKeywords(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setKeywords(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setCategory(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setCategory(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setType(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setType(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setSmallImage(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setSmallImage(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setMediumImage(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setMediumImage(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setLargeImage(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setLargeImage(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setTimeEntered(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setTimeEntered(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setLastModified(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setLastModified(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setPrice(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::business::user::commerce::money::Money::Money>>>) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setPrice(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setComment(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setComment(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setCustoms(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setCustoms(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setDownloads(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setDownloads(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setGroups(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setGroups(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setOptions(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setOptions(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setPermissions(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setPermissions(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn setSpecials(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::setSpecials(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self), p1) }
    fn getId(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getId(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getNumber(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getNumber(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getInBaskets(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getInBaskets(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getWeight(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getWeight(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getEnabled(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getEnabled(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getNewOrUsed(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getNewOrUsed(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getSummary(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getSummary(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getDistributor(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getDistributor(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getIdUsedByDistributor(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getIdUsedByDistributor(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getProducedBy(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getProducedBy(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getProductionDate(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getProductionDate(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getStartProductionDate(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getStartProductionDate(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getDescription(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getDescription(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getKeywords(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getKeywords(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getCategory(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getCategory(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getType(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getType(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getSmallImage(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getSmallImage(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getMediumImage(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getMediumImage(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getLargeImage(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getLargeImage(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getTimeEntered(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getTimeEntered(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getLastModified(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getLastModified(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getPrice(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::business::user::commerce::money::Money::Money>>> { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getPrice(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getComment(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getComment(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getCustoms(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getCustoms(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getDownloads(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getDownloads(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getGroups(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getGroups(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getOptions(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getOptions(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getPermissions(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getPermissions(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getSpecials(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getSpecials(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn getTotal(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::business::user::commerce::money::Money::Money>>> { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getTotal(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
}
impl crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface for Item {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::getKey(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem::toVector(crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn::as_BasicItem_mut(self)) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> { Self::toHashMap(self) }
}

impl Item {
    

    pub fn new_struct1(mut itemHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Self {
        let mut this = Self {
            base: BasicItem::new_struct1(itemHashMap.clone()),
            group: String::new(),
            tax: String::new(),
            shippingType: String::new(),
            shippingCost: String::new(),
            shippedDate: String::new(),
            shippingAddress: None,
            special: String::new(),
        };
        // super(...) is in the struct literal above
        this.setData(itemHashMap.clone());
        this
    }

    pub fn new1(mut itemHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(itemHashMap))))
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicItem::new_struct(),
            group: String::new(),
            tax: String::new(),
            shippingType: String::new(),
            shippingCost: String::new(),
            shippedDate: String::new(),
            shippingAddress: None,
            special: String::new(),
        };
        let mut EMPTY: String = { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
        this.group = EMPTY.clone();
        this.tax = EMPTY.clone();
        this.shippingType = EMPTY.clone();
        this.shippingCost = EMPTY.clone();
        this.shippingAddress = StreetAddress::new();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setData(&mut self, mut itemHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) {
        self.group = { let java_value = ({ let java_object = (jcall!(itemHashMap).get(crate::java_into_object(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_GROUP.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.tax = { let java_value = ({ let java_object = (jcall!(itemHashMap).get(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TAX.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.shippingType = { let java_value = ({ let java_object = (jcall!(itemHashMap).get(crate::java_into_object(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_NAME.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.shippingCost = { let java_value = ({ let java_object = (jcall!(itemHashMap).get(crate::java_into_object(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_COST.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        self.shippedDate = { let java_value = ({ let java_object = (jcall!(itemHashMap).get(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPEDDATE.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        jcall!(itemHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_NAME.with(Clone::clone).clone()), jcall!(itemHashMap).get(crate::java_into_object(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_NAME.with(Clone::clone).clone())));
        jcall!(itemHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_STREET.with(Clone::clone).clone()), jcall!(itemHashMap).get(crate::java_into_object(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_STREET.with(Clone::clone).clone())));
        jcall!(itemHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_CITY.with(Clone::clone).clone()), jcall!(itemHashMap).get(crate::java_into_object(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_CITY.with(Clone::clone).clone())));
        jcall!(itemHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_STATE.with(Clone::clone).clone()), jcall!(itemHashMap).get(crate::java_into_object(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_STATE.with(Clone::clone).clone())));
        jcall!(itemHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_CODE.with(Clone::clone).clone()), jcall!(itemHashMap).get(crate::java_into_object(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_CODE.with(Clone::clone).clone())));
        jcall!(itemHashMap).put(crate::java_into_object(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_COUNTRY.with(Clone::clone).clone()), jcall!(itemHashMap).get(crate::java_into_object(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_COUNTRY.with(Clone::clone).clone())));
        self.shippingAddress = StreetAddress::new1_HashMap(itemHashMap.clone());
    }
    

    pub fn setGroup(&mut self, mut value: String) {
        self.group = value.clone();
    }
    

    pub fn setTax(&mut self, mut value: String) {
        self.tax = value.clone();
    }
    

    pub fn setShippingType(&mut self, mut value: String) {
        self.shippingType = value.clone();
    }
    

    pub fn setShippingCost(&mut self, mut value: String) {
        self.shippingCost = value.clone();
    }
    

    pub fn setShippingAddress(&mut self, mut value: Option<Rc<RefCell<StreetAddress>>>) {
        self.shippingAddress = value.clone();
    }
    

    pub fn setSpecial(&mut self, mut value: String) {
        self.special = value.clone();
    }
    

    pub fn getGroup(&mut self) -> String {
        return self.group.clone();
    }
    

    pub fn getTax(&mut self) -> String {
        return self.tax.clone();
    }
    

    pub fn getShippingType(&mut self) -> String {
        return self.shippingType.clone();
    }
    

    pub fn getShippingCost(&mut self) -> String {
        return self.shippingCost.clone();
    }
    

    pub fn getShippingAddress(&mut self) -> Option<Rc<RefCell<StreetAddress>>> {
        return self.shippingAddress.clone();
    }
    

    pub fn getSpecial(&mut self) -> String {
        return self.special.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = self.base.toHashMap();
        jcall!(hashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_GROUP.with(Clone::clone).clone()), crate::java_into_object(self.group.clone()));
        jcall!(hashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TAX.with(Clone::clone).clone()), crate::java_into_object(self.tax.clone()));
        jcall!(hashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_NAME.with(Clone::clone).clone()), crate::java_into_object(self.shippingType.clone()));
        jcall!(hashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_COST.with(Clone::clone).clone()), crate::java_into_object(self.shippingCost.clone()));
        jcall!(hashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPEDDATE.with(Clone::clone).clone()), crate::java_into_object(self.shippedDate.clone()));
        jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).SPECIAL.clone(); java_field }.clone()), crate::java_into_object(self.special.clone()));
        return hashMap.clone();
    }
}
impl std::ops::Deref for Item {
    type Target = BasicItem;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for Item {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ItemDyn: crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn {
    fn as_Item(&self) -> &Item;
    fn as_Item_mut(&mut self) -> &mut Item;
    fn setData(&mut self, mut itemHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) { Item::setData(self.as_Item_mut(), itemHashMap) }
    fn setGroup(&mut self, mut value: String) { Item::setGroup(self.as_Item_mut(), value) }
    fn setTax(&mut self, mut value: String) { Item::setTax(self.as_Item_mut(), value) }
    fn setShippingType(&mut self, mut value: String) { Item::setShippingType(self.as_Item_mut(), value) }
    fn setShippingCost(&mut self, mut value: String) { Item::setShippingCost(self.as_Item_mut(), value) }
    fn setShippingAddress(&mut self, mut value: Option<Rc<RefCell<StreetAddress>>>) { Item::setShippingAddress(self.as_Item_mut(), value) }
    fn setSpecial(&mut self, mut value: String) { Item::setSpecial(self.as_Item_mut(), value) }
    fn getGroup(&mut self) -> String { Item::getGroup(self.as_Item_mut()) }
    fn getTax(&mut self) -> String { Item::getTax(self.as_Item_mut()) }
    fn getShippingType(&mut self) -> String { Item::getShippingType(self.as_Item_mut()) }
    fn getShippingCost(&mut self) -> String { Item::getShippingCost(self.as_Item_mut()) }
    fn getShippingAddress(&mut self) -> Option<Rc<RefCell<StreetAddress>>> { Item::getShippingAddress(self.as_Item_mut()) }
    fn getSpecial(&mut self) -> String { Item::getSpecial(self.as_Item_mut()) }
}
impl ItemDyn for Item {
    fn as_Item(&self) -> &Item { self }
    fn as_Item_mut(&mut self) -> &mut Item { self }
}

impl crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItemDyn for Item {
    fn as_BasicItem(&self) -> &crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem { &self.base }
    fn as_BasicItem_mut(&mut self) -> &mut crate::org::allbinary::business::user::commerce::inventory::item::BasicItem::BasicItem { &mut self.base }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> { Self::toHashMap(self) }
}


