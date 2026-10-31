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
// package org.allbinary.business.category.properties
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::validate::ValidationInterface::ValidationInterface;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::CategoryPropertiesView::CategoryPropertiesView;
use super::CategoryPropertiesInterface::CategoryPropertiesInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CategoryPropertiesValidationView {
    pub base: CategoryPropertiesView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

impl ValidationInterface for CategoryPropertiesValidationView {
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}

impl CategoryPropertiesValidationView {
    

    pub fn new_struct(mut categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>>) -> Self {
        let mut this = Self {
            base: CategoryPropertiesView::new_struct(categoryPropertiesInterface.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(categoryPropertiesInterface))))
    }
    

    // throws Exception
    pub fn isValid(&mut self) -> Option<bool> {
        return jcall!(self.categoryPropertiesInterface).isValid();
    }
    

    // throws Exception
    pub fn validationInfo(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(jcall!(self.categoryPropertiesInterface).validationInfo());
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to generate validation info error"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"), crate::java_into_object(e.clone()));
            }
            return String::from("Error Validating Data");
            }
        }
    }
    

    // throws Exception
    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    // throws Exception
    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
}
impl std::ops::Deref for CategoryPropertiesValidationView {
    type Target = CategoryPropertiesView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CategoryPropertiesValidationView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::category::properties::CategoryPropertiesView::CategoryPropertiesViewDyn for CategoryPropertiesValidationView {
    fn as_CategoryPropertiesView(&self) -> &crate::org::allbinary::business::category::properties::CategoryPropertiesView::CategoryPropertiesView { &self.base }
    fn as_CategoryPropertiesView_mut(&mut self) -> &mut crate::org::allbinary::business::category::properties::CategoryPropertiesView::CategoryPropertiesView { &mut self.base }
}


