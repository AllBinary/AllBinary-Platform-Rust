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
// package org.allbinary.business.context.modules.storefront.statistics.orders.history
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryInterface::OrderHistoryInterface;
use crate::org::allbinary::business::user::commerce::money::Money::Money;
use crate::org::allbinary::data::tables::user::commerce::inventory::order::OrderHistoryEntityFactory::OrderHistoryEntityFactory;
use crate::org::allbinary::data::tables::user::commerce::inventory::order::OrderHistoryEntityInterface::OrderHistoryEntityInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use super::StoreFrontOrdersHistoryStatisticsInterface::StoreFrontOrdersHistoryStatisticsInterface;
use super::StoreFrontOrdersHistoryStatisticsData::StoreFrontOrdersHistoryStatisticsData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StoreFrontOrdersHistoryStatistics {
    pub numberOfOrdersLong: Option<i64>,
    pub subTotalMoney: Option<Rc<RefCell<Money>>>,
    pub shippingCostMoney: Option<Rc<RefCell<Money>>>,
    pub taxesMoney: Option<Rc<RefCell<Money>>>,
    pub totalMoney: Option<Rc<RefCell<Money>>>,
}

impl StoreFrontOrdersHistoryStatisticsInterface for StoreFrontOrdersHistoryStatistics {
    fn getNumberOfOrders(&mut self) -> Option<i64> { Self::getNumberOfOrders(self) }
    fn getSubTotal(&mut self) -> Option<Rc<RefCell<Money>>> { Self::getSubTotal(self) }
    fn getShippingCost(&mut self) -> Option<Rc<RefCell<Money>>> { Self::getShippingCost(self) }
    fn getTaxes(&mut self) -> Option<Rc<RefCell<Money>>> { Self::getTaxes(self) }
    fn getTotal(&mut self) -> Option<Rc<RefCell<Money>>> { Self::getTotal(self) }
}
impl crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface for StoreFrontOrdersHistoryStatistics {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toVector(self) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::toHashMap(self) }
}

impl StoreFrontOrdersHistoryStatistics {
    

    pub fn new_struct(mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Self {
        let mut this = Self {
            numberOfOrdersLong: None,
            subTotalMoney: None,
            shippingCostMoney: None,
            taxesMoney: None,
            totalMoney: None,
        };
        this.subTotalMoney = Money::new();
        this.shippingCostMoney = Money::new();
        this.taxesMoney = Money::new();
        this.totalMoney = Money::new();
        let mut orderHistoryEntityInterface: Option<Rc<RefCell<dyn OrderHistoryEntityInterface>>> = OrderHistoryEntityFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn OrderHistoryEntityInterface>>);
        let mut orderHistoryInterfaceVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(orderHistoryEntityInterface).getStoreOrders(storeFrontInterface.clone());
        let mut numberOfOrders: i64 = 0_i64;
        let mut orderHistoryInterface: Option<Rc<RefCell<dyn OrderHistoryInterface>>> = None;
        let mut size: i32 = jcall!(orderHistoryInterfaceVector).size();
        for index in (0..size) {
            orderHistoryInterface = { let java_object = jcall!(orderHistoryInterfaceVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn OrderHistoryInterface>>>>().expect("ClassCastException").clone()) };
            numberOfOrders = numberOfOrders + 1_i64;
            { let java_receiver = this.subTotalMoney.clone(); let java_arguments = (jcall!(orderHistoryInterface).getSubTotal(),); jcall!(java_receiver).add(java_arguments.0); };
            { let java_receiver = this.shippingCostMoney.clone(); let java_arguments = (jcall!(orderHistoryInterface).getShippingCost(),); jcall!(java_receiver).add(java_arguments.0); };
            { let java_receiver = this.taxesMoney.clone(); let java_arguments = (jcall!(orderHistoryInterface).getTaxes(),); jcall!(java_receiver).add(java_arguments.0); };
            { let java_receiver = this.totalMoney.clone(); let java_arguments = (jcall!(orderHistoryInterface).getTotal(),); jcall!(java_receiver).add(java_arguments.0); };
        }
        this.numberOfOrdersLong = Some((numberOfOrders.clone()) as i64);
        this
    }

    pub fn new(mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(storeFrontInterface))))
    }
    

    pub fn getNumberOfOrders(&mut self) -> Option<i64> {
        return self.numberOfOrdersLong.clone();
    }
    

    pub fn getSubTotal(&mut self) -> Option<Rc<RefCell<Money>>> {
        return self.subTotalMoney.clone();
    }
    

    pub fn getShippingCost(&mut self) -> Option<Rc<RefCell<Money>>> {
        return self.shippingCostMoney.clone();
    }
    

    pub fn getTaxes(&mut self) -> Option<Rc<RefCell<Money>>> {
        return self.taxesMoney.clone();
    }
    

    pub fn getTotal(&mut self) -> Option<Rc<RefCell<Money>>> {
        return self.totalMoney.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        let mut storeFrontOrdersHistoryStatisticsData: Option<Rc<RefCell<StoreFrontOrdersHistoryStatisticsData>>> = StoreFrontOrdersHistoryStatisticsData::getInstance();
        jcall!(hashMap).put(jcall!(storeFrontOrdersHistoryStatisticsData).NUMBEROFORDERS.clone(), jcall!(self.getNumberOfOrders()).toString());
        jcall!(hashMap).put(jcall!(storeFrontOrdersHistoryStatisticsData).SUBTOTAL.clone(), jcall!(self.getSubTotal()).toString());
        jcall!(hashMap).put(jcall!(storeFrontOrdersHistoryStatisticsData).SHIPPINGCOST.clone(), jcall!(self.getShippingCost()).toString());
        jcall!(hashMap).put(jcall!(storeFrontOrdersHistoryStatisticsData).TAXES.clone(), jcall!(self.getTaxes()).toString());
        jcall!(hashMap).put(jcall!(storeFrontOrdersHistoryStatisticsData).TOTAL.clone(), jcall!(self.getTotal()).toString());
        return hashMap.clone();
    }
    

    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return None;
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return None;
    }
}

