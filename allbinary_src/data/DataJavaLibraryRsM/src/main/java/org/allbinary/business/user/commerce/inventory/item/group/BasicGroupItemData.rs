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
// package org.allbinary.business.user.commerce.inventory.item.group
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

pub struct BasicGroupItemData {
}

thread_local! { pub static BasicGroupItemData_ITEM_ONE: String = String::from("BASICGROUPITEM_ONE"); }
thread_local! { pub static BasicGroupItemData_ITEM_TWO: String = String::from("BASICGROUPITEM_TWO"); }
thread_local! { pub static BasicGroupItemData_ITEM_THREE: String = String::from("BASICGROUPITEM_THREE"); }
thread_local! { pub static BasicGroupItemData_ITEM_FOUR: String = String::from("BASICGROUPITEM_FOUR"); }
thread_local! { pub static BasicGroupItemData_ITEM_FIVE: String = String::from("BASICGROUPITEM_FIVE"); }
thread_local! { pub static BasicGroupItemData_ITEM_SIX: String = String::from("BASICGROUPITEM_SIX"); }
thread_local! { pub static BasicGroupItemData_ITEM_SEVEN: String = String::from("BASICGROUPITEM_SEVEN"); }
thread_local! { pub static BasicGroupItemData_ITEM_EIGHT: String = String::from("BASICGROUPITEM_EIGHT"); }
thread_local! { pub static BasicGroupItemData_ITEM_NINE: String = String::from("BASICGROUPITEM_NINE"); }
thread_local! { pub static BasicGroupItemData_ITEM_TEN: String = String::from("BASICGROUPITEM_TEN"); }

impl BasicGroupItemData {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

