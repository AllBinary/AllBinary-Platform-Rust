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
use crate::org::allbinary::business::context::modules::storefront::statistics::advertisements::StoreFrontAdvertisementsStatisticsInterface::StoreFrontAdvertisementsStatisticsInterface;
use crate::org::allbinary::business::context::modules::storefront::statistics::advertisements::StoreFrontAdvertisementsStatisticsView::StoreFrontAdvertisementsStatisticsView;
use crate::org::allbinary::business::context::modules::storefront::statistics::inventory::StoreFrontInventoryStatisticsInterface::StoreFrontInventoryStatisticsInterface;
use crate::org::allbinary::business::context::modules::storefront::statistics::inventory::StoreFrontInventoryStatisticsView::StoreFrontInventoryStatisticsView;
use crate::org::allbinary::business::context::modules::storefront::statistics::orders::history::StoreFrontOrdersHistoryStatisticsInterface::StoreFrontOrdersHistoryStatisticsInterface;
use crate::org::allbinary::business::context::modules::storefront::statistics::orders::history::StoreFrontOrdersHistoryStatisticsView::StoreFrontOrdersHistoryStatisticsView;
use crate::org::allbinary::business::context::modules::storefront::statistics::users::StoreFrontUsersStatisticsInterface::StoreFrontUsersStatisticsInterface;
use crate::org::allbinary::business::context::modules::storefront::statistics::users::StoreFrontUsersStatisticsView::StoreFrontUsersStatisticsView;
use crate::org::allbinary::business::user::role::UserRole::UserRole;
use crate::org::allbinary::business::user::role::UserRoleFactory::UserRoleFactory;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::StoreFrontStatisticsInterface::StoreFrontStatisticsInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RealTimeStoreFrontStatisticsView {
    pub storeFrontStatisticsInterface: Option<Rc<RefCell<dyn StoreFrontStatisticsInterface>>>,
    pub userRole: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
}

impl DomNodeInterface for RealTimeStoreFrontStatisticsView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl RealTimeStoreFrontStatisticsView {
    

    pub fn new_struct(mut storeFrontStatisticsInterface: Option<Rc<RefCell<dyn StoreFrontStatisticsInterface>>>, mut userRole: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>) -> Self {
        let mut this = Self {
            storeFrontStatisticsInterface: None,
            userRole: None,
        };
        this.storeFrontStatisticsInterface = storeFrontStatisticsInterface.clone();
        this.userRole = userRole.clone();
        this
    }

    pub fn new(mut storeFrontStatisticsInterface: Option<Rc<RefCell<dyn StoreFrontStatisticsInterface>>>, mut userRole: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(storeFrontStatisticsInterface, userRole))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(jcall!(StoreFrontStatisticsData::StoreFrontStatisticsData::getInstance()).NAME.clone());
        let mut storeFrontAdvertisementsStatisticsInterface: Option<Rc<RefCell<dyn StoreFrontAdvertisementsStatisticsInterface>>> = jcall!(self.storeFrontStatisticsInterface).getAdvertisements();
        let mut storeFrontAdvertisementsStatisticsDomNodeInterface: Option<Rc<RefCell<dyn DomNodeInterface>>> = StoreFrontAdvertisementsStatisticsView::new(storeFrontAdvertisementsStatisticsInterface.clone()).map(|reference| reference as Rc<RefCell<dyn DomNodeInterface>>);
        jcall!(node).appendChild(jcall!(storeFrontAdvertisementsStatisticsDomNodeInterface).toXmlNode(document.clone()));
        let mut storeFrontInventoryStatisticsInterface: Option<Rc<RefCell<dyn StoreFrontInventoryStatisticsInterface>>> = jcall!(self.storeFrontStatisticsInterface).getInventory();
        let mut storeFrontInventoryStatisticsDomNodeInterface: Option<Rc<RefCell<dyn DomNodeInterface>>> = StoreFrontInventoryStatisticsView::new(storeFrontInventoryStatisticsInterface.clone()).map(|reference| reference as Rc<RefCell<dyn DomNodeInterface>>);
        jcall!(node).appendChild(jcall!(storeFrontInventoryStatisticsDomNodeInterface).toXmlNode(document.clone()));
        let mut storeFrontOrdersHistoryStatisticsInterface: Option<Rc<RefCell<dyn StoreFrontOrdersHistoryStatisticsInterface>>> = jcall!(self.storeFrontStatisticsInterface).getOrders();
        let mut storeFrontOrdersHistoryStatisticsDomNodeInterface: Option<Rc<RefCell<dyn DomNodeInterface>>> = StoreFrontOrdersHistoryStatisticsView::new(storeFrontOrdersHistoryStatisticsInterface.clone()).map(|reference| reference as Rc<RefCell<dyn DomNodeInterface>>);
        jcall!(node).appendChild(jcall!(storeFrontOrdersHistoryStatisticsDomNodeInterface).toXmlNode(document.clone()));
        if match (&self.userRole, &jcall!(UserRoleFactory::getInstance()).ADMINISTRATOR) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut storeFrontUsersStatisticsInterface: Option<Rc<RefCell<dyn StoreFrontUsersStatisticsInterface>>> = jcall!(self.storeFrontStatisticsInterface).getUsers();
            let mut storeFrontUsersStatisticsDomNodeInterface: Option<Rc<RefCell<dyn DomNodeInterface>>> = StoreFrontUsersStatisticsView::new(storeFrontUsersStatisticsInterface.clone()).map(|reference| reference as Rc<RefCell<dyn DomNodeInterface>>);
            jcall!(node).appendChild(jcall!(storeFrontUsersStatisticsDomNodeInterface).toXmlNode(document.clone()));
        }
        return node.clone();
    }
}

