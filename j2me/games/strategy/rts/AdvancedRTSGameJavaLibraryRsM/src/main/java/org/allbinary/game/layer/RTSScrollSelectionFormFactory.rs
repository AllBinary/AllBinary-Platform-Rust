// 
// AllBinary Open License Version 1 
// Copyright (c) 2006 AllBinary 
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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::form::FormTypeFactory::FormTypeFactory;
use crate::org::allbinary::graphics::form::MultipleScrollSelectionHorizontalForm::MultipleScrollSelectionHorizontalForm;
use crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm;
use crate::org::allbinary::input::motion::button::CommonButtons::CommonButtons;
use crate::org::allbinary::input::motion::button::TouchButtonLocationHelper::TouchButtonLocationHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RTSScrollSelectionFormFactory {
}


impl RTSScrollSelectionFormFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(mut formLabel: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> {
        let mut basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>> = BasicColorFactory::getInstance();
        let mut touchButtonLocationHelper: Option<Rc<RefCell<TouchButtonLocationHelper>>> = TouchButtonLocationHelper::new();
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        let mut menuX: i32 = jcall!(CommonButtons::getInstance()).STANDARD_BUTTON_SIZE + jcall!(touchButtonLocationHelper).getColumnsRemainderHalf();
        let mut width: i32 = jcall!(displayInfo).getLastWidth() - (menuX * 2);
        let mut menuRectangle: Option<Rc<RefCell<Rectangle>>> = Rectangle::new(jcall!(PointFactory::getInstance()).createXY(menuX.clone(), 16), width.clone(), jcall!(CommonButtons::getInstance()).STANDARD_BUTTON_SIZE.clone());
        return MultipleScrollSelectionHorizontalForm::createForm(formLabel.clone(), items.clone(), menuRectangle.clone(), jcall!(FormTypeFactory::getInstance()).HORIZONTAL_FORM.clone(), 0, jcall!(basicColorFactory).BLACK.clone(), jcall!(basicColorFactory).WHITE.clone());
    }
}

