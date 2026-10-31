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
// package org.allbinary.data.tree.category
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::category::CategoryFactoryInterface::CategoryFactoryInterface;
use crate::org::allbinary::business::category::CategoryInterface::CategoryInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::io::file::directory::Directory::Directory;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::CategoryPrivateTree::CategoryPrivateTree;
use super::CategoryModifierTreeInterface::CategoryModifierTreeInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CategoryModifierTree {
    pub base: CategoryPrivateTree,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub directory: Option<Rc<RefCell<Directory>>>,
}

impl CategoryModifierTreeInterface for CategoryModifierTree {
    fn insert(&mut self, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut newChildCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) { Self::insert(self, parentCategoryInterface, newChildCategoryInterface) }
    fn delete(&mut self, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut existingChildCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) { Self::delete(self, parentCategoryInterface, existingChildCategoryInterface) }
}

impl CategoryModifierTree {
    

    pub fn new_struct(mut categoryFactoryInterface: Option<Rc<RefCell<dyn CategoryFactoryInterface>>>) -> Self {
        let mut this = Self {
            base: CategoryPrivateTree::new_struct(categoryFactoryInterface.clone()),
            logUtil: None,
            commonStrings: None,
            directory: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.directory = Directory::getInstance();
        // super(...) is in the struct literal above
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
            jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("CategoryModifierTree(CategoryFactoryInterface categoryFactoryInterface)"));
        }
        this
    }

    pub fn new(mut categoryFactoryInterface: Option<Rc<RefCell<dyn CategoryFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(categoryFactoryInterface))))
    }
    

    // synchronized
    pub fn insert(&mut self, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut newChildCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
                jcall!(self.logUtil).putF(String::from("Inserting"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert()"));
            }
            if jcall!(parentCategoryInterface).isValid().unwrap() {
                jcall!(parentCategoryInterface).addChild(newChildCategoryInterface.clone());
                let mut directoryToBeCreatedAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(format!("{}{}", jcall!(jcall!(newChildCategoryInterface).getRootFilePath()).toString(), jcall!(jcall!(newChildCategoryInterface).getPath()).toString()), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
                jcall!(self.directory).create(directoryToBeCreatedAbPath.clone());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
                    jcall!(self.logUtil).putF(String::from("Saving"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert()"));
                }
                self.save(parentCategoryInterface.clone());
                self.save(newChildCategoryInterface.clone());
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Will Not Add Null Category"))));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    // synchronized
    pub fn delete(&mut self, mut parentCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>, mut existingChildCategoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(parentCategoryInterface).isValid().unwrap() {
                jcall!(parentCategoryInterface).addChild(existingChildCategoryInterface.clone());
                let mut directoryToBeDeletedAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>> = AbPath::new(format!("{}{}", jcall!(jcall!(existingChildCategoryInterface).getRootFilePath()).toString(), jcall!(jcall!(existingChildCategoryInterface).getPath()).toString()), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>);
                self.delete(existingChildCategoryInterface.clone());
                jcall!(parentCategoryInterface).removeChild(existingChildCategoryInterface.clone());
                self.save(parentCategoryInterface.clone());
                jcall!(self.directory).remove(directoryToBeDeletedAbPath.clone());
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Will Not Add Null Category"))));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).delete.clone(), crate::java_into_object(e.clone()));
            }
            }
        }
    }
}
impl std::ops::Deref for CategoryModifierTree {
    type Target = CategoryPrivateTree;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CategoryModifierTree {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CategoryModifierTreeDyn: crate::org::allbinary::data::tree::category::CategoryPrivateTree::CategoryPrivateTreeDyn + crate::org::allbinary::data::tree::category::CategoryModifierTreeInterface::CategoryModifierTreeInterface {
    fn as_CategoryModifierTree(&self) -> &CategoryModifierTree;
    fn as_CategoryModifierTree_mut(&mut self) -> &mut CategoryModifierTree;
}
impl CategoryModifierTreeDyn for CategoryModifierTree {
    fn as_CategoryModifierTree(&self) -> &CategoryModifierTree { self }
    fn as_CategoryModifierTree_mut(&mut self) -> &mut CategoryModifierTree { self }
}

impl crate::org::allbinary::data::tree::category::CategoryPrivateTree::CategoryPrivateTreeDyn for CategoryModifierTree {
    fn as_CategoryPrivateTree(&self) -> &crate::org::allbinary::data::tree::category::CategoryPrivateTree::CategoryPrivateTree { &self.base }
    fn as_CategoryPrivateTree_mut(&mut self) -> &mut crate::org::allbinary::data::tree::category::CategoryPrivateTree::CategoryPrivateTree { &mut self.base }
}


