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
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::business::user::commerce::money::MoneyException::MoneyException;
use crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface;
use crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParams;
use super::Item::Item;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ItemFactoryUnused {
}


impl ItemFactoryUnused {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws MoneyException
    pub fn getInstance(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<dyn TableMappingInterface>>> {
        return (Item::new1(hashMap.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::item::Item::ItemDyn>>)).clone() /* as TableMappingInterface */;
    }
    

    // throws Exception, MoneyException
    // Java overloaded method: Rust needs a different name
    pub fn getInstance(mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<dyn TableMappingInterface>>> {
        return (Item::new1(jcall!(RequestParams::new1(request.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParamsDyn>>)).toHashMap()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::item::Item::ItemDyn>>)).clone() /* as TableMappingInterface */;
    }
}

