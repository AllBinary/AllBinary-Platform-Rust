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
// package org.allbinary.business.user.commerce.inventory.order
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::generator::OrderIdGenerator::OrderIdGenerator;
use crate::org::allbinary::data::tables::user::commerce::inventory::order::OrderHistoryEntity::OrderHistoryEntity;
use crate::org::allbinary::data::tables::user::commerce::inventory::order::OrderItemsEntity::OrderItemsEntity;
use super::Order::Order;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OrderProcessorUtil {
    pub ORDER_PROCESSED: String,
}

thread_local! { pub static OrderProcessorUtil_instance: Option<Rc<RefCell<OrderProcessorUtil>>> = OrderProcessorUtil::new(); }

impl OrderProcessorUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            ORDER_PROCESSED: String::new(),
        };
        this.ORDER_PROCESSED = String::from("Order Processed");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<OrderProcessorUtil>>> {
        return OrderProcessorUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn process(&mut self, mut userName: String, mut order: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn>>>) -> String {
        let mut orderItems: Option<Rc<RefCell<OrderItemsEntity>>> = OrderItemsEntity::new();
        let mut orderHistory: Option<Rc<RefCell<OrderHistoryEntity>>> = OrderHistoryEntity::new();
        jcall!(order).setId(jcall!(OrderIdGenerator::new()).getNext());
        jcall!(orderItems).insert(userName.clone(), order.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::order::OrderInterface::OrderInterface>>));
        jcall!(orderHistory).insert2(userName.clone(), order.clone());
        return self.ORDER_PROCESSED.clone();
    }
}

