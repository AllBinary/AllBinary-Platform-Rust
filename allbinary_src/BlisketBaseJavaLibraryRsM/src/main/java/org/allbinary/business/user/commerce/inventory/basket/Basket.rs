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
// package org.allbinary.business.user.commerce.inventory.basket
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::io::Serializable::Serializable;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Set::Set;
use crate::org::allbinary::business::user::commerce::inventory::item::ItemInterface::ItemInterface;
use crate::org::allbinary::business::user::commerce::money::Money::Money;
use crate::org::allbinary::business::user::commerce::money::MoneyException::MoneyException;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntity::InventoryEntity;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntityFactory::InventoryEntityFactory;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use super::BasketInterface::BasketInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Basket {
    pub items: Option<Rc<RefCell<HashMap>>>,
}

impl BasketInterface for Basket {
    fn addItem(&mut self, mut id: String, mut num: String) { Self::addItem(self, id, num) }
    fn removeItem(&mut self, mut id: String) { Self::removeItem(self, id) }
    fn adjustItem(&mut self, mut id: String, mut num: String) { Self::adjustItem(self, id, num) }
    fn getTotalWeight(&mut self) -> String { Self::getTotalWeight(self) }
    fn getNumberOfItems(&mut self) -> Option<i32> { Self::getNumberOfItems(self) }
    fn getIds(&mut self) -> Option<Rc<RefCell<dyn Set>>> { Self::getIds(self) }
    fn getItems(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::getItems(self) }
    fn getNumberOf(&mut self, mut product: String) -> Option<i32> { Self::getNumberOf(self, product) }
    fn getSubTotal(&mut self) -> Option<Rc<RefCell<Money>>> { Self::getSubTotal(self) }
}
// impl Serializable for Basket with the methods of the trait

impl Basket {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            items: None,
        };
        this.items = jcall!(StdUtil::getInstance()).createHashMap();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn addItem(&mut self, mut id: String, mut num: String) {
        if jcall!(self.items).containsKey(id.clone()) {
            let mut oldNumStr: String = (jcall!(self.items).get(id.clone())).clone() /* as String */.clone();
            let mut oldNumInt: Option<i32> = (oldNumStr.clone()).to_string();
            let mut numInt: Option<i32> = (num.clone()).to_string();
            let mut newNumInt: Option<i32> = Some((((oldNumInt).unwrap() as i32) + ((numInt).unwrap() as i32)) as i32);
            { let java_receiver = self.items.clone(); let java_arguments = (id.clone(), (newNumInt).expect("NullPointerException").to_string(),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
        } else {
            jcall!(self.items).put(id.clone(), num.clone());
        }
    }
    

    pub fn removeItem(&mut self, mut id: String) {
        jcall!(self.items).remove(id.clone());
    }
    

    pub fn adjustItem(&mut self, mut id: String, mut num: String) {
        if (((num.clone()).to_string()).unwrap() as i32) > 0 {
            jcall!(self.items).put(id.clone(), num.clone());
        } else {
            jcall!(self.items).remove(id.clone());
        }
    }
    

    pub fn getTotalWeight(&mut self) -> String {
        let mut itemsAndNumberInBasket: Option<Rc<RefCell<HashMap>>> = self.getItems();
        let mut totalWeight: Option<Rc<RefCell<Money>>> = Money::new();
        if jcall!(itemsAndNumberInBasket).size() > 0 {
            let mut items: Option<Rc<RefCell<dyn Set>>> = jcall!(itemsAndNumberInBasket).keySet();
            let mut product: String = String::new();
            let mut weightFloat: Option<f32> = None;
            let mut itemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(items).toArray();
            let mut size: i32 = (itemArray.len() as i32).clone();
            for index in (0..size) {
                product = (itemArray[(index) as usize]).clone() /* as String */.clone();
                weightFloat = Some((jcall!(InventoryEntity::new()).getWeight(product.clone())) as f32);
                weightFloat = Some((((self.getNumberOf(product.clone())).unwrap() as f32) * ((weightFloat).unwrap() as f32)) as f32);
                jcall!(totalWeight).add((weightFloat).expect("NullPointerException").to_string());
            }
        }
        return jcall!(totalWeight).toString();
    }
    

    pub fn getNumberOfItems(&mut self) -> Option<i32> {
        return Some((jcall!(self.items).size()) as i32);
    }
    

    pub fn getIds(&mut self) -> Option<Rc<RefCell<dyn Set>>> {
        let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(self.items).keySet();
        return set.clone();
    }
    

    pub fn getItems(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        return self.items.clone();
    }
    

    pub fn getNumberOf(&mut self, mut product: String) -> Option<i32> {
        let mut numStr: String = (jcall!(self.items).get(product.clone())).clone() /* as String */;
        return (numStr.clone()).to_string();
    }
    

    // throws MoneyException
    pub fn getSubTotal(&mut self) -> Option<Rc<RefCell<Money>>> {
        let mut itemsAndNumberInBasket: Option<Rc<RefCell<HashMap>>> = self.getItems();
        let mut subTotal: Option<Rc<RefCell<Money>>> = Money::new();
        if jcall!(itemsAndNumberInBasket).size() > 0 {
            let mut items: Option<Rc<RefCell<dyn Set>>> = jcall!(itemsAndNumberInBasket).keySet();
            let mut inventoryEntityFactory: Option<Rc<RefCell<InventoryEntityFactory>>> = InventoryEntityFactory::getInstance();
            let mut inventoryEntity: Option<Rc<RefCell<InventoryEntity>>> = (jcall!(inventoryEntityFactory).getInventoryEntityInstance()).clone() /* as InventoryEntity */;
            let mut itemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(items).toArray();
            let mut size: i32 = (itemArray.len() as i32).clone();
            for index in (0..size) {
                let mut product: String = (itemArray[(index) as usize]).clone() /* as String */.clone();
                let mut itemInterface: Option<Rc<RefCell<dyn ItemInterface>>> = jcall!(inventoryEntity).getItem(product.clone());
                let mut productTotal: Option<Rc<RefCell<Money>>> = jcall!(itemInterface).getPrice();
                jcall!(productTotal).multiply(((self.getNumberOf(product.clone())).unwrap() as i32));
                jcall!(subTotal).add(jcall!(productTotal).toString());
            }
        }
        return subTotal.clone();
    }
}

