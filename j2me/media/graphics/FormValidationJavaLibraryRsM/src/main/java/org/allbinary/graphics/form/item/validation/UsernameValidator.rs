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
// package org.allbinary.graphics.form.item.validation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::form::item::ABTextFieldItem::ABTextFieldItem;
use super::TextFieldItemValidator::TextFieldItemValidator;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UsernameValidator {
    pub base: TextFieldItemValidator,
}


impl UsernameValidator {
    

    pub fn new_struct(mut textFieldItem: Option<Rc<RefCell<ABTextFieldItem>>>, mut min: i32, mut max: i32, mut allowOnEmpty: bool) -> Self {
        let mut this = Self {
            base: TextFieldItemValidator::new_struct(textFieldItem.clone(), min.clone(), max.clone(), allowOnEmpty.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut textFieldItem: Option<Rc<RefCell<ABTextFieldItem>>>, mut min: i32, mut max: i32, mut allowOnEmpty: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(textFieldItem, min, max, allowOnEmpty))))
    }
}
impl std::ops::Deref for UsernameValidator {
    type Target = TextFieldItemValidator;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UsernameValidator {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::item::validation::TextFieldItemValidator::TextFieldItemValidatorDyn for UsernameValidator {
    fn as_TextFieldItemValidator(&self) -> &crate::org::allbinary::graphics::form::item::validation::TextFieldItemValidator::TextFieldItemValidator { &self.base }
    fn as_TextFieldItemValidator_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::item::validation::TextFieldItemValidator::TextFieldItemValidator { &mut self.base }
}
impl crate::org::allbinary::logic::control::validate::ValidatorBase::ValidatorBaseDyn for UsernameValidator {
    fn as_ValidatorBase(&self) -> &crate::org::allbinary::logic::control::validate::ValidatorBase::ValidatorBase { &self.base.base }
    fn as_ValidatorBase_mut(&mut self) -> &mut crate::org::allbinary::logic::control::validate::ValidatorBase::ValidatorBase { &mut self.base.base }
    fn toList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { crate::org::allbinary::graphics::form::item::validation::TextFieldItemValidator::TextFieldItemValidator::toList(crate::org::allbinary::graphics::form::item::validation::TextFieldItemValidator::TextFieldItemValidatorDyn::as_TextFieldItemValidator_mut(self)) }
}


