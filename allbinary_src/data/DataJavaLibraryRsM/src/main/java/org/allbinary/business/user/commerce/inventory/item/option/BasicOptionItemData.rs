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
// package org.allbinary.business.user.commerce.inventory.item.option
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

pub struct BasicOptionItemData {
}

pub static BasicOptionItemData_OPTION_ONE_TITLE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_TITLE"));
pub static BasicOptionItemData_DEFAULT_OPTION_ITEM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_DEFAULT_OPTION_ITEM"));
pub static BasicOptionItemData_DEFAULT_OPTION_VALUE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_DEFAULT_OPTION_VALUE"));
pub static BasicOptionItemData_OPTION_ONE_ONE_ITEM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_ONE_ITEM"));
pub static BasicOptionItemData_OPTION_ONE_ONE_VALUE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_ONE_VALUE"));
pub static BasicOptionItemData_OPTION_ONE_TWO_ITEM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_TWO_ITEM"));
pub static BasicOptionItemData_OPTION_ONE_TWO_VALUE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_TWO_VALUE"));
pub static BasicOptionItemData_OPTION_ONE_THREE_ITEM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_THREE_ITEM"));
pub static BasicOptionItemData_OPTION_ONE_THREE_VALUE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_THREE_VALUE"));
pub static BasicOptionItemData_OPTION_ONE_FOUR_ITEM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_FOUR_ITEM"));
pub static BasicOptionItemData_OPTION_ONE_FOUR_VALUE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_FOUR_VALUE"));
pub static BasicOptionItemData_OPTION_ONE_FIVE_ITEM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_FIVE_ITEM"));
pub static BasicOptionItemData_OPTION_ONE_FIVE_VALUE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_FIVE_VALUE"));
pub static BasicOptionItemData_OPTION_ONE_SIX_ITEM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_SIX_ITEM"));
pub static BasicOptionItemData_OPTION_ONE_SIX_VALUE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_SIX_VALUE"));
pub static BasicOptionItemData_OPTION_ONE_SEVEN_ITEM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_SEVEN_ITEM"));
pub static BasicOptionItemData_OPTION_ONE_SEVEN_VALUE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_SEVEN_VALUE"));
pub static BasicOptionItemData_OPTION_ONE_EIGHT_ITEM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_EIGHT_ITEM"));
pub static BasicOptionItemData_OPTION_ONE_EIGHT_VALUE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_EIGHT_VALUE"));
pub static BasicOptionItemData_OPTION_ONE_NINE_ITEM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_NINE_ITEM"));
pub static BasicOptionItemData_OPTION_ONE_NINE_VALUE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASICOPTIONITEM_OPTION_ONE_NINE_VALUE"));

impl BasicOptionItemData {

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

