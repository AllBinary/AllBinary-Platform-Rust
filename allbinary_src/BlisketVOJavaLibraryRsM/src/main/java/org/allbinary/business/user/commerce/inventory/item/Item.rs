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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData;
use crate::org::allbinary::business::user::address::StreetAddress::StreetAddress;
use crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData;
use crate::org::allbinary::business::user::commerce::money::MoneyException::MoneyException;
use crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
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

impl Item {
    

    pub fn new_struct1(mut itemHashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
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

    pub fn new1(mut itemHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
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
        let mut EMPTY: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
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
    

    pub fn setData(&mut self, mut itemHashMap: Option<Rc<RefCell<HashMap>>>) {
        self.group = (jcall!(itemHashMap).get(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_GROUP.with(Clone::clone).clone())).clone() /* as String */;
        self.tax = (jcall!(itemHashMap).get(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TAX.with(Clone::clone).clone())).clone() /* as String */;
        self.shippingType = (jcall!(itemHashMap).get(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_NAME.with(Clone::clone).clone())).clone() /* as String */;
        self.shippingCost = (jcall!(itemHashMap).get(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_COST.with(Clone::clone).clone())).clone() /* as String */;
        self.shippedDate = (jcall!(itemHashMap).get(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPEDDATE.with(Clone::clone).clone())).clone() /* as String */;
        jcall!(itemHashMap).put(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_NAME.with(Clone::clone).clone(), jcall!(itemHashMap).get(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_NAME.with(Clone::clone).clone()));
        jcall!(itemHashMap).put(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_STREET.with(Clone::clone).clone(), jcall!(itemHashMap).get(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_STREET.with(Clone::clone).clone()));
        jcall!(itemHashMap).put(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_CITY.with(Clone::clone).clone(), jcall!(itemHashMap).get(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_CITY.with(Clone::clone).clone()));
        jcall!(itemHashMap).put(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_STATE.with(Clone::clone).clone(), jcall!(itemHashMap).get(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_STATE.with(Clone::clone).clone()));
        jcall!(itemHashMap).put(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_CODE.with(Clone::clone).clone(), jcall!(itemHashMap).get(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_CODE.with(Clone::clone).clone()));
        jcall!(itemHashMap).put(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_COUNTRY.with(Clone::clone).clone(), jcall!(itemHashMap).get(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_COUNTRY.with(Clone::clone).clone()));
        self.shippingAddress = StreetAddress::new1(itemHashMap.clone());
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
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.base.toHashMap();
        jcall!(hashMap).put(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_GROUP.with(Clone::clone).clone(), self.group.clone());
        jcall!(hashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TAX.with(Clone::clone).clone(), self.tax.clone());
        jcall!(hashMap).put(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_NAME.with(Clone::clone).clone(), self.shippingType.clone());
        jcall!(hashMap).put(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_COST.with(Clone::clone).clone(), self.shippingCost.clone());
        jcall!(hashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPEDDATE.with(Clone::clone).clone(), self.shippedDate.clone());
        jcall!(hashMap).put(jcall!(EntryData::getInstance()).SPECIAL.clone(), self.special.clone());
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
    fn setData(&mut self, mut itemHashMap: Option<Rc<RefCell<HashMap>>>) { Item::setData(self.as_Item_mut(), itemHashMap) }
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
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::toHashMap(self) }
}


