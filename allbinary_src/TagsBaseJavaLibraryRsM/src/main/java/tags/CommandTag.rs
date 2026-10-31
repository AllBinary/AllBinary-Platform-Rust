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
// package tags
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::CustomTagSupport::CustomTagSupport;
use super::TagStrings::TagStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CommandTag {
    pub base: CustomTagSupport,
    pub tagStrings: Option<Rc<RefCell<TagStrings>>>,
    pub command: String,
}


impl CommandTag {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CustomTagSupport::new_struct(),
            tagStrings: None,
            command: String::new(),
        };
        this.tagStrings = TagStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setCommand(&mut self, mut command: String) {
        self.command = command.clone();
    }
    

    pub fn getCommand(&mut self) -> String {
        return self.command.clone();
    }
}
impl std::ops::Deref for CommandTag {
    type Target = CustomTagSupport;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CommandTag {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CommandTagDyn: crate::tags::CustomTagSupport::CustomTagSupportDyn {
    fn as_CommandTag(&self) -> &CommandTag;
    fn as_CommandTag_mut(&mut self) -> &mut CommandTag;
    fn setCommand(&mut self, mut command: String) { CommandTag::setCommand(self.as_CommandTag_mut(), command) }
    fn getCommand(&mut self) -> String { CommandTag::getCommand(self.as_CommandTag_mut()) }
}
impl CommandTagDyn for CommandTag {
    fn as_CommandTag(&self) -> &CommandTag { self }
    fn as_CommandTag_mut(&mut self) -> &mut CommandTag { self }
}

impl crate::tags::CustomTagSupport::CustomTagSupportDyn for CommandTag {
    fn as_CustomTagSupport(&self) -> &crate::tags::CustomTagSupport::CustomTagSupport { &self.base }
    fn as_CustomTagSupport_mut(&mut self) -> &mut crate::tags::CustomTagSupport::CustomTagSupport { &mut self.base }
}


