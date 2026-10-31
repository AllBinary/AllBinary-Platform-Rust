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
use crate::org::allbinary::graphics::j2me::StatusFactory::StatusFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::GraphicsItemInterfaceFactoryInterface::GraphicsItemInterfaceFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ToolFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub selectedTool: Option<Rc<RefCell<dyn GraphicsItemInterfaceFactoryInterface>>>,
    pub tool: String,
    pub SELECTED_LABEL: String,
}

thread_local! { pub static ToolFactory_instance: Option<Rc<RefCell<ToolFactory>>> = ToolFactory::new(); }

impl ToolFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            selectedTool: None,
            tool: String::new(),
            SELECTED_LABEL: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.selectedTool = None;
        this.tool = String::new();
        this.SELECTED_LABEL = String::from("Selected: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ToolFactory>>> {
        return ToolFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn setSelectedTool(&mut self, mut tool: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(StatusFactory::getInstance()).setStatus(format!("{}{}", self.SELECTED_LABEL, tool));
            self.tool = tool.clone();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setSelectionTool"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getSelectedToolFactory(&mut self) -> Option<Rc<RefCell<dyn GraphicsItemInterfaceFactoryInterface>>> {
        jcall!(StatusFactory::getInstance()).setStatus(format!("{}{}", String::from("Tool In Use: "), self.tool));
        if true {
            self.selectedTool = jcall!(GraphicItemFactory::GraphicItemFactory::getInstance()).getInstance1(self.tool.clone());
            jcall!(self.logUtil).putF(format!("{}{}", String::from("New Tool: "), self.selectedTool), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getSelectedToolFactory"));
            return self.selectedTool.clone();
        } else {
            return None;
        }
    }
}

