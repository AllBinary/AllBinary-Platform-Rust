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
use crate::org::allbinary::game::input::RTSLayerFormFactory::RTSLayerFormFactory;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm;
use crate::org::allbinary::graphics::form::item::ItemArraySingletonFactoryInterface::ItemArraySingletonFactoryInterface;
use super::RTSLayer::RTSLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdvancedRTSLayerFormFactory {
    pub base: RTSLayerFormFactory,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>>,
}


impl AdvancedRTSLayerFormFactory {
    

    pub fn new_struct(mut label: String, mut itemArrayFactoryInterface: Option<Rc<RefCell<dyn ItemArraySingletonFactoryInterface>>>) -> Self {
        let mut this = Self {
            base: RTSLayerFormFactory::new_struct(),
            logUtil: None,
            scrollSelectionForm: None,
        };
        this.logUtil = LogUtil::getInstance();
        let mut scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> = ScrollSelectionForm::getNullScrollSelectionForm();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>> = jcall!(itemArrayFactoryInterface).getItems();
            scrollSelectionForm = RTSScrollSelectionFormFactory::RTSScrollSelectionFormFactory::getInstance(label.clone(), items.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            }
        }
        this.scrollSelectionForm = scrollSelectionForm.clone();
        this
    }

    pub fn new(mut label: String, mut itemArrayFactoryInterface: Option<Rc<RefCell<dyn ItemArraySingletonFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(label, itemArrayFactoryInterface))))
    }
    

    pub fn getInstance(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> {
        return self.scrollSelectionForm.clone();
    }
}
impl std::ops::Deref for AdvancedRTSLayerFormFactory {
    type Target = RTSLayerFormFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AdvancedRTSLayerFormFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::RTSLayerFormFactory::RTSLayerFormFactoryDyn for AdvancedRTSLayerFormFactory {
    fn as_RTSLayerFormFactory(&self) -> &crate::org::allbinary::game::input::RTSLayerFormFactory::RTSLayerFormFactory { &self.base }
    fn as_RTSLayerFormFactory_mut(&mut self) -> &mut crate::org::allbinary::game::input::RTSLayerFormFactory::RTSLayerFormFactory { &mut self.base }
}
impl crate::org::allbinary::game::input::RTSLayerFormFactoryInterface::RTSLayerFormFactoryInterface for AdvancedRTSLayerFormFactory {
    fn getInstance(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> { Self::getInstance(self, rtsLayer) }
}


