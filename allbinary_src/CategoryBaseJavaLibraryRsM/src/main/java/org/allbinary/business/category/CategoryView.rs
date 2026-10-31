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
// package org.allbinary.business.category
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::category::properties::CategoryPropertiesInterface::CategoryPropertiesInterface;
use crate::org::allbinary::business::category::properties::CategoryPropertiesView::CategoryPropertiesView;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::java::anyType::InterfaceUtil::InterfaceUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::CategoryInterface::CategoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CategoryView {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>,
}

impl DomNodeInterface for CategoryView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl CategoryView {
    

    pub fn new_struct(mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            categoryInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.categoryInterface = categoryInterface.clone();
        this
    }

    pub fn new(mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(categoryInterface))))
    }
    

    pub fn getCategoryInterface(&mut self) -> Option<Rc<RefCell<dyn CategoryInterface>>> {
        return self.categoryInterface.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(jcall!(CategoryData::CategoryData::getInstance()).NAME.clone());
        let mut childCategoryVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.categoryInterface).getChildNodes();
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).CATEGORY.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Number Of Children: "), jcall!(childCategoryVector).size()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        let mut size: i32 = jcall!(childCategoryVector).size();
        for i in (0..size) {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(childCategoryVector).get(i.clone());
            let mut categoryPropertiesInterface: Option<Rc<RefCell<dyn CategoryPropertiesInterface>>> = None;
            if InterfaceUtil::isImplemented(String::from("CategoryPropertiesInterface"), anyType.clone()) {
                categoryPropertiesInterface = (anyType).clone() /* as CategoryPropertiesInterface */;
            } else if InterfaceUtil::isImplemented(String::from("CategoryInterface"), anyType.clone()) {
                let mut categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>> = (anyType).clone() /* as CategoryInterface */;
                categoryPropertiesInterface = jcall!(categoryInterface).getProperties();
            } else {
                let mut isImpl: String = InterfaceUtil::viewAll(std::any::type_name_of_val(&anyType), jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", format!("{}{}", String::from("CategoryView found unknown object.\n"), String::from("Show Testing:")), isImpl))));
            }
            let mut childCategoryNode: Option<Rc<RefCell<Node>>> = jcall!(CategoryPropertiesView::new(categoryPropertiesInterface.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::category::properties::CategoryPropertiesView::CategoryPropertiesViewDyn>>)).toXmlNode(document.clone());
            jcall!(node).appendChild(childCategoryNode.clone());
        }
        return node.clone();
    }
}
pub trait CategoryViewDyn {
    fn as_CategoryView(&self) -> &CategoryView;
    fn as_CategoryView_mut(&mut self) -> &mut CategoryView;
    fn getCategoryInterface(&mut self) -> Option<Rc<RefCell<dyn CategoryInterface>>> { CategoryView::getCategoryInterface(self.as_CategoryView_mut()) }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { CategoryView::toXmlNode(self.as_CategoryView_mut(), document) }
}
impl CategoryViewDyn for CategoryView {
    fn as_CategoryView(&self) -> &CategoryView { self }
    fn as_CategoryView_mut(&mut self) -> &mut CategoryView { self }
}


