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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::direction::DirectionFactory::DirectionFactory;
use super::LayerPlacementType::LayerPlacementType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LayerPlacementTypeFactory {
    pub MAP: Option<Rc<RefCell<LayerPlacementType>>>,
    pub LEFT: Option<Rc<RefCell<LayerPlacementType>>>,
    pub DOWN: Option<Rc<RefCell<LayerPlacementType>>>,
    pub UP: Option<Rc<RefCell<LayerPlacementType>>>,
    pub RIGHT: Option<Rc<RefCell<LayerPlacementType>>>,
}

thread_local! { pub static LayerPlacementTypeFactory_instance: Option<Rc<RefCell<LayerPlacementTypeFactory>>> = LayerPlacementTypeFactory::new(); }

impl LayerPlacementTypeFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            MAP: None,
            LEFT: None,
            DOWN: None,
            UP: None,
            RIGHT: None,
        };
        this.MAP = LayerPlacementType::new(i32::MAX.clone());
        this.LEFT = LayerPlacementType::new(jcall!(jcall!(DirectionFactory::getInstance()).LEFT).getValue());
        this.DOWN = LayerPlacementType::new(jcall!(jcall!(DirectionFactory::getInstance()).DOWN).getValue());
        this.UP = LayerPlacementType::new(jcall!(jcall!(DirectionFactory::getInstance()).UP).getValue());
        this.RIGHT = LayerPlacementType::new(jcall!(jcall!(DirectionFactory::getInstance()).RIGHT).getValue());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<LayerPlacementTypeFactory>>> {
        return LayerPlacementTypeFactory_instance.with(Clone::clone).clone();
    }
}

