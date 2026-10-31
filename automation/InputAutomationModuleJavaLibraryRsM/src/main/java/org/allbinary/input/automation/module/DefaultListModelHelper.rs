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
// package org.allbinary.input.automation.module
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::swing::*;
use crate::org::allbinary::logic::StdUtil::StdUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DefaultListModelHelper {
    pub defaultListModel: Option<Rc<RefCell<DefaultListModel>>>,
    pub vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl DefaultListModelHelper {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            defaultListModel: None,
            vector: None,
        };
        this.vector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.defaultListModel = DefaultListModel::new();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn initDefaultModelList(&mut self) {
        self.defaultListModel = DefaultListModel::new();
        let mut size: i32 = jcall!(self.vector).size();
        for index in (0..size) {
            let mut moduleName: String = (jcall!(self.vector).get(index.clone())).clone() /* as String */;
            jcall!(self.defaultListModel).addElement(moduleName.clone());
        }
    }
    

    pub fn getListModel(&mut self) -> Option<Rc<RefCell<ListModel>>> {
        return (self.defaultListModel).clone() /* as ListModel */;
    }
    

    pub fn add(&mut self, mut string: String) {
        jcall!(self.vector).add(crate::java_into_object(string.clone()));
    }
    

    pub fn remove(&mut self, mut string: String) {
        jcall!(self.vector).remove(crate::java_into_object(string.clone()));
    }
}

