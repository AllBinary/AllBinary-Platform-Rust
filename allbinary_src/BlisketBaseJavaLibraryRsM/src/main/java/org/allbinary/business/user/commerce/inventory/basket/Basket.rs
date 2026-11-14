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
use super::*;
// use crate::java::io::Serializable::Serializable;
use crate::java::util::HashMap::HashMap;
use crate::java::util::Set::Set;
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
    pub items: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>,
}

impl BasketInterface for Basket {
    fn addItem(&mut self, mut id: String, mut num: String) { Self::addItem(self, id, num) }
    fn removeItem(&mut self, mut id: String) { Self::removeItem(self, id) }
    fn adjustItem(&mut self, mut id: String, mut num: String) { Self::adjustItem(self, id, num) }
    fn getTotalWeight(&mut self) -> String { Self::getTotalWeight(self) }
    fn getNumberOfItems(&mut self) -> Option<i32> { Self::getNumberOfItems(self) }
    fn getIds(&mut self) -> Option<Rc<RefCell<dyn Set>>> { Self::getIds(self) }
    fn getItems(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> { Self::getItems(self) }
    fn getNumberOf(&mut self, mut product: String) -> Option<i32> { Self::getNumberOf(self, product) }
    fn getSubTotal(&mut self) -> Option<Rc<RefCell<Money>>> { Self::getSubTotal(self) }
}
// impl Serializable for Basket with the methods of the trait

impl Basket {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

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
        if jcall!(self.items).containsKey(crate::java_into_object(id.clone())) {
            let mut oldNumStr: String = { let java_object = (jcall!(self.items).get(crate::java_into_object(id.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }.clone();
            let mut oldNumInt: Option<i32> = Some((oldNumStr.clone()).trim().parse::<i32>().expect("NumberFormatException"));
            let mut numInt: Option<i32> = Some((num.clone()).trim().parse::<i32>().expect("NumberFormatException"));
            let mut newNumInt: Option<i32> = Some((((oldNumInt).unwrap() as i32) + ((numInt).unwrap() as i32)) as i32);
            { let java_receiver = self.items.clone(); let java_arguments = (crate::java_into_object(id.clone()), crate::java_into_object((newNumInt).expect("NullPointerException").to_string()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
        } else {
            jcall!(self.items).put(crate::java_into_object(id.clone()), crate::java_into_object(num.clone()));
        }
    }
    

    pub fn removeItem(&mut self, mut id: String) {
        jcall!(self.items).remove(crate::java_into_object(id.clone()));
    }
    

    pub fn adjustItem(&mut self, mut id: String, mut num: String) {
        if ((Some((num.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32) > 0 {
            jcall!(self.items).put(crate::java_into_object(id.clone()), crate::java_into_object(num.clone()));
        } else {
            jcall!(self.items).remove(crate::java_into_object(id.clone()));
        }
    }
    

    pub fn getTotalWeight(&mut self) -> String {
        let mut itemsAndNumberInBasket: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = self.getItems();
        let mut totalWeight: Option<Rc<RefCell<Money>>> = Money::new();
        if jcall!(itemsAndNumberInBasket).size() > 0 {
            let mut items: Option<Rc<RefCell<dyn Set>>> = jcall!(itemsAndNumberInBasket).keySet();
            let mut product: String = String::new();
            let mut weightFloat: Option<f32> = None;
            let mut itemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(items).toArray();
            let mut size: i32 = (itemArray.len() as i32).clone();
            for index in (0..size) {
                product = { let java_object = (itemArray[(index) as usize]).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }.clone();
                weightFloat = { let java_value = (Some((jcall!(InventoryEntity::new()).getWeight(product.clone())) as f32)).clone(); java_value };
                weightFloat = { let java_value = (Some(({ let java_value = ((self.getNumberOf(product.clone())).unwrap() as f32); java_value } * { let java_value = ((weightFloat).unwrap() as f32); java_value }) as f32)).clone(); java_value };
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
    

    pub fn getItems(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        return self.items.clone();
    }
    

    pub fn getNumberOf(&mut self, mut product: String) -> Option<i32> {
        let mut numStr: String = { let java_object = (jcall!(self.items).get(crate::java_into_object(product.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
        return Some((numStr.clone()).trim().parse::<i32>().expect("NumberFormatException"));
    }
    

    // throws MoneyException
    pub fn getSubTotal(&mut self) -> Option<Rc<RefCell<Money>>> {
        let mut itemsAndNumberInBasket: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = self.getItems();
        let mut subTotal: Option<Rc<RefCell<Money>>> = Money::new();
        if jcall!(itemsAndNumberInBasket).size() > 0 {
            let mut items: Option<Rc<RefCell<dyn Set>>> = jcall!(itemsAndNumberInBasket).keySet();
            let mut inventoryEntityFactory: Option<Rc<RefCell<InventoryEntityFactory>>> = InventoryEntityFactory::getInstance();
            let mut inventoryEntity: Option<Rc<RefCell<InventoryEntity>>> = (jcall!(inventoryEntityFactory).getInventoryEntityInstance()).clone();
            let mut itemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(items).toArray();
            let mut size: i32 = (itemArray.len() as i32).clone();
            for index in (0..size) {
                let mut product: String = { let java_object = (itemArray[(index) as usize]).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }.clone();
                let mut itemInterface: Option<Rc<RefCell<dyn ItemInterface>>> = jcall!(inventoryEntity).getItem(product.clone());
                let mut productTotal: Option<Rc<RefCell<Money>>> = jcall!(itemInterface).getPrice();
                jcall!(productTotal).multiply_int({ let java_argument = ((self.getNumberOf(product.clone())).unwrap() as i32); java_argument });
                jcall!(subTotal).add(jcall!(productTotal).toString());
            }
        }
        return subTotal.clone();
    }
}

