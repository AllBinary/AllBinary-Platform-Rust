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
// package org.allbinary.game.identification
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::GroupInterface::GroupInterface;
use super::GroupCommonFactory::GroupCommonFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Group {
    pub name: String,
    pub teamId: i16,
    pub string: String,
}

impl GroupInterface for Group {
    fn getGroupName(&mut self) -> String { Self::getGroupName(self) }
    fn getGroupId(&mut self) -> i16 { Self::getGroupId(self) }
}

impl Group {
    

    pub fn new_struct(mut teamName: String, mut teamId: i16) -> Self {
        let mut this = Self {
            name: String::new(),
            teamId: 0,
            string: String::new(),
        };
        this.name = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.string = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.teamId = teamId.clone();
        this.setName(teamName.clone());
        this
    }

    pub fn new(mut teamName: String, mut teamId: i16) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(teamName, teamId))))
    }
    

    pub fn getGroupName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn setName(&mut self, mut name: String) {
        self.name = name.clone();
        let mut groupCommonFactory: Option<Rc<RefCell<GroupCommonFactory>>> = GroupCommonFactory::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(groupCommonFactory).GROUP_NAME_LABEL.clone());
        jcall!(stringBuffer).append(self.name.clone());
        jcall!(stringBuffer).append(jcall!(groupCommonFactory).ID_LABEL.clone());
        jcall!(stringBuffer).appendshort(self.teamId.clone());
        self.string = jcall!(stringBuffer).toString();
    }
    

    pub fn getGroupId(&mut self) -> i16 {
        return self.teamId.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return self.string.clone();
    }
}

