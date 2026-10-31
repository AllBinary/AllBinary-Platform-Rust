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
// package org.allbinary.logic.io.file.visitor
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::util::visitor::VisitorInterface::VisitorInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BooleanFileVisitor {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub filterStringBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

// impl VisitorInterface for BooleanFileVisitor with the methods of the trait

impl BooleanFileVisitor {
    

    pub fn new_struct(mut filterStringBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Self {
        let mut this = Self {
            commonStrings: None,
            filterStringBasicArrayList: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.setFilterStringBasicArrayList(filterStringBasicArrayList.clone());
        PreLogUtil::put(format!("{}{}", String::from("Filter BasicArrayList: "), jcall!(this.getFilterStringBasicArrayList()).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this
    }

    pub fn new(mut filterStringBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(filterStringBasicArrayList))))
    }
    

    pub fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return (self.visit((anyType).clone() /* as AbFile */)).clone() /* as Any */;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn visit(&mut self, mut file: Option<Rc<RefCell<AbFile>>>) -> Option<bool> {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getFilterStringBasicArrayList();
        let mut size: i32 = jcall!(list).size();
        let mut nextFileFilterString: String = String::new();
        for index in (0..size) {
            nextFileFilterString = (jcall!(list).get(index.clone())).clone() /* as String */;
            if self.visit2(file.clone(), nextFileFilterString.clone()).unwrap() {
                return true;
            }
        }
        return false;
    }
    

    pub fn getFilterStringBasicArrayList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.filterStringBasicArrayList.clone();
    }
    

    pub fn setFilterStringBasicArrayList(&mut self, mut filterStringBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.filterStringBasicArrayList = filterStringBasicArrayList.clone();
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn visit2(&mut self, mut file: Option<Rc<RefCell<AbFile>>>, mut fileNameString: String) -> Option<bool> {
        return false;
    }
}
pub trait BooleanFileVisitorDyn {
    fn as_BooleanFileVisitor(&self) -> &BooleanFileVisitor;
    fn as_BooleanFileVisitor_mut(&mut self) -> &mut BooleanFileVisitor;
    fn getFilterStringBasicArrayList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { BooleanFileVisitor::getFilterStringBasicArrayList(self.as_BooleanFileVisitor_mut()) }
    fn setFilterStringBasicArrayList(&mut self, mut filterStringBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { BooleanFileVisitor::setFilterStringBasicArrayList(self.as_BooleanFileVisitor_mut(), filterStringBasicArrayList) }
    fn visit2(&mut self, mut file: Option<Rc<RefCell<AbFile>>>, mut fileNameString: String) -> Option<bool> { BooleanFileVisitor::visit2(self.as_BooleanFileVisitor_mut(), file, fileNameString) }
}
impl BooleanFileVisitorDyn for BooleanFileVisitor {
    fn as_BooleanFileVisitor(&self) -> &BooleanFileVisitor { self }
    fn as_BooleanFileVisitor_mut(&mut self) -> &mut BooleanFileVisitor { self }
}


