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
// package org.allbinary.logic.visual.transform.template.customizer.includes.style.css.template.retail
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::logic::control::validate::ValidationInterface::ValidationInterface;
use crate::org::allbinary::logic::visual::dhtml::style::StyleData::StyleData;
use crate::org::allbinary::logic::visual::dhtml::style::StylesData::StylesData;
use crate::org::allbinary::logic::visual::dhtml::style::color::ColorsView::ColorsView;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::CssStyleValidation::CssStyleValidation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct EditCssStyleValidation {
    pub base: CssStyleValidation,
}

impl ValidationInterface for EditCssStyleValidation {
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}
impl DomNodeInterface for EditCssStyleValidation {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl EditCssStyleValidation {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CssStyleValidation::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut document: Option<Rc<RefCell<Document>>>) -> Self {
        let mut this = Self {
            base: CssStyleValidation::new_struct1(document.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(document))))
    }
    

    pub fn new_struct1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            base: CssStyleValidation::new_struct1(hashMap.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut hashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(hashMap))))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        return self.base.isValid();
    }
    

    pub fn validationInfo(&mut self) -> String {
        return self.base.validationInfo();
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(jcall!(StylesData::getInstance()).NAME.clone());
        let mut styleNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(jcall!(StyleData::getInstance()).NAME.clone());
        jcall!(node).appendChild(styleNode.clone());
        jcall!(styleNode).appendChild(jcall!(ColorsView::new()).toXmlNode(document.clone()));
        let mut size: i32 = jcall!(self.cssStyleElementVector).size();
        for i in (0..size) {
            let mut styleDomNodeInterface: Option<Rc<RefCell<dyn DomNodeInterface>>> = { let java_object = jcall!(self.base.cssStyleElementVector).get(i.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn DomNodeInterface>>>>().expect("ClassCastException").clone()) };
            jcall!(styleNode).appendChild(jcall!(styleDomNodeInterface).toXmlNode(document.clone()));
        }
        return node.clone();
    }
}
impl std::ops::Deref for EditCssStyleValidation {
    type Target = CssStyleValidation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for EditCssStyleValidation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::transform::template::customizer::includes::style::css::template::retail::CssStyleValidation::CssStyleValidationDyn for EditCssStyleValidation {
    fn as_CssStyleValidation(&self) -> &crate::org::allbinary::logic::visual::transform::template::customizer::includes::style::css::template::retail::CssStyleValidation::CssStyleValidation { &self.base }
    fn as_CssStyleValidation_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::template::customizer::includes::style::css::template::retail::CssStyleValidation::CssStyleValidation { &mut self.base }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl crate::org::allbinary::logic::control::validate::Validation::ValidationDyn for EditCssStyleValidation {
    fn as_Validation(&self) -> &crate::org::allbinary::logic::control::validate::Validation::Validation { &self.base.base }
    fn as_Validation_mut(&mut self) -> &mut crate::org::allbinary::logic::control::validate::Validation::Validation { &mut self.base.base }
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}


