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

thread_local! { pub static BasicOptionItemData_OPTION_ONE_TITLE: String = String::from("BASICOPTIONITEM_OPTION_ONE_TITLE"); }
thread_local! { pub static BasicOptionItemData_DEFAULT_OPTION_ITEM: String = String::from("BASICOPTIONITEM_DEFAULT_OPTION_ITEM"); }
thread_local! { pub static BasicOptionItemData_DEFAULT_OPTION_VALUE: String = String::from("BASICOPTIONITEM_DEFAULT_OPTION_VALUE"); }
thread_local! { pub static BasicOptionItemData_OPTION_ONE_ONE_ITEM: String = String::from("BASICOPTIONITEM_OPTION_ONE_ONE_ITEM"); }
thread_local! { pub static BasicOptionItemData_OPTION_ONE_ONE_VALUE: String = String::from("BASICOPTIONITEM_OPTION_ONE_ONE_VALUE"); }
thread_local! { pub static BasicOptionItemData_OPTION_ONE_TWO_ITEM: String = String::from("BASICOPTIONITEM_OPTION_ONE_TWO_ITEM"); }
thread_local! { pub static BasicOptionItemData_OPTION_ONE_TWO_VALUE: String = String::from("BASICOPTIONITEM_OPTION_ONE_TWO_VALUE"); }
thread_local! { pub static BasicOptionItemData_OPTION_ONE_THREE_ITEM: String = String::from("BASICOPTIONITEM_OPTION_ONE_THREE_ITEM"); }
thread_local! { pub static BasicOptionItemData_OPTION_ONE_THREE_VALUE: String = String::from("BASICOPTIONITEM_OPTION_ONE_THREE_VALUE"); }
thread_local! { pub static BasicOptionItemData_OPTION_ONE_FOUR_ITEM: String = String::from("BASICOPTIONITEM_OPTION_ONE_FOUR_ITEM"); }
thread_local! { pub static BasicOptionItemData_OPTION_ONE_FOUR_VALUE: String = String::from("BASICOPTIONITEM_OPTION_ONE_FOUR_VALUE"); }
thread_local! { pub static BasicOptionItemData_OPTION_ONE_FIVE_ITEM: String = String::from("BASICOPTIONITEM_OPTION_ONE_FIVE_ITEM"); }
thread_local! { pub static BasicOptionItemData_OPTION_ONE_FIVE_VALUE: String = String::from("BASICOPTIONITEM_OPTION_ONE_FIVE_VALUE"); }
thread_local! { pub static BasicOptionItemData_OPTION_ONE_SIX_ITEM: String = String::from("BASICOPTIONITEM_OPTION_ONE_SIX_ITEM"); }
thread_local! { pub static BasicOptionItemData_OPTION_ONE_SIX_VALUE: String = String::from("BASICOPTIONITEM_OPTION_ONE_SIX_VALUE"); }
thread_local! { pub static BasicOptionItemData_OPTION_ONE_SEVEN_ITEM: String = String::from("BASICOPTIONITEM_OPTION_ONE_SEVEN_ITEM"); }
thread_local! { pub static BasicOptionItemData_OPTION_ONE_SEVEN_VALUE: String = String::from("BASICOPTIONITEM_OPTION_ONE_SEVEN_VALUE"); }
thread_local! { pub static BasicOptionItemData_OPTION_ONE_EIGHT_ITEM: String = String::from("BASICOPTIONITEM_OPTION_ONE_EIGHT_ITEM"); }
thread_local! { pub static BasicOptionItemData_OPTION_ONE_EIGHT_VALUE: String = String::from("BASICOPTIONITEM_OPTION_ONE_EIGHT_VALUE"); }
thread_local! { pub static BasicOptionItemData_OPTION_ONE_NINE_ITEM: String = String::from("BASICOPTIONITEM_OPTION_ONE_NINE_ITEM"); }
thread_local! { pub static BasicOptionItemData_OPTION_ONE_NINE_VALUE: String = String::from("BASICOPTIONITEM_OPTION_ONE_NINE_VALUE"); }

impl BasicOptionItemData {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

