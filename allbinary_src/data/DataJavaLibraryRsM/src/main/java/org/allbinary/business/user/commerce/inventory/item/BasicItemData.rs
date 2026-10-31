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
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicItemData {
    pub ITEM: String,
    pub TOTAL: String,
    pub ID: String,
    pub NUMBER: String,
    pub INBASKETS: String,
    pub WEIGHT: String,
    pub NEWORUSED: String,
    pub SUMMARY: String,
    pub DISTRIBUTOR: String,
    pub IDUSEDBYDISTRIBUTOR: String,
    pub PRODUCEDBY: String,
    pub PRODUCTIONDATE: String,
    pub STARTPRODUCTIONDATE: String,
    pub DESCRIPTION: String,
    pub KEYWORDS: String,
    pub CATEGORY: String,
    pub TYPE: String,
    pub IMAGE: String,
    pub SMALLIMAGE: String,
    pub MEDIUMIMAGE: String,
    pub LARGEIMAGE: String,
    pub PRICE: String,
    pub COMMENT: String,
    pub CUSTOMS: String,
    pub DOWNLOADS: String,
    pub GROUPS: String,
    pub OPTIONS: String,
    pub PERMISSIONS: String,
    pub SPECIALS: String,
}

thread_local! { pub static BasicItemData_instance: Option<Rc<RefCell<BasicItemData>>> = BasicItemData::new(); }

impl BasicItemData {

    pub fn new_struct() -> Self {
        let mut this = Self {
            ITEM: String::new(),
            TOTAL: String::new(),
            ID: String::new(),
            NUMBER: String::new(),
            INBASKETS: String::new(),
            WEIGHT: String::new(),
            NEWORUSED: String::new(),
            SUMMARY: String::new(),
            DISTRIBUTOR: String::new(),
            IDUSEDBYDISTRIBUTOR: String::new(),
            PRODUCEDBY: String::new(),
            PRODUCTIONDATE: String::new(),
            STARTPRODUCTIONDATE: String::new(),
            DESCRIPTION: String::new(),
            KEYWORDS: String::new(),
            CATEGORY: String::new(),
            TYPE: String::new(),
            IMAGE: String::new(),
            SMALLIMAGE: String::new(),
            MEDIUMIMAGE: String::new(),
            LARGEIMAGE: String::new(),
            PRICE: String::new(),
            COMMENT: String::new(),
            CUSTOMS: String::new(),
            DOWNLOADS: String::new(),
            GROUPS: String::new(),
            OPTIONS: String::new(),
            PERMISSIONS: String::new(),
            SPECIALS: String::new(),
        };
        this.ITEM = String::from("BASICITEM");
        this.TOTAL = String::from("BASICITEM_TOTAL");
        this.ID = String::from("BASICITEM_ID");
        this.NUMBER = String::from("BASICITEM_NUMBER");
        this.INBASKETS = String::from("BASICITEM_IN_BASKETS");
        this.WEIGHT = String::from("BASICITEM_WEIGHT");
        this.NEWORUSED = String::from("BASICITEM_NEW_OR_USED");
        this.SUMMARY = String::from("BASICITEM_SUMMARY");
        this.DISTRIBUTOR = String::from("BASICITEM_DISTRIBUTOR");
        this.IDUSEDBYDISTRIBUTOR = String::from("BASICITEM_IDUSEDBYDISTRIBUTOR");
        this.PRODUCEDBY = String::from("BASICITEM_PRODUCEDBY");
        this.PRODUCTIONDATE = String::from("BASICITEM_PRODUCTIONDATE");
        this.STARTPRODUCTIONDATE = String::from("BASICITEM_STARTPRODUCTIONDATE");
        this.DESCRIPTION = String::from("BASICITEM_DESCRIPTION");
        this.KEYWORDS = String::from("BASICITEM_KEYWORDS");
        this.CATEGORY = String::from("BASICITEM_CATEGORY");
        this.TYPE = String::from("BASICITEM_TYPE");
        this.IMAGE = String::from("BASICITEM_IMG");
        this.SMALLIMAGE = String::from("BASICITEM_SMALL_IMG");
        this.MEDIUMIMAGE = String::from("BASICITEM_MEDIUM_IMG");
        this.LARGEIMAGE = String::from("BASICITEM_LARGE_IMG");
        this.PRICE = String::from("BASICITEM_PRICE");
        this.COMMENT = String::from("BASICITEM_COMMENT");
        this.CUSTOMS = String::from("BASICITEM_CUSTOMS");
        this.DOWNLOADS = String::from("BASICITEM_DOWNLOADS");
        this.GROUPS = String::from("BASICITEM_GROUPS");
        this.OPTIONS = String::from("BASICITEM_OPTIONS");
        this.PERMISSIONS = String::from("BASICITEM_PERMISSIONS");
        this.SPECIALS = String::from("BASICITEM_SPECIALS");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<BasicItemData>>> {
        return BasicItemData_instance.with(Clone::clone).clone();
    }
}

