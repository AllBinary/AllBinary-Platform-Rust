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
use crate::org::allbinary::business::category::CategoryComponent::CategoryComponent;
use crate::org::allbinary::business::category::CategoryData::CategoryData;
use crate::org::allbinary::business::category::CategoryFactoryInterface::CategoryFactoryInterface;
use crate::org::allbinary::business::category::CategoryInterface::CategoryInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::crypt::file::CryptFileWriter::CryptFileWriter;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CategoryPrivateTree {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub categoryFactoryInterface: Option<Rc<RefCell<dyn CategoryFactoryInterface>>>,
}


impl CategoryPrivateTree {
    

    pub fn new_struct(mut categoryFactoryInterface: Option<Rc<RefCell<dyn CategoryFactoryInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            categoryFactoryInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).putF(jcall!(commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("CategoryPrivateTree(CategoryFactoryInterface categoryFactoryInterface)"));
        }
        this.categoryFactoryInterface = categoryFactoryInterface.clone();
        this
    }

    pub fn new(mut categoryFactoryInterface: Option<Rc<RefCell<dyn CategoryFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(categoryFactoryInterface))))
    }
    

    // throws Exception
    // synchronized
    pub fn save(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) {
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Saving Category File: "), jcall!(categoryInterface).getFilePath()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("save()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut document: Option<Rc<RefCell<Document>>> = jcall!(CategoryComponent::new(categoryInterface.clone())).toXmlDoc();
        let mut cryptFileWriter: Option<Rc<RefCell<CryptFileWriter>>> = CryptFileWriter::new(jcall!(CategoryData::getInstance()).UNCRYPTED_EXTENSION.clone(), jcall!(CategoryData::getInstance()).ENCRYPTED_EXTENSION.clone());
        jcall!(cryptFileWriter).write(jcall!(categoryInterface).getFilePath(), document.clone());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
            jcall!(self.logUtil).putF(String::from("end"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("save()"));
        }
    }
    

    // throws Exception
    // synchronized
    pub fn delete(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) {
        if jcall!(categoryInterface).isLeaf() {
            let mut categoryFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFileFromAbPath(jcall!(categoryInterface).getFilePath());
            if jcall!(categoryFile).delete() {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Removed: "), jcall!(categoryInterface).getFilePath()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("delete(CategoryInterface)"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
            }
        } else {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(String::from("Not a leaf unable to remove"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("delete(CategoryInterface)"));
            }
        }
    }
}
pub trait CategoryPrivateTreeDyn {
    fn as_CategoryPrivateTree(&self) -> &CategoryPrivateTree;
    fn as_CategoryPrivateTree_mut(&mut self) -> &mut CategoryPrivateTree;
    fn save(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) { CategoryPrivateTree::save(self.as_CategoryPrivateTree_mut(), categoryInterface) }
    fn delete(&mut self, mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) { CategoryPrivateTree::delete(self.as_CategoryPrivateTree_mut(), categoryInterface) }
}
impl CategoryPrivateTreeDyn for CategoryPrivateTree {
    fn as_CategoryPrivateTree(&self) -> &CategoryPrivateTree { self }
    fn as_CategoryPrivateTree_mut(&mut self) -> &mut CategoryPrivateTree { self }
}


