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
// package org.allbinary.graphics.form.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Command::Command;
// use crate::javax::microedition::lcdui::ImageItem::ImageItem;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::form::ScreenInfo::ScreenInfo;
use crate::org::allbinary::logic::VectorUtil::VectorUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::util::visitor::Visitor::Visitor;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::CommandTextItem::CommandTextItem;
use super::ABCustomItem::ABCustomItem;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CommandTextItemArrayFactory {
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub visitorInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>>,
}


impl CommandTextItemArrayFactory {
    

    pub fn new_struct(mut visitorInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>>) -> Self {
        let mut this = Self {
            list: None,
            visitorInterface: None,
        };
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.visitorInterface = visitorInterface.clone();
        this
    }

    pub fn new(mut visitorInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(visitorInterface))))
    }
    

    pub fn getInstance(&mut self, mut vector: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>> {
        let mut vectorUtil: Option<Rc<RefCell<VectorUtil>>> = VectorUtil::getInstance();
        let mut size: i32 = jcall!(vectorUtil).getSize(crate::java_into_object(vector.clone()));
        jcall!(self.list).clear();
        let mut textItem: Option<Rc<RefCell<CommandTextItem>>> = None;
        let mut priorityLimit: i32 = 7;
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        let mut isLargeEnoughDisplayForBigMenu: bool = (jcall!(displayInfo).isPortrait() && jcall!(displayInfo).getLastHeight() >= jcall!(ScreenInfo::getInstance()).MEDIUM_WIDTH) || (!jcall!(displayInfo).isPortrait() && jcall!(displayInfo).getLastWidth() >= jcall!(ScreenInfo::getInstance()).MEDIUM_WIDTH);
        if !isLargeEnoughDisplayForBigMenu && size > 3 {
            priorityLimit = 3;
        }
        let mut command: Option<Rc<RefCell<Command>>> = None;
        for index in (0..size) {
            command = (jcall!(vectorUtil).elementAt(crate::java_into_object(vector.clone()), index.clone())).clone() /* as Command */;
            if jcall!(command).getPriority() < priorityLimit {
                let mut aBoolean: Option<bool> = { let java_object = jcall!(self.visitorInterface).visit(crate::java_into_object(command.clone())); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<bool>().copied().or_else(|| object.downcast_ref::<Option<bool>>().copied().flatten()).expect("ClassCastException") }) };
                if aBoolean.unwrap() {
                    textItem = CommandTextItem::new(command.clone(), unsafe { crate::javax::microedition::lcdui::ImageItem::ImageItem_LAYOUT_DEFAULT.clone() }.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone());
                    jcall!(self.list).add(crate::java_into_object(textItem.clone()));
                }
            }
        }
        let mut textItemArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>> = vec![None; (jcall!(self.list).size()) as usize];
        let mut size2: i32 = (textItemArray.len() as i32).clone();
        for index in (0..size2) {
            textItemArray[(index) as usize] = { let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>().expect("ClassCastException").clone()) };
        }
        return textItemArray.clone();
    }
}

