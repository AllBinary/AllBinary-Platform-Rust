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
// package views.generic.inventory
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemData::BasicItemData;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntity::InventoryEntity;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InventoryColumnUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub basicItemData: Option<Rc<RefCell<BasicItemData>>>,
}

thread_local! { pub static InventoryColumnUtil_instance: Option<Rc<RefCell<InventoryColumnUtil>>> = InventoryColumnUtil::new(); }

impl InventoryColumnUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            basicItemData: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.basicItemData = BasicItemData::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<InventoryColumnUtil>>> {
        return InventoryColumnUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn getColumnWhereLike(&mut self, mut inventoryEntity: Option<Rc<RefCell<InventoryEntity>>>, mut category: String, mut column: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        jcall!(self.logUtil).putF(format!("{}{}", String::from("Start Category: "), category), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getKeywords"));
        let mut keywords: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut vectorOfHashMaps: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(inventoryEntity).getAllRows();
        let mut size: i32 = jcall!(vectorOfHashMaps).size();
        for index in (0..size) {
            let mut hashMap: Option<Rc<RefCell<HashMap>>> = (jcall!(vectorOfHashMaps).get(index.clone())).clone() /* as HashMap */;
            let mut categoryName: String = (jcall!(hashMap).get(jcall!(self.basicItemData).CATEGORY.clone())).clone() /* as String */;
            if !jcall!(StringValidationUtil::getInstance()).isEmpty(categoryName.clone()) && categoryName.starts_with(&category.clone()) {
                jcall!(keywords).add(crate::java_into_object(jcall!(hashMap).get(column.clone())));
            }
        }
        return keywords.clone();
    }
}

