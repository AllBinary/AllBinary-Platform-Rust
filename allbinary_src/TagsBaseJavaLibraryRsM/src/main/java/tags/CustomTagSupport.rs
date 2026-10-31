// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package tags
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::servlet::jsp::tagext::TagSupport::TagSupport;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::TagStrings::TagStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomTagSupport {
    pub base: TagSupport,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub tagStrings: Option<Rc<RefCell<TagStrings>>>,
}


impl CustomTagSupport {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TagSupport::new_struct(),
            commonStrings: None,
            tagStrings: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.tagStrings = TagStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for CustomTagSupport {
    type Target = TagSupport;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CustomTagSupport {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CustomTagSupportDyn {
    fn as_CustomTagSupport(&self) -> &CustomTagSupport;
    fn as_CustomTagSupport_mut(&mut self) -> &mut CustomTagSupport;
}
impl CustomTagSupportDyn for CustomTagSupport {
    fn as_CustomTagSupport(&self) -> &CustomTagSupport { self }
    fn as_CustomTagSupport_mut(&mut self) -> &mut CustomTagSupport { self }
}



