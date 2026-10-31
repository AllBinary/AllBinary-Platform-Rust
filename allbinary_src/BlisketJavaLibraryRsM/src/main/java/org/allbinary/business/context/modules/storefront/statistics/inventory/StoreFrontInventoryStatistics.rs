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
// package org.allbinary.business.context.modules.storefront.statistics.inventory
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::commerce::inventory::item::ItemInterface::ItemInterface;
use crate::org::allbinary::business::user::commerce::money::Money::Money;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntityFactory::InventoryEntityFactory;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntityInterface::InventoryEntityInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use super::StoreFrontInventoryStatisticsInterface::StoreFrontInventoryStatisticsInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreFrontInventoryStatistics {
    pub totalNumberOfItems: i64,
    pub totalInventorySaleValueMoney: Option<Rc<RefCell<Money>>>,
}

impl StoreFrontInventoryStatisticsInterface for StoreFrontInventoryStatistics {
    fn getNumber(&mut self) -> Option<i64> { Self::getNumber(self) }
    fn getTotal(&mut self) -> Option<Rc<RefCell<Money>>> { Self::getTotal(self) }
}
impl crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface for StoreFrontInventoryStatistics {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toVector(self) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::toHashMap(self) }
}

impl StoreFrontInventoryStatistics {
    

    pub fn new_struct(mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Self {
        let mut this = Self {
            totalNumberOfItems: 0_i64,
            totalInventorySaleValueMoney: None,
        };
        this.totalInventorySaleValueMoney = Money::new();
        let mut inventoryEntityInterface: Option<Rc<RefCell<dyn InventoryEntityInterface>>> = jcall!(InventoryEntityFactory::getInstance()).getInventoryEntityInstance();
        let mut itemVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(inventoryEntityInterface).getItems(storeFrontInterface.clone());
        let mut size: i32 = jcall!(itemVector).size();
        for index in (0..size) {
            let mut itemInterface: Option<Rc<RefCell<dyn ItemInterface>>> = { let java_object = jcall!(itemVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ItemInterface>>>>().expect("ClassCastException").clone()) };
            let mut numberInStock: i32 = (((jcall!(itemInterface).getNumber()).to_string()).unwrap() as i32);
            this.totalNumberOfItems = this.totalNumberOfItems + ((numberInStock) as i64);
            let mut itemPriceMoney: Option<Rc<RefCell<Money>>> = jcall!(itemInterface).getPrice();
            jcall!(itemPriceMoney).multiply(numberInStock.clone());
            { let java_receiver = this.totalInventorySaleValueMoney.clone(); let java_arguments = (jcall!(itemPriceMoney).toString(),); jcall!(java_receiver).add(java_arguments.0); };
        }
        this
    }

    pub fn new(mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(storeFrontInterface))))
    }
    

    pub fn getNumber(&mut self) -> Option<i64> {
        return Some((self.totalNumberOfItems.clone()) as i64);
    }
    

    pub fn getTotal(&mut self) -> Option<Rc<RefCell<Money>>> {
        return self.totalInventorySaleValueMoney.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(jcall!(StoreFrontInventoryStatisticsData::StoreFrontInventoryStatisticsData::getInstance()).NUMBEROFITEMS.clone(), jcall!(self.getNumber()).toString());
        jcall!(hashMap).put(jcall!(StoreFrontInventoryStatisticsData::StoreFrontInventoryStatisticsData::getInstance()).TOTALVALUE.clone(), jcall!(self.getTotal()).toString());
        return hashMap.clone();
    }
    

    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return None;
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return None;
    }
}

