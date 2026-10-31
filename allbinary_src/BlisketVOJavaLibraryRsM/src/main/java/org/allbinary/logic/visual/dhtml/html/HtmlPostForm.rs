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
// package org.allbinary.logic.visual.dhtml.html
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::HtmlForm::HtmlForm;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HtmlPostForm {
    pub base: HtmlForm,
}


impl HtmlPostForm {
    

    pub fn new_struct(mut action: String) -> Self {
        let mut this = Self {
            base: HtmlForm::new_struct(action.clone()),
        };
        // super(...) is in the struct literal above
        this.setPost();
        this
    }

    pub fn new(mut action: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(action))))
    }
}
impl std::ops::Deref for HtmlPostForm {
    type Target = HtmlForm;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HtmlPostForm {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::dhtml::html::HtmlForm::HtmlFormDyn for HtmlPostForm {
    fn as_HtmlForm(&self) -> &crate::org::allbinary::logic::visual::dhtml::html::HtmlForm::HtmlForm { &self.base }
    fn as_HtmlForm_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::dhtml::html::HtmlForm::HtmlForm { &mut self.base }
}


