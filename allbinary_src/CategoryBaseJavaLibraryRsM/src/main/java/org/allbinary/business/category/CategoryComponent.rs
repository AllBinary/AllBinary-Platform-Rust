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
// package org.allbinary.business.category
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::tree::dom::document::DomDocumentHelper::DomDocumentHelper;
use crate::org::allbinary::logic::visual::transform::TransformInterface::TransformInterface;
use crate::org::allbinary::logic::visual::transform::data::TransformDocumentInterface::TransformDocumentInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::w3c::dom::Document::Document;
use super::CategoryView::CategoryView;
use super::CategoryInterface::CategoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CategoryComponent {
    pub base: CategoryView,
    pub NO_TYPE: i32,
}

impl TransformInterface for CategoryComponent {
    fn getTypeId(&mut self) -> i32 { Self::getTypeId(self) }
    fn setTransformDocumentInterface(&mut self, mut viewDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>>) { Self::setTransformDocumentInterface(self, viewDocumentInterface) }
    fn getTransformDocumentInterface(&mut self) -> Option<Rc<RefCell<dyn TransformDocumentInterface>>> { Self::getTransformDocumentInterface(self) }
    fn view(&mut self) -> String { Self::view(self) }
}
impl crate::org::allbinary::logic::visual::transform::info::CompositeTransformInfoInterface::CompositeTransformInfoInterface for CategoryComponent {
    fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> { Self::getTransformInfoInterface(self) }
}
impl crate::org::allbinary::data::tree::dom::document::mapping::DomDocumentMappingInterface::DomDocumentMappingInterface for CategoryComponent {
    fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toXmlDoc(self) }
}

impl CategoryComponent {
    

    pub fn new_struct(mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> Self {
        let mut this = Self {
            base: CategoryView::new_struct(categoryInterface.clone()),
            NO_TYPE: 0,
        };
        this.NO_TYPE = 0;
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(categoryInterface))))
    }
    

    pub fn getTypeId(&mut self) -> i32 {
        return self.NO_TYPE.clone();
    }
    

    pub fn getTransformDocumentInterface(&mut self) -> Option<Rc<RefCell<dyn TransformDocumentInterface>>> {
        return None;
    }
    

    // throws Exception
    pub fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn TransformInfoInterface>>> {
        return None;
    }
    

    pub fn setTransformDocumentInterface(&mut self, mut viewDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>>) {
    }
    

    // throws Exception
    pub fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        let mut document: Option<Rc<RefCell<Document>>> = DomDocumentHelper::create();
        jcall!(document).appendChild(jcall!(CategoryView::new(self.getCategoryInterface()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::category::CategoryView::CategoryViewDyn>>)).toXmlNode(document.clone()));
        return document.clone();
    }
    

    // throws Exception
    pub fn view(&mut self) -> String {
        return None;
    }
}
impl std::ops::Deref for CategoryComponent {
    type Target = CategoryView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CategoryComponent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::category::CategoryView::CategoryViewDyn for CategoryComponent {
    fn as_CategoryView(&self) -> &crate::org::allbinary::business::category::CategoryView::CategoryView { &self.base }
    fn as_CategoryView_mut(&mut self) -> &mut crate::org::allbinary::business::category::CategoryView::CategoryView { &mut self.base }
}


