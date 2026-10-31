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
use crate::org::allbinary::logic::control::validate::ValidatorBase::ValidatorBase;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TextFieldItemValidator {
    pub base: ValidatorBase,
    pub textFieldItem: Option<Rc<RefCell<ABTextFieldItem>>>,
    pub min: i32,
    pub max: i32,
    pub allowOnEmpty: bool,
}


impl TextFieldItemValidator {
    

    pub fn new_struct(mut textFieldItem: Option<Rc<RefCell<ABTextFieldItem>>>, mut min: i32, mut max: i32, mut allowOnEmpty: bool) -> Self {
        let mut this = Self {
            base: ValidatorBase::new_struct(),
            textFieldItem: None,
            min: 0,
            max: 0,
            allowOnEmpty: false,
        };
        this.textFieldItem = textFieldItem.clone();
        this.min = min.clone();
        this.max = max.clone();
        this.allowOnEmpty = allowOnEmpty.clone();
        this
    }

    pub fn new(mut textFieldItem: Option<Rc<RefCell<ABTextFieldItem>>>, mut min: i32, mut max: i32, mut allowOnEmpty: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(textFieldItem, min, max, allowOnEmpty))))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        let mut booleanFactory: Option<Rc<RefCell<BooleanFactory>>> = BooleanFactory::getInstance();
        let mut result: Option<bool> = jcall!(booleanFactory).TRUE.clone();
        let mut string: String = jcall!(self.textFieldItem).getString();
        let mut textLength: i32 = (string.chars().count() as i32);
        if (textLength == 0 && self.allowOnEmpty) || textLength > self.min && textLength < self.max {
        } else {
            if textLength < self.min {
                result = jcall!(booleanFactory).FALSE.clone();
            } else if textLength > self.max {
                result = jcall!(booleanFactory).FALSE.clone();
            }
        }
        return result.clone();
    }
    

    pub fn toList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut string: String = jcall!(self.textFieldItem).getString();
        let mut textLength: i32 = (string.chars().count() as i32);
        if textLength > self.min && textLength < self.max {
        } else {
            let mut label: String = jcall!(self.textFieldItem).getLabel();
            let mut name: String = label[(0) as usize..((label.chars().count() as i32) - 2) as usize].to_string();
            if textLength < self.min {
                jcall!(list).add(crate::java_into_object(jcall!(StringMaker::new()).append(name.clone()).append(String::from(" is to short")).toString()));
            } else if textLength > self.max {
                jcall!(list).add(crate::java_into_object(jcall!(StringMaker::new()).append(name.clone()).append(String::from(" is to long")).toString()));
            }
        }
        return list.clone();
    }
}
impl std::ops::Deref for TextFieldItemValidator {
    type Target = ValidatorBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TextFieldItemValidator {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait TextFieldItemValidatorDyn: crate::org::allbinary::logic::control::validate::ValidatorBase::ValidatorBaseDyn {
    fn as_TextFieldItemValidator(&self) -> &TextFieldItemValidator;
    fn as_TextFieldItemValidator_mut(&mut self) -> &mut TextFieldItemValidator;
}
impl TextFieldItemValidatorDyn for TextFieldItemValidator {
    fn as_TextFieldItemValidator(&self) -> &TextFieldItemValidator { self }
    fn as_TextFieldItemValidator_mut(&mut self) -> &mut TextFieldItemValidator { self }
}

impl crate::org::allbinary::logic::control::validate::ValidatorBase::ValidatorBaseDyn for TextFieldItemValidator {
    fn as_ValidatorBase(&self) -> &crate::org::allbinary::logic::control::validate::ValidatorBase::ValidatorBase { &self.base }
    fn as_ValidatorBase_mut(&mut self) -> &mut crate::org::allbinary::logic::control::validate::ValidatorBase::ValidatorBase { &mut self.base }
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toList(self) }
}


