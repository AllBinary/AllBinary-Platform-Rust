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
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NumberTextFieldItemValidator {
    pub base: ValidatorBase,
    pub textFieldItem: Option<Rc<RefCell<ABTextFieldItem>>>,
    pub min: i32,
    pub max: i32,
    pub maxChars: i32,
}


impl NumberTextFieldItemValidator {
    

    pub fn new_struct(mut textFieldItem: Option<Rc<RefCell<ABTextFieldItem>>>, mut maxChars: i32, mut min: i32, mut max: i32) -> Self {
        let mut this = Self {
            base: ValidatorBase::new_struct(),
            textFieldItem: None,
            min: 0,
            max: 0,
            maxChars: 0,
        };
        this.textFieldItem = textFieldItem.clone();
        this.min = min.clone();
        this.max = max.clone();
        this.maxChars = maxChars.clone();
        this
    }

    pub fn new(mut textFieldItem: Option<Rc<RefCell<ABTextFieldItem>>>, mut maxChars: i32, mut min: i32, mut max: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(textFieldItem, maxChars, min, max))))
    }
    

    pub fn isNumberValid(&mut self, mut value: Option<i32>) -> Option<bool> {
        return jcall!(BooleanFactory::getInstance()).TRUE.clone();
    }
    

    pub fn toNumberVector(&mut self, mut value: Option<i32>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        let mut result: Option<bool> = jcall!(BooleanFactory::getInstance()).TRUE.clone();
        let mut string: String = jcall!(self.textFieldItem).getString();
        let mut textLength: i32 = (string.chars().count() as i32);
        if textLength > 0 && textLength < self.maxChars {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                jcall!(self.textFieldItem).getString().parse::<i32>().unwrap();
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                result = jcall!(BooleanFactory::getInstance()).FALSE.clone();
                }
            }
        } else {
            if textLength < 1 {
                result = jcall!(BooleanFactory::getInstance()).FALSE.clone();
            } else if textLength > self.maxChars {
                result = jcall!(BooleanFactory::getInstance()).FALSE.clone();
            }
        }
        return result.clone();
    }
    

    pub fn toList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut string: String = jcall!(self.textFieldItem).getString();
        let mut textLength: i32 = (string.chars().count() as i32);
        let mut label: String = jcall!(self.textFieldItem).getLabel();
        let mut name: String = label[(0) as usize..((label.chars().count() as i32) - 2) as usize].to_string();
        let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        if textLength > 0 && textLength < self.maxChars {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut number: Option<i32> = jcall!(smallIntegerSingletonFactory).createInstance(jcall!(self.textFieldItem).getString().parse::<i32>().unwrap());
                if ((number).unwrap() as i32) > self.min {
                    jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                    jcall!(list).add(crate::java_into_object(jcall!(stringMaker).append(name.clone()).append(String::from(" is to small")).toString()));
                } else if ((number).unwrap() as i32) > self.max {
                    jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                    jcall!(list).add(crate::java_into_object(jcall!(stringMaker).append(name.clone()).append(String::from(" is to large")).toString()));
                }
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                jcall!(list).add(crate::java_into_object(jcall!(stringMaker).append(name.clone()).append(String::from(" is not a number")).toString()));
                }
            }
        } else {
            if textLength < 1 {
                jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                jcall!(list).add(crate::java_into_object(jcall!(stringMaker).append(name.clone()).append(String::from(" is to short")).toString()));
            } else if textLength > self.maxChars {
                jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                jcall!(list).add(crate::java_into_object(jcall!(stringMaker).append(name.clone()).append(String::from(" is to long")).toString()));
            }
        }
        return list.clone();
    }
}
impl std::ops::Deref for NumberTextFieldItemValidator {
    type Target = ValidatorBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NumberTextFieldItemValidator {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::control::validate::ValidatorBase::ValidatorBaseDyn for NumberTextFieldItemValidator {
    fn as_ValidatorBase(&self) -> &crate::org::allbinary::logic::control::validate::ValidatorBase::ValidatorBase { &self.base }
    fn as_ValidatorBase_mut(&mut self) -> &mut crate::org::allbinary::logic::control::validate::ValidatorBase::ValidatorBase { &mut self.base }
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toList(self) }
}


