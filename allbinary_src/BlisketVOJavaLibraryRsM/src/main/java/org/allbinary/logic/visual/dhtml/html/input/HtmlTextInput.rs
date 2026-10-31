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
// package org.allbinary.logic.visual.dhtml.html.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::HtmlLengthInput::HtmlLengthInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HtmlTextInput {
    pub base: HtmlLengthInput,
}


impl HtmlTextInput {
    

    pub fn new_struct(mut before: String, mut name: String, mut value: String, mut after: String) -> Self {
        let mut this = Self {
            base: HtmlLengthInput::new_struct(before.clone(), jcall!(HtmlInputData::HtmlInputData::getInstance()).TEXT.clone(), name.clone(), value.clone(), after.clone()),
        };
        // super(...) is in the struct literal above
        this.setSize(Some((19) as i32));
        this.setMaxLength(Some((60) as i32));
        this
    }

    pub fn new(mut before: String, mut name: String, mut value: String, mut after: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(before, name, value, after))))
    }
}
impl std::ops::Deref for HtmlTextInput {
    type Target = HtmlLengthInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HtmlTextInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::dhtml::html::input::HtmlInput::HtmlInputDyn for HtmlTextInput {
    fn as_HtmlInput(&self) -> &crate::org::allbinary::logic::visual::dhtml::html::input::HtmlInput::HtmlInput { &self.base.base.base }
    fn as_HtmlInput_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::dhtml::html::input::HtmlInput::HtmlInput { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::visual::dhtml::html::input::HtmlLengthInput::HtmlLengthInputDyn for HtmlTextInput {
    fn as_HtmlLengthInput(&self) -> &crate::org::allbinary::logic::visual::dhtml::html::input::HtmlLengthInput::HtmlLengthInput { &self.base }
    fn as_HtmlLengthInput_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::dhtml::html::input::HtmlLengthInput::HtmlLengthInput { &mut self.base }
}
impl crate::org::allbinary::logic::visual::dhtml::html::input::HtmlValueInput::HtmlValueInputDyn for HtmlTextInput {
    fn as_HtmlValueInput(&self) -> &crate::org::allbinary::logic::visual::dhtml::html::input::HtmlValueInput::HtmlValueInput { &self.base.base }
    fn as_HtmlValueInput_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::dhtml::html::input::HtmlValueInput::HtmlValueInput { &mut self.base.base }
}
impl crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTagDyn for HtmlTextInput {
    fn as_HtmlTag(&self) -> &crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTag { &self.base.base.base.base }
    fn as_HtmlTag_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTag { &mut self.base.base.base.base }
}


