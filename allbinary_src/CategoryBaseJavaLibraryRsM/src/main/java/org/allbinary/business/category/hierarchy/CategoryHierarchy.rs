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
// package org.allbinary.business.category.hierarchy
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::category::CategoryData::CategoryData;
use crate::org::allbinary::business::category::CategoryInterface::CategoryInterface;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::CategoryHierarchyInterface::CategoryHierarchyInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CategoryHierarchy {
    pub rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>,
    pub parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>,
    pub level: i32,
}

impl CategoryHierarchyInterface for CategoryHierarchy {
    fn getLevel(&mut self) -> i32 { Self::getLevel(self) }
    fn getLevelString(&mut self) -> String { Self::getLevelString(self) }
    fn getRoot(&mut self) -> Option<Rc<RefCell<dyn CategoryInterface>>> { Self::getRoot(self) }
    fn getParent(&mut self) -> Option<Rc<RefCell<dyn CategoryInterface>>> { Self::getParent(self) }
    fn setLevel(&mut self, mut level: i32) { Self::setLevel(self, level) }
    fn setRoot(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) { Self::setRoot(self, categoryInterface) }
    fn setParent(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) { Self::setParent(self, categoryInterface) }
}

impl CategoryHierarchy {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct2(mut rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> Self {
        let mut this = Self {
            rootCategoryInterface: None,
            parentCategoryInterface: None,
            level: 0,
        };
        this.level = 0;
        this.setRoot(rootCategoryInterface.clone());
        this.setParent(parentCategoryInterface.clone());
        this
    }

    pub fn new2(mut rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(rootCategoryInterface, parentCategoryInterface))))
    }
    

    pub fn new_struct3(mut rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut level: i32) -> Self {
        let mut this = Self {
            rootCategoryInterface: None,
            parentCategoryInterface: None,
            level: 0,
        };
        this.level = level.clone();
        this.setRoot(rootCategoryInterface.clone());
        this.setParent(parentCategoryInterface.clone());
        this
    }

    pub fn new3(mut rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut level: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3(rootCategoryInterface, parentCategoryInterface, level))))
    }
    

    pub fn new_struct3_CategoryInterface_CategoryInterface_HashMap(mut rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut categoryPropertiesHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Self {
        let mut this = Self {
            rootCategoryInterface: None,
            parentCategoryInterface: None,
            level: 0,
        };
        let mut levelString: String = jcall!(StringUtil::getInstance()).getNonNull({ let java_object = (jcall!(categoryPropertiesHashMap).get(crate::java_into_object({ let java_field = jcall!(CategoryData::getInstance()).LEVEL.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() });
        this.level = { let java_value = (((Some((levelString.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32)).clone(); java_value };
        this.setRoot(rootCategoryInterface.clone());
        this.setParent(parentCategoryInterface.clone());
        this
    }

    pub fn new3_CategoryInterface_CategoryInterface_HashMap(mut rootCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut categoryPropertiesHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct3_CategoryInterface_CategoryInterface_HashMap(rootCategoryInterface, parentCategoryInterface, categoryPropertiesHashMap))))
    }
    

    pub fn getLevel(&mut self) -> i32 {
        return self.level.clone();
    }
    

    pub fn getLevelString(&mut self) -> String {
        return (self.level.clone()).to_string();
    }
    

    pub fn setLevel(&mut self, mut level: i32) {
        self.level = level.clone();
    }
    

    pub fn setParent(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) {
        self.parentCategoryInterface = categoryInterface.clone();
    }
    

    pub fn setRoot(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) {
        self.rootCategoryInterface = categoryInterface.clone();
    }
    

    pub fn getRoot(&mut self) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        return self.rootCategoryInterface.clone();
    }
    

    pub fn getParent(&mut self) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        return self.parentCategoryInterface.clone();
    }
}

