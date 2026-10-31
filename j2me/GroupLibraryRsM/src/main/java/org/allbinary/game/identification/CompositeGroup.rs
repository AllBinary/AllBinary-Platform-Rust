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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::Group::Group;
use super::GroupCommonFactory::GroupCommonFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CompositeGroup {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub groupInterface: Option<Rc<RefCell<Group>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl CompositeGroup {
    

    pub fn new_struct(mut groupInterface: Option<Rc<RefCell<Group>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            groupInterface: None,
            list: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.groupInterface = groupInterface.clone();
        this
    }

    pub fn new(mut groupInterface: Option<Rc<RefCell<Group>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(groupInterface))))
    }
    

    pub fn clear(&mut self) {
        jcall!(self.list).clear();
    }
    

    pub fn add(&mut self, mut groupInterface: Option<Rc<RefCell<Group>>>) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut groupCommonFactory: Option<Rc<RefCell<GroupCommonFactory>>> = GroupCommonFactory::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(groupCommonFactory).ADDING_LABEL.clone());
        jcall!(stringBuffer).append(jcall!(groupInterface).toString());
        jcall!(stringBuffer).append(jcall!(groupCommonFactory).TO_LABEL.clone());
        jcall!(stringBuffer).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(self.groupInterface.clone())));
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).ADD.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(self.list).add(crate::java_into_object(groupInterface.clone()));
    }
    

    pub fn isInGroup(&mut self, mut groupInterface: Option<Rc<RefCell<Group>>>) -> bool {
        if jcall!(self.list).contains(crate::java_into_object(groupInterface.clone())) {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn getGroup(&mut self) -> Option<Rc<RefCell<Group>>> {
        return self.groupInterface.clone();
    }
}

