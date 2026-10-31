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
// use crate::java::util::HashSet::HashSet;
// use crate::java::util::Set::Set;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::user::commerce::inventory::item::Item::Item;
use crate::org::allbinary::business::user::commerce::inventory::item::ItemView::ItemView;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasketReview {
    pub items: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl BasketReview {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            items: None,
        };
        this.items = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        return true;
    }
    

    pub fn addItem(&mut self, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::item::Item::ItemDyn>>>) {
        jcall!(self.items).add(crate::java_into_object(item.clone()));
    }
    

    pub fn getItems(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.items.clone();
    }
    

    pub fn removeItem(&mut self, mut id: String) {
        let mut itemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(self.items).toArray();
        let mut size: i32 = (itemArray.len() as i32).clone();
        for index in (0..size) {
            let mut item: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::item::Item::ItemDyn>>> = (itemArray[(index) as usize]).clone() /* as Item */;
            if jcall!(jcall!(item).as_Item_mut().getId()).compareTo(id.clone()) == 0 {
                jcall!(self.items).remove(crate::java_into_object(item.clone()));
            }
        }
    }
    

    pub fn adjustItem(&mut self, mut id: String, mut num: String) {
        let mut itemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(self.items).toArray();
        let mut size: i32 = (itemArray.len() as i32).clone();
        for index in (0..size) {
            let mut item: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::item::Item::ItemDyn>>> = (itemArray[(index) as usize]).clone() /* as Item */;
            if jcall!(jcall!(item).as_Item_mut().getId()).compareTo(id.clone()) == 0 {
                jcall!(item).as_Item_mut().setNumber(num.clone());
            }
        }
    }
    

    pub fn getTotalWeight(&mut self) -> String {
        let mut weightFloat: f32 = 0.0_f32;
        let mut itemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(self.items).toArray();
        let mut size: i32 = (itemArray.len() as i32).clone();
        for index in (0..size) {
            let mut item: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::item::Item::ItemDyn>>> = (itemArray[(index) as usize]).clone() /* as Item */;
            weightFloat += ((Some((jcall!(item).as_Item_mut().getWeight()) as f32)).unwrap() as f32);
        }
        return jcall!(Some((weightFloat.clone()) as f32)).toString();
    }
    

    pub fn getNumberOfItems(&mut self) -> Option<i32> {
        return Some((jcall!(self.items).size()) as i32);
    }
    

    pub fn getIds(&mut self) -> Option<Rc<RefCell<dyn Set>>> {
        let mut idSet: Option<Rc<RefCell<HashSet>>> = HashSet::new();
        let mut itemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(self.items).toArray();
        let mut size: i32 = (itemArray.len() as i32).clone();
        for index in (0..size) {
            let mut item: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::item::Item::ItemDyn>>> = (itemArray[(index) as usize]).clone() /* as Item */;
            jcall!(idSet).add(jcall!(item).as_Item_mut().getId());
        }
        return (idSet).clone() /* as Set */;
    }
    

    pub fn getNumberOf(&mut self, mut id: String) -> Option<i32> {
        let mut itemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(self.items).toArray();
        let mut size: i32 = (itemArray.len() as i32).clone();
        for index in (0..size) {
            let mut item: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::item::Item::ItemDyn>>> = (itemArray[(index) as usize]).clone() /* as Item */;
            if jcall!(jcall!(item).as_Item_mut().getId()).compareTo(id.clone()) == 0 {
                return Some((jcall!(item).as_Item_mut().getNumber()) as i32);
            }
        }
        return Some((0) as i32);
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(BasketData_BASKET.with(Clone::clone).clone());
        let mut itemArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(self.items).toArray();
        let mut size: i32 = (itemArray.len() as i32).clone();
        for index in (0..size) {
            let mut item: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::item::Item::ItemDyn>>> = (itemArray[(index) as usize]).clone() /* as Item */;
            jcall!(node).appendChild(jcall!(ItemView::new(item.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>))).toXmlNode(document.clone()));
        }
        return node.clone();
    }
}

