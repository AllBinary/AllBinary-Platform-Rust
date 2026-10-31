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
// package org.allbinary.business.context.modules.storefront.statistics
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::context::modules::storefront::statistics::advertisements::StoreFrontAdvertisementsStatistics::StoreFrontAdvertisementsStatistics;
use crate::org::allbinary::business::context::modules::storefront::statistics::advertisements::StoreFrontAdvertisementsStatisticsInterface::StoreFrontAdvertisementsStatisticsInterface;
use crate::org::allbinary::business::context::modules::storefront::statistics::inventory::StoreFrontInventoryStatistics::StoreFrontInventoryStatistics;
use crate::org::allbinary::business::context::modules::storefront::statistics::inventory::StoreFrontInventoryStatisticsInterface::StoreFrontInventoryStatisticsInterface;
use crate::org::allbinary::business::context::modules::storefront::statistics::orders::history::StoreFrontOrdersHistoryStatistics::StoreFrontOrdersHistoryStatistics;
use crate::org::allbinary::business::context::modules::storefront::statistics::orders::history::StoreFrontOrdersHistoryStatisticsInterface::StoreFrontOrdersHistoryStatisticsInterface;
use crate::org::allbinary::business::context::modules::storefront::statistics::users::StoreFrontUsersStatistics::StoreFrontUsersStatistics;
use crate::org::allbinary::business::context::modules::storefront::statistics::users::StoreFrontUsersStatisticsInterface::StoreFrontUsersStatisticsInterface;
use super::StoreFrontStatisticsInterface::StoreFrontStatisticsInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RealTimeStoreFrontStatistics {
    pub storeFrontAdvertisementsStatisticsInterface: Option<Rc<RefCell<dyn StoreFrontAdvertisementsStatisticsInterface>>>,
    pub storeFrontUsersStatisticsInterface: Option<Rc<RefCell<dyn StoreFrontUsersStatisticsInterface>>>,
    pub storeFrontInventoryStatisticsInterface: Option<Rc<RefCell<dyn StoreFrontInventoryStatisticsInterface>>>,
    pub storeFrontOrderHistoryStatisticsInterface: Option<Rc<RefCell<dyn StoreFrontOrdersHistoryStatisticsInterface>>>,
}

impl StoreFrontStatisticsInterface for RealTimeStoreFrontStatistics {
    fn getAdvertisements(&mut self) -> Option<Rc<RefCell<dyn StoreFrontAdvertisementsStatisticsInterface>>> { Self::getAdvertisements(self) }
    fn getUsers(&mut self) -> Option<Rc<RefCell<dyn StoreFrontUsersStatisticsInterface>>> { Self::getUsers(self) }
    fn getInventory(&mut self) -> Option<Rc<RefCell<dyn StoreFrontInventoryStatisticsInterface>>> { Self::getInventory(self) }
    fn getOrders(&mut self) -> Option<Rc<RefCell<dyn StoreFrontOrdersHistoryStatisticsInterface>>> { Self::getOrders(self) }
}

impl RealTimeStoreFrontStatistics {
    

    pub fn new_struct(mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Self {
        let mut this = Self {
            storeFrontAdvertisementsStatisticsInterface: None,
            storeFrontUsersStatisticsInterface: None,
            storeFrontInventoryStatisticsInterface: None,
            storeFrontOrderHistoryStatisticsInterface: None,
        };
        this.storeFrontAdvertisementsStatisticsInterface = (StoreFrontAdvertisementsStatistics::new(storeFrontInterface.clone())).clone() /* as StoreFrontAdvertisementsStatisticsInterface */;
        this.storeFrontUsersStatisticsInterface = (StoreFrontUsersStatistics::new(storeFrontInterface.clone())).clone() /* as StoreFrontUsersStatisticsInterface */;
        this.storeFrontInventoryStatisticsInterface = (StoreFrontInventoryStatistics::new(storeFrontInterface.clone())).clone() /* as StoreFrontInventoryStatisticsInterface */;
        this.storeFrontOrderHistoryStatisticsInterface = (StoreFrontOrdersHistoryStatistics::new(storeFrontInterface.clone())).clone() /* as StoreFrontOrdersHistoryStatisticsInterface */;
        this
    }

    pub fn new(mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(storeFrontInterface))))
    }
    

    pub fn getAdvertisements(&mut self) -> Option<Rc<RefCell<dyn StoreFrontAdvertisementsStatisticsInterface>>> {
        return self.storeFrontAdvertisementsStatisticsInterface.clone();
    }
    

    pub fn getUsers(&mut self) -> Option<Rc<RefCell<dyn StoreFrontUsersStatisticsInterface>>> {
        return self.storeFrontUsersStatisticsInterface.clone();
    }
    

    pub fn getInventory(&mut self) -> Option<Rc<RefCell<dyn StoreFrontInventoryStatisticsInterface>>> {
        return self.storeFrontInventoryStatisticsInterface.clone();
    }
    

    pub fn getOrders(&mut self) -> Option<Rc<RefCell<dyn StoreFrontOrdersHistoryStatisticsInterface>>> {
        return self.storeFrontOrderHistoryStatisticsInterface.clone();
    }
}

