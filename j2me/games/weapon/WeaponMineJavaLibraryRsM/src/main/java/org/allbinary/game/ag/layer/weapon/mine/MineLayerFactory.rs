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
// package org.allbinary.game.ag.layer.weapon.mine
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerFactoryInterface::AllBinaryLayerFactoryInterface;
use crate::org::allbinary::view::ViewPositionFactoryInterface::ViewPositionFactoryInterface;
use super::MineLayer::MineLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MineLayerFactory {
    pub viewPositionFactoryInterface: Option<Rc<RefCell<dyn ViewPositionFactoryInterface>>>,
}

impl AllBinaryLayerFactoryInterface for MineLayerFactory {
    fn getInstance(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> { Self::getInstance(self) }
}

impl MineLayerFactory {
    

    pub fn new_struct(mut viewPositionFactoryInterface: Option<Rc<RefCell<dyn ViewPositionFactoryInterface>>>) -> Self {
        let mut this = Self {
            viewPositionFactoryInterface: None,
        };
        this.viewPositionFactoryInterface = viewPositionFactoryInterface.clone();
        this
    }

    pub fn new(mut viewPositionFactoryInterface: Option<Rc<RefCell<dyn ViewPositionFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(viewPositionFactoryInterface))))
    }
    

    // throws Exception
    pub fn getInstance(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> {
        return MineLayer::new(jcall!(self.viewPositionFactoryInterface).getInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>);
    }
}

