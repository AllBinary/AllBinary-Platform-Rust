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
// package org.allbinary.logic.visual.dhtml.style.css.property.font
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::visual::dhtml::style::css::property::CssPropertyView::CssPropertyView;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FontFamilyView {
    pub base: CssPropertyView,
}


impl FontFamilyView {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CssPropertyView::new_struct1(jcall!(FontFamilyData::FontFamilyData::getInstance()).VALUE.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            base: CssPropertyView::new_struct1(node.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
}
impl std::ops::Deref for FontFamilyView {
    type Target = CssPropertyView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for FontFamilyView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait FontFamilyViewDyn: crate::org::allbinary::logic::visual::dhtml::style::css::property::CssPropertyView::CssPropertyViewDyn {
    fn as_FontFamilyView(&self) -> &FontFamilyView;
    fn as_FontFamilyView_mut(&mut self) -> &mut FontFamilyView;
}
impl FontFamilyViewDyn for FontFamilyView {
    fn as_FontFamilyView(&self) -> &FontFamilyView { self }
    fn as_FontFamilyView_mut(&mut self) -> &mut FontFamilyView { self }
}

impl crate::org::allbinary::logic::visual::dhtml::style::css::property::CssPropertyView::CssPropertyViewDyn for FontFamilyView {
    fn as_CssPropertyView(&self) -> &crate::org::allbinary::logic::visual::dhtml::style::css::property::CssPropertyView::CssPropertyView { &self.base }
    fn as_CssPropertyView_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::dhtml::style::css::property::CssPropertyView::CssPropertyView { &mut self.base }
}


