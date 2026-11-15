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

pub static BasicGroupItemData_ITEM_ONE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICGROUPITEM_ONE"));
pub static BasicGroupItemData_ITEM_TWO: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICGROUPITEM_TWO"));
pub static BasicGroupItemData_ITEM_THREE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICGROUPITEM_THREE"));
pub static BasicGroupItemData_ITEM_FOUR: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICGROUPITEM_FOUR"));
pub static BasicGroupItemData_ITEM_FIVE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICGROUPITEM_FIVE"));
pub static BasicGroupItemData_ITEM_SIX: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICGROUPITEM_SIX"));
pub static BasicGroupItemData_ITEM_SEVEN: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICGROUPITEM_SEVEN"));
pub static BasicGroupItemData_ITEM_EIGHT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICGROUPITEM_EIGHT"));
pub static BasicGroupItemData_ITEM_NINE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICGROUPITEM_NINE"));
pub static BasicGroupItemData_ITEM_TEN: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICGROUPITEM_TEN"));

impl BasicGroupItemData {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

