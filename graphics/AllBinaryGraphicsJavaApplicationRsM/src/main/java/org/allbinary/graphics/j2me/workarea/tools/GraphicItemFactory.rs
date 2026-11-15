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
// package org.allbinary.graphics.j2me.workarea.tools
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::log::LOGGING::LOGGING;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::w3c::dom::Node::Node;
use crate::java::lang::Exception::*;
use super::SelectionToolFactory::SelectionToolFactory;
use super::GraphicsItemInterfaceFactoryInterface::GraphicsItemInterfaceFactoryInterface;
use super::LinesGraphicItemFactory::LinesGraphicItemFactory;
use super::GraphicItemInterface::GraphicItemInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GraphicItemFactory {
    pub graphicItems: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>,
    pub DEFAULT: Option<Rc<RefCell<dyn GraphicsItemInterfaceFactoryInterface>>>,
}

pub static GraphicItemFactory_instance: crate::JavaStatic<Option<Rc<RefCell<GraphicItemFactory>>>> = crate::JavaStatic::new(|| GraphicItemFactory::new());

impl GraphicItemFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GraphicItemFactory>>> {
        return GraphicItemFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            graphicItems: None,
            DEFAULT: None,
        };
        this.DEFAULT = SelectionToolFactory::new().map(|reference| reference as Rc<RefCell<dyn GraphicsItemInterfaceFactoryInterface>>).map(|reference| reference as Rc<RefCell<dyn GraphicsItemInterfaceFactoryInterface>>);
        this.graphicItems = { let java_value = (jcall!(StdUtil::getInstance()).createHashMap()).clone(); java_value };
        { let java_receiver = this.graphicItems.clone(); let java_arguments = (crate::java_into_object(LinesGraphicItem::LinesGraphicItem::getStaticName()), crate::java_into_object(LinesGraphicItemFactory::new()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getInstance1(&mut self, mut itemName: String) -> Option<Rc<RefCell<dyn GraphicsItemInterfaceFactoryInterface>>> {
        let mut graphicsItemInterfaceFactoryInterface: Option<Rc<RefCell<dyn GraphicsItemInterfaceFactoryInterface>>> = { let java_object = (jcall!(self.graphicItems).get(crate::java_into_object(itemName.clone()))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn GraphicsItemInterfaceFactoryInterface>>>>().expect("ClassCastException").clone()) };
        if graphicsItemInterfaceFactoryInterface.is_none() {
            graphicsItemInterfaceFactoryInterface = self.DEFAULT.clone();
        }
        return graphicsItemInterfaceFactoryInterface.clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance1_BasicArrayList(&mut self, mut graphicItemNodeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        let mut numberOfItems: i32 = jcall!(graphicItemNodeList).size();
        let mut graphicItemHashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        if LOGGING::contains(crate::org::allbinary::log::LOGGING::LOGGING_MENUEVENT.with(Clone::clone).clone()) {
        }
        for index in (0..numberOfItems) {
            let mut graphicItem: Option<Rc<RefCell<dyn GraphicItemInterface>>> = GraphicItemBuilder::GraphicItemBuilder::getInstance((jcall!(graphicItemNodeList).get(index.clone())).clone() /* as Node */);
            jcall!(graphicItemHashMap).put(crate::java_into_object(jcall!(graphicItem).getTreeNode()), crate::java_into_object(graphicItem.clone()));
        }
        return graphicItemHashMap.clone();
    }
}

