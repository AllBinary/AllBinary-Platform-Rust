// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.logic.visual.transform.info.objectConfig
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::io::path::AbPath::AbPath;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use super::TransformInfoObjectConfigAndManipulatorFactoryInterface::TransformInfoObjectConfigAndManipulatorFactoryInterface;
use super::TransformInfoObjectConfigInterface::TransformInfoObjectConfigInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoObjectConfigAndManipulatorFactoryBase {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

// impl TransformInfoObjectConfigAndManipulatorFactoryInterface for TransformInfoObjectConfigAndManipulatorFactoryBase with the methods of the trait

impl TransformInfoObjectConfigAndManipulatorFactoryBase {

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonStrings: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance3(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut objectConfigFileAbPath: Option<Rc<RefCell<dyn crate::org::allbinary::logic::io::path::AbPath::AbPathDyn>>>) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getInstance3(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait TransformInfoObjectConfigAndManipulatorFactoryBaseDyn {
    fn as_TransformInfoObjectConfigAndManipulatorFactoryBase(&self) -> &TransformInfoObjectConfigAndManipulatorFactoryBase;
    fn as_TransformInfoObjectConfigAndManipulatorFactoryBase_mut(&mut self) -> &mut TransformInfoObjectConfigAndManipulatorFactoryBase;
    fn getInstance(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<dyn TransformInfoObjectConfigInterface>>> { TransformInfoObjectConfigAndManipulatorFactoryBase::getInstance(self.as_TransformInfoObjectConfigAndManipulatorFactoryBase_mut(), abeClientInformation, transformInfoInterface) }
}
impl TransformInfoObjectConfigAndManipulatorFactoryBaseDyn for TransformInfoObjectConfigAndManipulatorFactoryBase {
    fn as_TransformInfoObjectConfigAndManipulatorFactoryBase(&self) -> &TransformInfoObjectConfigAndManipulatorFactoryBase { self }
    fn as_TransformInfoObjectConfigAndManipulatorFactoryBase_mut(&mut self) -> &mut TransformInfoObjectConfigAndManipulatorFactoryBase { self }
}


