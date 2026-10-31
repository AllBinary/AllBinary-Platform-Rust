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
// package org.allbinary.graphics.form
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::TouchFeatureFactory::TouchFeatureFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use super::CommandCurrentSelectionForm::CommandCurrentSelectionForm;
use super::ScrollSelectionForm::ScrollSelectionForm;
use super::FormType::FormType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CommandCurrentSelectionFormFactory {
}


impl CommandCurrentSelectionFormFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut formType: Option<Rc<RefCell<FormType>>>, mut border: i32, mut moveForSmallScreen: bool, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> {
        if jcall!(Features::getInstance()).isFeature(jcall!(TouchFeatureFactory::getInstance()).TOUCH_ENABLED.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            let mut scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> = CommandCurrentSelectionForm::new(title.clone(), items.clone(), border.clone(), moveForSmallScreen.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>);
            jcall!(scrollSelectionForm).init(rectangle.clone(), formType.clone());
            return scrollSelectionForm.clone();
        } else {
            return ScrollSelectionFormNoneFactory::ScrollSelectionFormNoneFactory::getInstance();
        }
    }
}

